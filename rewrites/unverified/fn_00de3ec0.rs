// original: 0x00DE3EC0 UIListLayout::vf0

/// Return this class's name hash through the shared string-hash helper.
/// The string pointer remains an image reference after relocation.
lf_checker_rt::export!(cdecl, rw_00DE3EC0() -> u32 {
    const CLASS_NAME_TEXT: u32 = 0x00EFDD00;
    let name = lf_checker_rt::relocated(CLASS_NAME_TEXT);
    lf_checker_rt::callee_cdecl!(1, u32, name)
});
