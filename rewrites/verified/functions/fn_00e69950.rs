// original: 0x00e69950 veh_objcall_4arg_01 (proposed)

/// Initialises a vehicle global object with two name pointers.
///
/// Calls the object initialiser (thiscall, four stack words) with `OBJ`
/// in ECX and the words (`A0`, `A1`, `A2`, `A3`); the callee cleans the
/// stack. The result is discarded. Takes no arguments, returns
/// nothing (cdecl/0).
///
/// Original: 0x00E69950 (cdecl, no arguments, one call).
lf_checker_rt::export!(cdecl, rw_00e69950() -> () {
    unsafe {
        /// Object the initialiser runs on (file VA).
        const OBJ: u32 = 0x0166DA78;
        /// Stack words (file VAs where they are addresses).
        const A0: u32 = 0x0;
        const A1: u32 = 0x00EB01A8;
        const A2: u32 = 0x0;
        const A3: u32 = 0x00EB0184;
        /// Initialiser callee id.
        const INIT: u32 = 1;
        let _: u32 = lf_checker_rt::callee_thiscall!(INIT, u32, lf_checker_rt::relocated(OBJ),
            A0, lf_checker_rt::relocated(A1), A2, lf_checker_rt::relocated(A3));
    }
});
