// original: 0x00b95950 NativeImpl_CLEAR_AREA_OF_CARS

/// Clears cars from an area, converting a low altitude first.
///
/// Same head as `rw_00b95520`: `z` against the -100.0 threshold, converted
/// through `CONV_CALLEE` when not below (NaN keeps `z`), packed with (`x`,
/// `y`) into a frame buffer. The buffer is cleared with (`w`, 0, 1, 0).
/// The stack check is off (the original writes its own `z` slot back); the
/// value is verified through the snapshot.
///
/// Original: 0x00B95950 (cdecl, four stack words, no return value).
lf_checker_rt::export!(cdecl, rw_00b95950(x: u32, y: u32, z: u32, w: u32) -> u32 {
    unsafe {
        const CONV_CALLEE: u32 = 1;
        const REG_CALLEE: u32 = 2;
        const THRESH: u32 = 0x00FE8DF8;
        const CONV_MODE: u32 = 4;
        let t = f32::from_bits(lf_checker_rt::global::<u32>(THRESH).read());
        let zf = f32::from_bits(z);
        let conv: u32 = if !(t >= zf) {
            z
        } else {
            let r: f32 = lf_checker_rt::callee_cdecl!(CONV_CALLEE, f32, x, y, CONV_MODE);
            r.to_bits()
        };
        let mut buf = [x, y, conv];
        let _: u32 = lf_checker_rt::callee_cdecl!(
            REG_CALLEE, u32, buf.as_mut_ptr() as u32, w, 0, 1, 0
        );
        0
    }
});
