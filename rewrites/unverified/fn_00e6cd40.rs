// original: 0x00e6cd40 disableimposters

/// Register the vehicle-imposter debug flag with the game's option registry.
///
/// Passes two constant strings (the flag name and its one-line description)
/// and two zero words to the registry callee, with the registry object in
/// `ecx` (`thiscall`, four stack words, callee cleans up), and returns the
/// callee's result unchanged. No arguments of its own (`cdecl`, plain `ret`).
/// The callee is intercepted and scripted by the checker; this wrapper only
/// forwards the constants, so the contract compares the call arguments, the
/// object pointer and the returned value on every trial.
lf_checker_rt::export!(cdecl, rw_00e6cd40() -> u32 {
    unsafe {
        /// Registry object (file VA of a zero-initialized global).
        const REGISTRY: u32 = 0x0178E7B0;
        /// Flag-name string (file VA).
        const FLAG_NAME: u32 = 0x00EE9ED4;
        /// Flag-description string (file VA).
        const FLAG_DESCR: u32 = 0x00EE9EA8;
        /// Intercepted registry callee id (see contract).
        const CALLEE: u32 = 1;
        lf_checker_rt::callee_thiscall!(CALLEE, u32,
            lf_checker_rt::relocated(REGISTRY),
            0u32,
            lf_checker_rt::relocated(FLAG_NAME),
            0u32,
            lf_checker_rt::relocated(FLAG_DESCR))
    }
});
