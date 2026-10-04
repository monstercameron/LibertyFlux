// original: 0x008D41D0 emit_quad_fixed_uv
/// Quad emitter, fixed-UV variant.
///
/// Same shape as the full-UV emitter but the trailing UV pair is a per-vertex
/// constant corner — (0,1), (0,0), (1,1), (1,0) — instead of a second set of
/// pointer arguments, so this entry takes five arguments.
lf_checker_rt::export!(cdecl, rb92_fn2(b0: u32, b1: u32, b2: u32, b3: u32, k: u32) -> u32 {
    // Intercepted callees (ids match the contract's `callees` table).
    const CAL_BEGIN: u32 = 1; // begin primitive (cdecl/2)
    const CAL_VERT: u32 = 2; // emit one vertex (cdecl/9)
    const CAL_END: u32 = 3; // end primitive, tail-called (cdecl/5)

    const NEG_ONE_BITS: u32 = 0xBF80_0000; // -1.0f, fixed vertex field
    const ZERO_BITS: u32 = 0x0000_0000; // 0.0f
    const ONE_BITS: u32 = 0x3F80_0000; // 1.0f
    // Fixed UV corners, one pair per vertex, in emission order.
    const CORNERS: [(u32, u32); 4] = [
        (ZERO_BITS, ONE_BITS),
        (ZERO_BITS, ZERO_BITS),
        (ONE_BITS, ONE_BITS),
        (ONE_BITS, ZERO_BITS),
    ];

    #[inline(always)]
    fn pair(ptr: u32) -> (u32, u32) {
        // Two consecutive float words, moved as bits.
        unsafe {
            let p = ptr as *const u32;
            (*p, *p.add(1))
        }
    }

    unsafe {
        lf_checker_rt::callee_cdecl!(CAL_BEGIN, u32, 4u32, 4u32);
        let key = *(k as *const u32);
        let bs = [b0, b1, b2, b3];
        let mut i = 0;
        while i < 4 {
            let (bx, by) = pair(bs[i]);
            let (u, v) = CORNERS[i];
            lf_checker_rt::callee_cdecl!(
                CAL_VERT, u32, bx, by, 0u32, 0u32, 0u32, NEG_ONE_BITS, key, u, v
            );
            i += 1;
        }
        lf_checker_rt::callee_cdecl!(CAL_END, u32, b0, b1, b2, b3, k)
    }
});
