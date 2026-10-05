// original: 0x00e69890 veh_objcall_dispatch_01 (proposed)

/// Runs a method on a vehicle global object, then notifies the dispatcher.
///
/// Calls the object method (thiscall, no stack words) with `OBJ` in ECX,
/// then the dispatcher (a one-word cdecl callee) with `TARGET`. Both
/// results are discarded. Takes no arguments, returns nothing (cdecl/0).
///
/// Original: 0x00E69890 (cdecl, no arguments, two calls).
lf_checker_rt::export!(cdecl, rw_00e69890() -> () {
    unsafe {
        /// Object the method runs on (file VA).
        const OBJ: u32 = 0x0166D9C0;
        /// Forwarded address (file VA).
        const TARGET: u32 = 0x00E72710;
        /// Object-method callee id.
        const METHOD: u32 = 1;
        /// Dispatcher callee id.
        const DISPATCH: u32 = 2;
        let _: u32 = lf_checker_rt::callee_thiscall!(METHOD, u32, lf_checker_rt::relocated(OBJ));
        let _: u32 = lf_checker_rt::callee_cdecl!(DISPATCH, u32, lf_checker_rt::relocated(TARGET));
    }
});
