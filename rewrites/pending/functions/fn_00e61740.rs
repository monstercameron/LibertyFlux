// original: 0x00e61740 timing_lock_init_and_register_1
/// Initialise a lock, then register the handler.
///
/// Initialises the critical section at `0x01A02268` via InitializeCriticalSection (stubbed, stdcall/1), then passes the code pointer `0x00E702C0` to the registrar helper (stubbed, cdecl/1) and returns its answer.
export!(cdecl, rw_00e61740() -> u32 {
    unsafe {
        let _: u32 = callee_stdcall!(0, u32, relocated(0x01a02268));
        callee_cdecl!(1, u32, relocated(0x00e702c0))
    }
});
