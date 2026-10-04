// original: 0x00DDE0E0 UITextField::vf0
/// Resolve the text field's stock style name through the style registry
/// and return the registry's answer.
lf_checker_rt::export!(cdecl, rw_dde0e0() -> u32 {
    lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(0x00EFD1F8))
});
