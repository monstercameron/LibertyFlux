// original: 0x00E47DB0 UIBenchMark::vf0

/// Return the CRC result for this class's static type-name string. The vtable method does not inspect its object; it passes the relocated read-only string address to the CRC helper and returns that helper's 32-bit result. The method uses thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00e47db0(this: u32) -> u32 {
    let _ = this;
    const TYPE_NAME_TEXT: u32 = 0x00F168D8;
    lf_checker_rt::callee_cdecl!(
        1, u32, lf_checker_rt::relocated(TYPE_NAME_TEXT)
    )
});
