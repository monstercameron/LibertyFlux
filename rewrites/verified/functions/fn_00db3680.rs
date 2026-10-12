// original: 0x00DB3680 UIStackLayout::vf0

/// Return the helper's 32-bit class-name hash for the relocated name string.
/// The function passes the string pointer as one cdecl argument and returns
/// the helper's result unchanged.
lf_checker_rt::export!(cdecl, rw_00db3680() -> u32 {
    unsafe {
        const CLASS_NAME: u32 = 0x00ef2708;
        lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(CLASS_NAME))
    }
});
