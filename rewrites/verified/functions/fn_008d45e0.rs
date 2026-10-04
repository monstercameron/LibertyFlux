// original: 0x008D45E0 emit_quad_xyz_fixed_uv
/// Quad emitter, 3-component variant.
///
/// Opens a 5-by-4 primitive, then emits four vertices. Vertex `i` carries a
/// three-float position from `b[i]`, the key dword `*k`, two zero words, the
/// constant float -1.0, and a fixed UV corner — (1,1), (0,1), (0,0), (1,0).
/// Control then passes to the end primitive with the same five arguments,
/// whose result is returned.
lf_checker_rt::export!(cdecl, rb92_fn4(b0: u32, b1: u32, b2: u32, b3: u32, k: u32) -> u32 {
    // Intercepted callees (ids match the contract's `callees` table).
    const CAL_BEGIN: u32 = 1; // begin primitive (cdecl/2)
    const CAL_VERT: u32 = 2; // emit one vertex (cdecl/9)
    const CAL_END: u32 = 3; // end primitive, tail-called (cdecl/5)

    const NEG_ONE_BITS: u32 = 0xBF80_0000; // -1.0f, fixed vertex field
    const ZERO_BITS: u32 = 0x0000_0000; // 0.0f
    const ONE_BITS: u32 = 0x3F80_0000; // 1.0f
    // Fixed UV corners, one pair per vertex, in emission order.
    const CORNERS: [(u32, u32); 4] = [
        (ONE_BITS, ONE_BITS),
        (ZERO_BITS, ONE_BITS),
        (ZERO_BITS, ZERO_BITS),
        (ONE_BITS, ZERO_BITS),
    ];

    #[inline(always)]
    fn triple(ptr: u32) -> (u32, u32, u32) {
        // Three consecutive float words, moved as bits.
        unsafe {
            let p = ptr as *const u32;
            (*p, *p.add(1), *p.add(2))
        }
    }

    unsafe {
        lf_checker_rt::callee_cdecl!(CAL_BEGIN, u32, 5u32, 4u32);
        let key = *(k as *const u32);
        let bs = [b0, b1, b2, b3];
        let mut i = 0;
        while i < 4 {
            let (bx, by, bz) = triple(bs[i]);
            let (u, v) = CORNERS[i];
            lf_checker_rt::callee_cdecl!(
                CAL_VERT, u32, bx, by, bz, 0u32, 0u32, NEG_ONE_BITS, key, u, v
            );
            i += 1;
        }
        lf_checker_rt::callee_cdecl!(CAL_END, u32, b0, b1, b2, b3, k)
    }
});
