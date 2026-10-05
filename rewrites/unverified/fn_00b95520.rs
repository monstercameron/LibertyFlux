// original: 0x00b95520 NativeImpl_ADD_HOSPITAL_RESTART

/// Registers a hospital restart point, converting a low altitude first.
///
/// Compares the altitude `z` against the -100.0 threshold with `comiss`
/// semantics (unordered counts as below, so NaN keeps `z`): when below, the
/// position is converted through `CONV_CALLEE` with (`x`, `y`, `CONV_MODE`),
/// whose x87 result replaces `z`. The triple is packed into a frame buffer
/// and registered with (`w`, `extra`) through `REG_CALLEE`.
///
/// The original writes the converted value back into its own `z` argument
/// slot, which a Rust rewrite cannot address, so the contract disables the
/// stack check; the value is still verified through the buffer snapshot
/// (see `narrowed`).
///
/// Original: 0x00B95520 (cdecl, five stack words, no return value).
lf_checker_rt::export!(cdecl, rw_00b95520(x: u32, y: u32, z: u32, w: u32, extra: u32) -> u32 {
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
            REG_CALLEE, u32, buf.as_mut_ptr() as u32, w, extra
        );
        0
    }
});
