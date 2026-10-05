// original: 0x00b97690 NativeImpl_REGISTER_SAVE_HOUSE_2

/// Registers a save house from a position, a radius and two integers.
///
/// Packs the first three float arguments into a three-float frame buffer
/// (the `(an instruction of the original)` in the original only reserves the fourth outgoing slot,
/// which is then overwritten with the fourth float) and calls `REG_CALLEE`
/// with (buffer, `f3`, `i4`, `i5`). No value is returned.
///
/// The buffer pointer differs legitimately between frames, so the contract
/// skips that call argument and snapshot-verifies the three packed floats.
///
/// Original: 0x00B97690 (cdecl, six stack words, no return value).
lf_checker_rt::export!(cdecl, rw_00b97690(f0: u32, f1: u32, f2: u32, f3: u32, i4: u32, i5: u32) -> u32 {
    const REG_CALLEE: u32 = 1;
    let mut buf = [f0, f1, f2];
    let _: u32 = lf_checker_rt::callee_cdecl!(
        REG_CALLEE, u32, buf.as_mut_ptr() as u32, f3, i4, i5
    );
    0
});
