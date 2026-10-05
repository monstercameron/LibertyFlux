// original: 0x00e68500 veh_register_8500

/// Register one static vehicle callback with the shared registrar.
///
/// Pushes the callback address `CALLBACK` (a file VA reached through the
/// relocated image) and calls the registrar (cdecl/1, intercepted and
/// answered by the checker), returning the registrar's answer. Takes no
/// arguments and touches no other state.
///
/// Original: 0x00E68500 (cdecl/0, one direct call).
lf_checker_rt::export!(cdecl, rw_00e68500() -> u32 {
    unsafe {
        /// Callback registered by this instance (file VA).
        const CALLBACK: u32 = 0x00E72330;
        lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(CALLBACK))
    }
});
