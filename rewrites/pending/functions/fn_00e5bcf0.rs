// original: 0x00E5BCF0 register_callback_e6e1c0
/// Registers one callback routine with the game's single-argument registrar.
///
/// Behaviour: passes the callback's address to the registrar and returns the
/// registrar's result unchanged. The pushed address is a fixed code address
/// (relocated by the loader), so it is derived through `relocated`.
lf_checker_rt::export!(cdecl, rs188_00e5bcf0() -> u32 {
    /// File VA of the callback handed to the registrar.
    const CALLBACK: u32 = 0x00E6E1C0;
    /// Registrar routine: cdecl/1, result returned as-is.
    const CAL_REGISTER: u32 = 1;
    lf_checker_rt::callee_cdecl!(CAL_REGISTER, u32, lf_checker_rt::relocated(CALLBACK))
});
