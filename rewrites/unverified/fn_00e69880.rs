// original: 0x00e69880 veh_dispatch_push_01 (proposed)

/// Forwards one address to the vehicle dispatcher.
///
/// Calls the dispatcher (a one-word cdecl callee) with `TARGET` and
/// discards the result. Takes no arguments, returns nothing (cdecl/0).
///
/// Original: 0x00E69880 (cdecl, no arguments, one call).
lf_checker_rt::export!(cdecl, rw_00e69880() -> () {
    unsafe {
        /// Forwarded address (file VA).
        const TARGET: u32 = 0x00E726F0;
        /// Dispatcher callee id.
        const DISPATCH: u32 = 1;
        let _: u32 = lf_checker_rt::callee_cdecl!(DISPATCH, u32, lf_checker_rt::relocated(TARGET));
    }
});
