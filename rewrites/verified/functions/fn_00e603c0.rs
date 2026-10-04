// original: 0x00e603c0 timing_init_register_01
/// Run one static initialiser then register its callback.
///
/// Calls the initialiser routine (stubbed, cdecl/0) and passes the code
/// pointer `0x00E6F880` to the registrar helper (stubbed, cdecl/1).
/// Returns the registrar's answer.
export!(cdecl, rw_00e603c0() -> u32 {
    unsafe {
        let _: u32 = callee_cdecl!(1, u32,);
        callee_cdecl!(2, u32, relocated(0xE6F880))
    }
});
