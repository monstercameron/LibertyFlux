// original: 0x00e60480 timing_init_register_07
/// Run one static initialiser then register its callback.
///
/// Calls the initialiser routine (stubbed, cdecl/0) and passes the code
/// pointer `0x00E6F8E0` to the registrar helper (stubbed, cdecl/1).
/// Returns the registrar's answer.
export!(cdecl, rw_00e60480() -> u32 {
    unsafe {
        let _: u32 = callee_cdecl!(1, u32,);
        callee_cdecl!(2, u32, relocated(0xE6F8E0))
    }
});
