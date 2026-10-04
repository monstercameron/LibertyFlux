// original: 0x00be41a0 CTaskComplexUseAttractor::vf18 (symbols)

/// Tune this task's attractor use, then resolve and check the effect chain.
///
/// Asks the attractor (`this + ATTRACTOR_OFF`, 0x14) its kind through its
/// first virtual and subtracts `KIND_BIAS` (2). A kind of `RANDOM_KIND`
/// (0x100, bias-adjusted 0) draws a random count: the low 16 bits of the draw
/// are scaled by `RAND_SCALE` (2^-15) and `RAND_SPAN` (-50.0f), truncated,
/// and mapped to `20 * (1 - value)` as the tune argument. A kind of
/// `SET_KIND` (0x101, adjusted 1) tunes with 0 when the argument's word at
/// `+ LIMIT_OFF` (0xb88) is at most `LIMIT_AT` (10, signed), and with -10
/// otherwise. Any other kind skips tuning. The tune callee always takes the
/// incoming argument as object; its answer is discarded. (The -10 case
/// shares the random path's call site in the original: one jump, not a
/// second call.)
///
/// Then the lookup callee (cdecl: it pops nothing itself) runs over
/// (argument, attractor) with the resolve callee consuming the two leftover
/// stack words as its own arguments; both answers are discarded and this
/// returns zero.
///
/// Float order is the original's; the scaled draw always lands in [-100, 0],
/// so the truncation is exact.
///
/// Original: 0x00be41a0 (thiscall, one stack word: the incoming argument).
lf_checker_rt::export!(thiscall, rw_00be41a0(this: u32, arg: u32) -> u32 {
    unsafe {
        const ATTRACTOR_OFF: u32 = 0x14;
        const KIND_SLOT: u32 = 0x00;
        const KIND_BIAS: u32 = 2;
        const RANDOM_KIND: u32 = 0;
        const SET_KIND: u32 = 1;
        const LIMIT_OFF: u32 = 0xb88;
        const LIMIT_AT: u32 = 10;
        const SET_LO: u32 = 0;
        const SET_HI: u32 = 0xfffffff6; // -10
        const RAND_SCALE_BITS: u32 = 0x38000000; // 2^-15
        const RAND_SPAN_BITS: u32 = 0xc2480000; // -50.0f
        const TUNE_ZERO: u32 = 2;
        const DRAW: u32 = 3;
        const TUNE: u32 = 4;
        const LOOKUP: u32 = 5;
        const RESOLVE: u32 = 6;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        // The kind query runs through the attractor's own vtable (stub id 1
        // in the contract), exactly like the original: no ctable use here.
        let attractor = (this.wrapping_add(ATTRACTOR_OFF) as *const u32).read_unaligned();
        let vtable = (attractor as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(KIND_SLOT) as *const u32).read_unaligned();
        let kind_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let adjusted = kind_of(attractor).wrapping_sub(KIND_BIAS);
        if adjusted == RANDOM_KIND {
            let draw = lf_checker_rt::callee_cdecl!(DRAW, u32,) & 0xffff;
            let scaled = mul(
                mul(draw as f32, f32::from_bits(RAND_SCALE_BITS)),
                f32::from_bits(RAND_SPAN_BITS),
            );
            let count = (1i32.wrapping_sub(scaled as i32) as u32).wrapping_mul(20);
            lf_checker_rt::callee_thiscall!(TUNE, u32, arg, count);
        } else if adjusted == SET_KIND {
            // Signed compare (the original branches on less-or-equal). The
            // high case shares the tune call site above, like the original.
            let limit = (arg.wrapping_add(LIMIT_OFF) as *const i32).read_unaligned();
            if limit > LIMIT_AT as i32 {
                lf_checker_rt::callee_thiscall!(TUNE, u32, arg, SET_HI);
            } else {
                lf_checker_rt::callee_thiscall!(TUNE_ZERO, u32, arg, SET_LO);
            }
        }
        let found = lf_checker_rt::callee_cdecl!(LOOKUP, u32, arg, attractor);
        lf_checker_rt::callee_thiscall!(RESOLVE, u32, found, arg, attractor);
        0
    }
});
