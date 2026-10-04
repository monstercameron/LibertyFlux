// original: 0x00cf9220 ladder_reachability_check (proposed)

/// Check whether a ped can reach a ladder and publish the climb parameters.
///
/// Cdecl with five words (`seat`, `mode`, `out_a`, `out_b`, `extra`).
/// Allocates a one-entry buffer with the alloc callee (0x100), stores
/// `seat` at entry 0 with a counter that always ends at 1, then either
/// uses `mode` directly (when non-negative) or scans the mode-table group
/// for `seat`'s class (count callee, then per-entry fetch callee plus a
/// virtual-slot +4 ask, stopping at answer 0x0e). No scan hit, an empty
/// group, or a null low byte from the seek callee (called with (`seat`,
/// mode-or-hit, three frame out-pointers, `extra`)) takes the early
/// return with al 0.
///
/// On success the two 4-float out-blocks go to `out_a`/`out_b`, two
/// running slots keep the third word of each, and the height-scale global
/// is set from the target (`[seat+0x1c]` when `[seat+0x20]` is null,
/// else the widened -`[t+0x10]` double converted back to float: the
/// callee is stubbed with a plain integer answer the original ignores
/// (it converts the pre-call double still sitting in its vector register,
/// which the stub preserves), so the rewrite converts the same double.
/// `[t+0x14]`'s widened pair is dead after the call on both sides.) A
/// virtual-slot +0x54 ask on `seat` then
/// supplies four floats for the next four globals. One loop pass runs the
/// step callee over entry 0, clamps the running slots (min against
/// `[out_b+8]`, max against `[out_a+8]`, minus/plus 1.0), and runs the
/// finish callee with (frame slot, the relocated 0xcf91b0 helper
/// address, buffer slot address, 0x10, 0xd). The buffer is always
/// released, and the return is the release callee's answer with its low
/// byte replaced by 0 (early paths) or 1 (success).
///
/// Frame slots the original pre-fills but never reads back ([F+0x64],
/// [F+0x68], [F+0x70], [F+0x74], [F+0x78]) and the in-loop `esi = -2` are
/// dead and omitted. The float callee keeps the pre-call double in its
/// vector register on both sides (the stub never touches it), so the
/// height global verifies the whole negate-and-widen data flow instead of
/// a scripted answer. Float order is the original's throughout.
lf_checker_rt::export!(cdecl, rw_00cf9220(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const CLASS_TABLE: u32 = 0x1295cd8;
        const CLASS_WORD: u32 = 0x2e;
        const SEAT_TARGET: u32 = 0x20;
        const SEAT_HEIGHT: u32 = 0x1c;
        const HEIGHT_GLOBAL: u32 = 0x171de28;
        const VEC_GLOBAL: u32 = 0x171de30;
        const NEG_MASK: u32 = 0x8000_0000;
        const ONE: f32 = 1.0;
        const FINISH_HELPER: u32 = 0x00cf91b0;
        const MALLOC: u32 = 1;
        const FREE: u32 = 2;
        const COUNT: u32 = 3;
        const FETCH: u32 = 4;
        const ASK_SLOT: u32 = 5;
        const SEEK: u32 = 6;
        const FPOP: u32 = 7;
        const PUBLISH: u32 = 8;
        const STEP: u32 = 9;
        const FINISH: u32 = 10;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let buf = lf_checker_rt::callee_cdecl!(MALLOC, u32, 0x100);
        let mut count: u16 = 0;
        let first = count;
        count = count.wrapping_add(1);
        ((buf + first as u32 * 4) as *mut u32).write_unaligned(a0);
        let mut mode = a1;
        if (mode as i32) <= -1 {
            let class = ((a0 + CLASS_WORD) as *const u16).read_unaligned() as i32;
            let table = lf_checker_rt::relocated(CLASS_TABLE);
            let group = rd32(table.wrapping_add((class << 2) as u32));
            let mut n = lf_checker_rt::callee_thiscall!(COUNT, u32, group);
            if (n as i32) <= 0 {
                let freed = lf_checker_rt::callee_cdecl!(FREE, u32, buf);
                return freed & 0xffff_ff00;
            }
            let mut i: u32 = 0;
            let hit = loop {
                let obj = lf_checker_rt::callee_thiscall!(FETCH, u32, group, i);
                let ask: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + 4) as usize);
                if (ask(obj) & 0xff) == 0x0e {
                    break Some(i);
                }
                i += 1;
                n = lf_checker_rt::callee_thiscall!(COUNT, u32, group);
                if !((i as i32) < (n as i32)) {
                    break None;
                }
            };
            match hit {
                Some(h) => mode = h,
                None => {
                    let freed = lf_checker_rt::callee_cdecl!(FREE, u32, buf);
                    return freed & 0xffff_ff00;
                }
            }
        }
        let mut b30 = [0u32; 4];
        let mut b40 = [0u32; 4];
        let mut b50 = [0u32; 1];
        let sought = lf_checker_rt::callee_cdecl!(
            SEEK, u32, a0, mode, b30.as_mut_ptr() as u32,
            b40.as_mut_ptr() as u32, b50.as_mut_ptr() as u32, a4
        );
        if (sought & 0xff) == 0 {
            let freed = lf_checker_rt::callee_cdecl!(FREE, u32, buf);
            return freed & 0xffff_ff00;
        }
        for k in 0..4u32 {
            ((a2 + k * 4) as *mut u32).write_unaligned(b40[k as usize]);
            ((a3 + k * 4) as *mut u32).write_unaligned(b30[k as usize]);
        }
        let mut s3 = b40[2];
        let mut s4 = b30[2];
        let target = rd32(a0 + SEAT_TARGET);
        let height: f32 = if target == 0 {
            rdf(a0 + SEAT_HEIGHT)
        } else {
            let f0 = rdf(target + 0x10);
            let neg = f32::from_bits(f0.to_bits() ^ NEG_MASK);
            let _ignored: u32 = lf_checker_rt::callee_thiscall!(FPOP, u32, a0);
            let wide = core::hint::black_box(neg) as f64;
            core::hint::black_box(wide) as f32
        };
        (lf_checker_rt::global::<u32>(HEIGHT_GLOBAL)).write_unaligned(height.to_bits());
        let mut b60 = [0u32; 1];
        let publish: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(a0) + 0x54) as usize);
        let vec = publish(a0, b60.as_mut_ptr() as u32);
        for k in 0..4u32 {
            (lf_checker_rt::global::<u32>(VEC_GLOBAL + k * 4)).write_unaligned(rd32(vec + k * 4));
        }
        let mut i: u32 = 0;
        if (i as u16) < count {
            loop {
                let entry = rd32(buf + i * 4);
                let mut bp = [0u32; 1];
                lf_checker_rt::callee_cdecl!(
                    STEP, u32, entry, mode, a2, a3, bp.as_mut_ptr() as u32, a4
                );
                let mut x1 = f32::from_bits(s4);
                let c0 = rdf(a3 + 8);
                if x1 > c0 {
                    x1 = c0;
                }
                let mut x0 = f32::from_bits(s3);
                let c1 = rdf(a2 + 8);
                if c1 > x0 {
                    x0 = c1;
                }
                x1 = sub(x1, ONE);
                x0 = add(x0, ONE);
                s4 = x1.to_bits();
                s3 = x0.to_bits();
                b60[0] = (count as u32) | 0x0040_0000;
                let bufslot = buf;
                lf_checker_rt::callee_cdecl!(
                    FINISH, u32, b60.as_mut_ptr() as u32,
                    lf_checker_rt::relocated(FINISH_HELPER),
                    (&bufslot as *const u32) as u32, 0x10, 0x0d
                );
                i += 1;
                if !(i < count as u32) {
                    break;
                }
            }
        }
        let freed = lf_checker_rt::callee_cdecl!(FREE, u32, buf);
        (freed & 0xffff_ff00) | 1
    }
});
