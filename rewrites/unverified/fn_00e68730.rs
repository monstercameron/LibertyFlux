// original: 0x00e68730 veh_register_8730

/// Register one static vehicle callback with the shared registrar.
///
/// Pushes the callback address `CALLBACK` (a file VA reached through the
/// relocated image) and calls the registrar (cdecl/1, intercepted and
/// answered by the checker), returning the registrar's answer. Takes no
/// arguments and touches no other state.
///
/// Original: 0x00E68730 (cdecl/0, one direct call).
lf_checker_rt::export!(cdecl, rw_00e68730() -> u32 {
    unsafe {
        /// Callback registered by this instance (file VA).
        const CALLBACK: u32 = 0x00E723B0;
        lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(CALLBACK))
    }
});
