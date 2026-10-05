// original: 0x00692A30 audio_scaled_mix_pass (proposed)

/// Scaled mix pass over one voice bank (original 0x00692A30).
///
/// `this` points at the bank (vtable-ish word at `+0`, table at `+0x0c`,
/// 16-bit object count `m` at `+0x10`). For each of the `m` objects: ask the
/// `helper` object's data-table slot at `+0x0c` for a scale (thiscall with
/// the object's key bytes and a frame slot preset to `1.0`; the callee
/// answers in AL and writes the scale back). A zero AL skips the object and
/// leaves the full 32-bit answer in EAX (only the low byte is tested).
///
/// The scale fills scratch: exactly `1.0` copies `n` words from `src`
/// (the original's `ucomiss`+`lahf` idiom takes the copy path only on
/// ordered-equal, so NaN scales multiply), anything else multiplies. Slot
/// `k` then gathers `keys[k]` -> `+0x0c` array -> entry `i`; a leaf whose
/// flag byte (`+4`) has bit `0x10` zeroes both its words. When `vcall` is
/// non-null it observes (key-hi byte, key-lo word, `n`, samples) through
/// its virtual slot at `+0x10`. Every sample above `0.0` (NaN is not above)
/// feeds its accumulator: mode 1 and mode 0 go through the two helper
/// routines, mode 2 accumulates `obj[0x10] += leaf[0x10] * sample` inline
/// unless the object's flag bit `0x10` is set, other modes do nothing.
///
/// The float operation order is the original's. The count `m` is compared
/// signed (`jle`) but comes from a 16-bit load, so it is `== 0` in practice.
/// `n` is an unsigned word count in `4..=16` by contract.
///
/// Scratch: the original reserves three `n`-word buffers with alloca (the
/// third is zeroed and never read here); the rewrite uses fixed stack
/// arrays. EAX on exit is the distance between the first two alloca
/// buffers (16-rounded `n*4`) whenever the mix loop ran, the last
/// data-table answer when the last object was skipped, else `m`.
///
/// Original: 0x00692A30 (thiscall, six stack words; the fifth is not read).
#[allow(clippy::too_many_arguments)]
fn audio_scaled_mix_pass(
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
        let _spare = [0u32; 16];
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
            // Data-table scale call; the stub writes the scale back.
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
            let mut k = 0usize;
            while k < count {
                let key = rd32(keys.wrapping_add((k as u32).wrapping_mul(4)));
                let arr = rd32(key.wrapping_add(TABLE_OFF));
                let leaf = rd32(arr.wrapping_add(i.wrapping_mul(4)));
                leaves[k] = leaf;
                if rd8(leaf.wrapping_add(FLAG_OFF)) & SKIP_FLAG != 0 {
                    leaves[k] = 0;
                    samples[k] = 0;
                }
                k += 1;
            }
            if vcall != 0 {
                let slot = rd32(rd32(vcall).wrapping_add(VTABLE_SLOT_OBSERVE));
                let observe: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                // Dead: the mix loop resets EAX first thing, like the original.
                let _ = observe(vcall, b5, w6, n, samples.as_ptr() as u32);
            }
            let mut k = 0usize;
            while k < count {
                let sample = f32::from_bits(samples[k]);
                if sample > 0.0 {
                    let entry = leaves[k];
                    let dl = rd8(obj.wrapping_add(FLAG_OFF));
                    match dl & 0x0f {
                        1 => {
                            let _ = lf_checker_rt::callee_thiscall!(HELPER_MODE1, u32, obj, entry);
                        }
                        0 => {
                            let _ = lf_checker_rt::callee_thiscall!(HELPER_MODE0, u32, obj, entry);
                        }
                        2 if dl & SKIP_FLAG == 0 => {
                            let prod = mul(rdf(entry.wrapping_add(ACC_OFF)), sample);
                            let cur = rdf(obj.wrapping_add(ACC_OFF));
                            wrf(obj.wrapping_add(ACC_OFF), add(prod, cur));
                        }
                        _ => {}
                    }
                }
                k += 1;
            }
            ret = delta;
            i += 1;
        }
        ret
    }
}

lf_checker_rt::export!(thiscall, rw_00692A30(this_ptr: u32, n: u32, keys: u32, vcall: u32, helper: u32, _u: u32, src: u32) -> u32 {
    audio_scaled_mix_pass(this_ptr, n, keys, vcall, helper, src)
});
