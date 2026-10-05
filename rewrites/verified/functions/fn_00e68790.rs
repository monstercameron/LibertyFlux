// original: 0x00e68790 veh_register_8790

/// Register one static vehicle callback with the shared registrar.
///
/// Pushes the callback address `CALLBACK` (a file VA reached through the
/// relocated image) and calls the registrar (cdecl/1, intercepted and
/// answered by the checker), returning the registrar's answer. Takes no
/// arguments and touches no other state.
///
/// Original: 0x00E68790 (cdecl/0, one direct call).
lf_checker_rt::export!(cdecl, rw_00e68790() -> u32 {
    unsafe {
        /// Callback registered by this instance (file VA).
        const CALLBACK: u32 = 0x00E723E0;
        lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(CALLBACK))
    }
});
