// original: 0x00d8d0f0 audio_slot_gather_filter (proposed)

/// Gather three dwords per audio slot into a scratch buffer, ask a helper to
/// score every slot, then keep or reject each slot by a float test and store
/// the survivors.
///
/// Arguments (cdecl, seven stack words): `a0`, `a1`, `a2` are opaque values
/// forwarded to the helper; `dst` points at the output records (stride
/// 0x60, first record header at `dst + 0x18 - 0x18`); `src` points at the
/// input slots (stride 0x20); `n` is the slot count (negative means none);
/// `fbits` is a float upper bound.
///
/// Gather: `g[i] = src[i * 0x20 .. +12]` (three dwords) for `i` in
/// `0..n`. The helper runs as
/// `helper(a0, a1, a2, g, n, f, r)`: `f` receives one float per slot and `r`
/// three dwords per slot (two integers, then the score float). Neither side
/// reads `f` or `r` before the helper writes them.
///
/// Filter, per slot `i` with `x3 = src[i*0x20+8]`, `x1 = src[i*0x20+0x18]`,
/// `x2 = r[i].score`: the slot passes only if `x3 > x1`, `x2 > 0`,
/// `fbits > 0` (all ordered comparisons, NaN fails) and
/// `t = (f[i] - x3) * max(x2, fbits) / (x1 - x3)` does not exceed the
/// constant at `CONST_ADDR` (unordered fails too). A passing `t` is stored
/// as computed, except an IEEE-equal `t` takes the constant's bits (so
/// `-0.0` against `+0.0` stores `+0.0`). The output record is flag 1 plus
/// copies of the
/// inputs, the score, `t`, and a global pointer's word at `+0x50`; a failing
/// slot stores flag 0 only. The float operation order is the original's.
///
/// Original: 0x00d8d0f0 (cdecl, seven stack words; no meaningful return).
lf_checker_rt::export!(cdecl, rw_00d8d0f0(a0: u32, a1: u32, a2: u32, dst: u32, src: u32, n: u32, fbits: u32) -> u32 {
    unsafe {
        const SRC_STRIDE: u32 = 0x20;
        const DST_STRIDE: u32 = 0x60;
        const DST_BASE_OFF: u32 = 0x18;
        const FLAG_OFF: u32 = 0x18; // below the record base
        const MAX_SLOTS: usize = 8; // contract keeps n in -2..=6
        const HELPER: u32 = 1;
        const COOKIE_CHECK: u32 = 2;
        const CONST_ADDR: u32 = 0x00fe88e8;
        const GLOBAL_PTR: u32 = 0x018b8968;
        const GLOBAL_WORD_OFF: u32 = 0x50;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        let count = (n as i32).max(0) as usize;
        // Gather buffer, zeroed: the contract snapshots the whole buffer
        // including slots past the count, which hold the defined stack fill.
        let mut g = [0u32; MAX_SLOTS * 4];
        let mut i = 0usize;
        while i < count {
            let s = src + i as u32 * SRC_STRIDE;
            g[i * 4] = rd32(s);
            g[i * 4 + 1] = rd32(s + 4);
            g[i * 4 + 2] = rd32(s + 8);
            i += 1;
        }
        let mut f = [0u32; MAX_SLOTS];
        let mut r = [0u32; MAX_SLOTS * 4];
        let _: u32 = lf_checker_rt::callee_cdecl!(
            HELPER, u32, a0, a1, a2, g.as_mut_ptr() as u32, n,
            f.as_mut_ptr() as u32, r.as_mut_ptr() as u32
        );

        let bound = f32::from_bits(fbits);
        let cap = f32::from_bits(rd32(lf_checker_rt::relocated(CONST_ADDR)));
        let gword = rd32(rd32(lf_checker_rt::relocated(GLOBAL_PTR)) + GLOBAL_WORD_OFF);
        let mut k = 0usize;
        while k < count {
            let s = src + k as u32 * SRC_STRIDE + 8;
            let d = dst + DST_BASE_OFF + k as u32 * DST_STRIDE;
            let x3 = f32::from_bits(rd32(s));
            let x1 = f32::from_bits(rd32(s + 16));
            let x2 = f32::from_bits(r[4 * k + 2]);
            let mut pass = x3 > x1 && x2 > 0.0 && bound > 0.0;
            let mut t = 0.0f32;
            if pass {
                // max(x2, bound): the jump takes the bound unless x2 is
                // strictly greater, so unordered also yields the bound.
                let m = if x2 > bound { x2 } else { bound };
                let num = mul(sub(f32::from_bits(f[k]), x3), m);
                t = div(num, sub(x1, x3));
                // Fail unless t <= cap (ordered). The below-branch jumps
                // past the copy, so a smaller t is kept as is; only an
                // IEEE-equal t (the fall-through) takes the constant's bits,
                // even -0.0 against +0.0.
                if !(cap >= t) {
                    pass = false;
                } else if !(cap > t) {
                    t = cap;
                }
            }
            if pass {
                wr32(d - FLAG_OFF, 1);
                wr32(d - 8, rd32(s - 8));
                wr32(d - 4, rd32(s - 4));
                wr32(d, f[k]);
                wr32(d + 8, r[4 * k]);
                wr32(d + 12, r[4 * k + 1]);
                wr32(d + 16, r[4 * k + 2]);
                wr32(d + 0x28, t.to_bits());
                wr32(d + 0x30, gword);
                ((d + 0x3a) as *mut u16).write_unaligned(0);
            } else {
                wr32(d - FLAG_OFF, 0);
            }
            k += 1;
        }

        let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE_CHECK, u32,);
        0
    }
});
