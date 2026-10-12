// original: 0x00C0D1D0 UIMusicContainer::get_class_name_hash

/// Computes the class-name hash by passing the relocated `UIMusicContainer`
/// name string to the CRC helper and returning the helper's 32-bit result.
lf_checker_rt::export!(cdecl, rw_c0d1d0() -> u32 {
    const CRC32_OF_STRING: u32 = 1;
    const CLASS_NAME_VA: u32 = 0x00EC3920;
    let name = lf_checker_rt::relocated(CLASS_NAME_VA);
    lf_checker_rt::callee_cdecl!(CRC32_OF_STRING, u32, name)
});
