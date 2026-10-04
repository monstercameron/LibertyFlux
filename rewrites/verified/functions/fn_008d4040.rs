// original: 0x008D4040 emit_quad_uv_arrays
/// Quad emitter, full-UV variant.
///
/// Opens a 4-by-4 primitive, then emits four vertices. Vertex `i` pairs the
/// position from `b[i]` with the UV from `a[i]`; every vertex also carries
/// the key dword `*k`, three zero words and the constant float -1.0. Control
/// then passes to the end primitive with the same nine arguments, whose
/// result is returned.
///
/// All pointer arguments are read but never written; the floats are moved
/// bit-exactly (the original only ever issues `movss` on them).
lf_checker_rt::export!(cdecl, rb92_fn1(
    b0: u32, b1: u32, b2: u32, b3: u32,
    a0: u32, a1: u32, a2: u32, a3: u32,
    k: u32,
) -> u32 {
    // Intercepted callees (ids match the contract's `callees` table).
    const CAL_BEGIN: u32 = 1; // begin primitive (cdecl/2)
    const CAL_VERT: u32 = 2; // emit one vertex (cdecl/9)
    const CAL_END: u32 = 3; // end primitive, tail-called (cdecl/9)

    const NEG_ONE_BITS: u32 = 0xBF80_0000; // -1.0f, fixed vertex field

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
        let aa = [a0, a1, a2, a3];
        let mut i = 0;
        while i < 4 {
            let (bx, by) = pair(bs[i]);
            let (ax, ay) = pair(aa[i]);
            lf_checker_rt::callee_cdecl!(
                CAL_VERT, u32, bx, by, 0u32, 0u32, 0u32, NEG_ONE_BITS, key, ax, ay
            );
            i += 1;
        }
        lf_checker_rt::callee_cdecl!(CAL_END, u32, b0, b1, b2, b3, a0, a1, a2, a3, k)
    }
});
