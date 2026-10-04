// original: 0x008d3f50 matrix_forward_full
/// Forwards two 4-float rows and an integer to the shared 11-word matrix
/// worker: row A, a zero word, row B, the integer, a zero word. Returns the
/// worker's answer. Floats travel as bit patterns.
export!(cdecl, rw_008d3f50(a: u32, b: u32, extra: u32) -> u32 {
    unsafe {
        let x = a as *const u32;
        let y = b as *const u32;
        callee_cdecl!(
            1,
            u32,
            *x,
            *x.add(1),
            *x.add(2),
            *x.add(3),
            0,
            *y,
            *y.add(1),
            *y.add(2),
            *y.add(3),
            extra,
            0
        )
    }
});
