// original: 0x00b91170 NativeImpl_RENDER_LOADING_CLOCK

/// Draws the loading clock from packed floats, a color and three integers.
///
/// Packs nine float arguments and one integer word into a frame array and
/// passes five sliding windows into it (2, 4, 6, 8 and 10 words, each
/// running to the array end) together with an integer, a color packed from
/// the low bytes of four integer arguments, and two more integers, through
/// a single 9-argument `DRAW` call. The fifth stack word is an opaque
/// integer copied into the array mid-point. No value is returned.
///
/// All five buffer pointers are skipped call arguments with
/// snapshot-verified contents; the window lengths follow the layout (see
/// `narrowed`).
///
/// Original: 0x00B91170 (cdecl, seventeen stack words, no return value).
lf_checker_rt::export!(cdecl, rw_00b91170(
    f0: u32, f1: u32, f2: u32, f3: u32, f4: u32, a5: u32,
    f6: u32, f7: u32, f8: u32, f9: u32, i10: u32, c11: u32,
    c12: u32, c13: u32, c14: u32, i15: u32, i16: u32,
) -> u32 {
    const DRAW: u32 = 1;
    // Frame array in ascending address order.
    let mut cells = [f8, f9, f6, f7, f4, a5, f2, f3, f0, f1];
    let color = ((c14 & 0xFF) << 24) | ((c11 & 0xFF) << 16) | ((c12 & 0xFF) << 8) | (c13 & 0xFF);
    let _: u32 = lf_checker_rt::callee_cdecl!(
        DRAW, u32,
        unsafe { cells.as_mut_ptr().add(8) } as u32,
        unsafe { cells.as_mut_ptr().add(6) } as u32,
        unsafe { cells.as_mut_ptr().add(4) } as u32,
        unsafe { cells.as_mut_ptr().add(2) } as u32,
        cells.as_mut_ptr() as u32,
        i10, color, i15, i16
    );
    0
});
