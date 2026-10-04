// original: 0x008d3fd0 matrix_forward_identity_b
/// Forwards one 4-float row and an integer to the shared 11-word matrix
/// worker with a constant second row (0, 0, 1, 1): row A, a zero word, the
/// constants, the integer, a zero word. Returns the worker's answer.
export!(cdecl, rw_008d3fd0(a: u32, extra: u32) -> u32 {
    unsafe {
        const ONE: u32 = 0x3F80_0000;
        let x = a as *const u32;
        callee_cdecl!(
            1,
            u32,
            *x,
            *x.add(1),
            *x.add(2),
            *x.add(3),
            0,
            0,
            0,
            ONE,
            ONE,
            extra,
            0
        )
    }
});
