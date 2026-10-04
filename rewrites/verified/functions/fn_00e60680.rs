// original: 0x00e60680 timing_init_register_09
/// Run one static initialiser then register its callback.
///
/// Calls the initialiser routine (stubbed, cdecl/0) and passes the code
/// pointer `0x00E6F990` to the registrar helper (stubbed, cdecl/1).
/// Returns the registrar's answer.
export!(cdecl, rw_00e60680() -> u32 {
    unsafe {
        let _: u32 = callee_cdecl!(1, u32,);
        callee_cdecl!(2, u32, relocated(0xE6F990))
    }
});
