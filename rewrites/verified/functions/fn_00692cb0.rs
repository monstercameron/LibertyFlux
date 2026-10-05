// original: 0x00692CB0 audio_keyed_scaled_mix_pass (proposed)

/// Keyed scaled mix pass over one voice bank (original 0x00692CB0).
///
/// Same shape as `rw_00692A30` (data-table scale call with AL answer,
/// scale-or-copy fill, virtual observe call, mode-dispatched mix with the
/// same EAX rules), but the gather phase is a keyed search: slot 0 walks
/// one candidate table from a single persistent cursor until it finds the
/// entry whose 24-bit key (byte at `+5` shifted left 16, or-ed with the
/// word at `+6`) matches the object's, advancing past smaller keys and
/// giving up past larger ones with an unsigned comparison (`ja`; the keys
/// are 24-bit so signed order agrees bit for bit). Later slots reuse the
/// candidate just below the cursor from their own tables. A slot whose
/// search overruns its bound sits out; a search that never finds a live
/// leaf skips the observe call and the mix. The mix additionally requires
/// slot 0 to hold a leaf.
///
/// Cursor and EAX details: the cursor persists across the `m` objects (the
/// third scratch buffer is zeroed once, never per object), and the cursor
/// is compared against its bound SIGNED (`jge`/`jl`), matched here with
/// `i32` comparisons. EAX on a give-up is the object's low key word with
/// its low byte stamped to 1 (only the low word is live in EAX there); on
/// a bound overrun before any slot it is the bound; otherwise the last
/// loaded leaf word, the observe answer, or the 16-rounded `n*4` exactly
/// as in `rw_00692A30`.
///
/// Out of contract: `n < 4` (stale stack in the snapshot, as in
/// `rw_00692A30`; the original's scalar tail also reads a stale register
/// there on later objects).
///
/// Original: 0x00692CB0 (thiscall, six stack words; the fifth is not read).
#[allow(clippy::too_many_arguments)]
fn audio_keyed_scaled_mix_pass(
    this_ptr: u32,
    n: u32,
    keys: u32,
    vcall: u32,
    helper: u32,
    src: u32,
) -> u32 {
    unsafe {
        const TABLE_OFF: u32 = 0x0c;
        const COUNT_OFF: u32 = 0x10;
        const FLAG_OFF: u32 = 0x04;
        const KEY_HI_OFF: u32 = 0x05;
        const KEY_LO_OFF: u32 = 0x06;
        const ACC_OFF: u32 = 0x10;
        const SKIP_FLAG: u8 = 0x10;
        const VTABLE_SLOT_OBSERVE: u32 = 0x10;
        const DTABLE_SLOT_SCALE: u32 = 0x0c;
        const ONE: f32 = 1.0;
        const HELPER_MODE1: u32 = 2;
        const HELPER_MODE0: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn key24(obj: u32) -> u32 {
            unsafe {
                ((rd8(obj.wrapping_add(KEY_HI_OFF)) as u32) << 16)
                    | rd16(obj.wrapping_add(KEY_LO_OFF))
            }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let count = n as usize;
        let delta = n.wrapping_mul(4).wrapping_add(15) & !15;
        let mut samples = [0u32; 16];
        let mut leaves = [0u32; 16];
        let mut cursor0 = 0u32;
        let vt = rd32(this_ptr);
        let bank = rd32(vt.wrapping_add(TABLE_OFF));
        let m = rd16(vt.wrapping_add(COUNT_OFF));
        if m == 0 {
            return m;
        }
        let mut ret = m;
        let mut i = 0u32;
        while i < m {
            let obj = rd32(bank.wrapping_add(i.wrapping_mul(4)));
            let b5 = rd8(obj.wrapping_add(KEY_HI_OFF)) as u32;
            let w6 = rd16(obj.wrapping_add(KEY_LO_OFF));
            let mut scale = ONE;
            let dt_slot = rd32(rd32(helper).wrapping_add(DTABLE_SLOT_SCALE));
            let ask: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(dt_slot as usize);
            let ans = ask(helper, b5, w6, core::ptr::addr_of_mut!(scale) as u32);
            if ans as u8 == 0 {
                ret = ans;
                i += 1;
                continue;
            }
            let scale_now = scale;
            if scale_now == ONE {
                let src_words =
                    core::slice::from_raw_parts(src as *const u32, count);
                samples[..count].copy_from_slice(src_words);
            } else {
                let mut k = 0usize;
                while k < count {
                    let v = rdf(src.wrapping_add((k as u32).wrapping_mul(4)));
                    samples[k] = mul(v, scale_now).to_bits();
                    k += 1;
                }
            }
            leaves[..count].fill(0);
            let mut live = false;
            let mut inner_eax = 0u32;
            let mut s = 0usize;
            'gather: while s < count {
                let key = rd32(keys.wrapping_add((s as u32).wrapping_mul(4)));
                if s == 0 {
                    let bound = rd16(key.wrapping_add(COUNT_OFF));
                    if (cursor0 as i32) >= (bound as i32) {
                        inner_eax = bound;
                        break 'gather;
                    }
                    let cands = rd32(key.wrapping_add(TABLE_OFF));
                    let want = key24(obj);
                    loop {
                        let cand = rd32(
                            cands.wrapping_add(cursor0.wrapping_mul(4)),
                        );
                        let got = key24(cand);
                        if got == want {
                            leaves[0] = cand;
                            cursor0 = cursor0.wrapping_add(1);
                            break;
                        }
                        if got > want {
                            inner_eax = (w6 & !0xFF) | 1;
                            break 'gather;
                        }
                        cursor0 = cursor0.wrapping_add(1);
                        if (cursor0 as i32) >= (bound as i32) {
                            break;
                        }
                    }
                } else {
                    let arr = rd32(key.wrapping_add(TABLE_OFF));
                    leaves[s] = rd32(
                        arr.wrapping_add(
                            cursor0.wrapping_sub(1).wrapping_mul(4),
                        ),
                    );
                }
                let loaded = leaves[s];
                inner_eax = loaded;
                if loaded == 0 {
                    samples[s] = 0;
                } else if rd8(loaded.wrapping_add(FLAG_OFF)) & SKIP_FLAG != 0 {
                    leaves[s] = 0;
                    samples[s] = 0;
                } else {
                    live = true;
                }
                s += 1;
            }
            if live {
                if vcall != 0 {
                    let slot =
                        rd32(rd32(vcall).wrapping_add(VTABLE_SLOT_OBSERVE));
                    let observe: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
                        core::mem::transmute(slot as usize);
                    ret = observe(vcall, b5, w6, n, samples.as_ptr() as u32);
                } else {
                    ret = inner_eax;
                }
                if leaves[0] != 0 && n > 0 {
                    let mut k = 0usize;
                    while k < count {
                        let sample = f32::from_bits(samples[k]);
                        if sample > 0.0 {
                            let entry = leaves[k];
                            let dl = rd8(obj.wrapping_add(FLAG_OFF));
                            match dl & 0x0f {
                                1 => {
                                    let _ =
                                        lf_checker_rt::callee_thiscall!(HELPER_MODE1, u32, obj, entry);
                                }
                                0 => {
                                    let _ =
                                        lf_checker_rt::callee_thiscall!(HELPER_MODE0, u32, obj, entry);
                                }
                                2 if dl & SKIP_FLAG == 0 => {
                                    let prod =
                                        mul(rdf(entry.wrapping_add(ACC_OFF)), sample);
                                    let cur = rdf(obj.wrapping_add(ACC_OFF));
                                    wrf(obj.wrapping_add(ACC_OFF), add(prod, cur));
                                }
                                _ => {}
                            }
                        }
                        k += 1;
                    }
                    ret = delta;
                }
            } else {
                ret = inner_eax;
            }
            i += 1;
        }
        ret
    }
}

lf_checker_rt::export!(thiscall, rw_00692CB0(this_ptr: u32, n: u32, keys: u32, vcall: u32, helper: u32, _u: u32, src: u32) -> u32 {
    audio_keyed_scaled_mix_pass(this_ptr, n, keys, vcall, helper, src)
});
