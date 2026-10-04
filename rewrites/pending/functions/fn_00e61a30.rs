// original: 0x00e61a30 timing_lock_init_and_register_2
/// Initialise a lock, then register the handler.
///
/// Initialises the critical section at `0x01A05834` via InitializeCriticalSection (stubbed, stdcall/1), then passes the code pointer `0x00E704A0` to the registrar helper (stubbed, cdecl/1) and returns its answer.
export!(cdecl, rw_00e61a30() -> u32 {
    unsafe {
        let _: u32 = callee_stdcall!(0, u32, relocated(0x01a05834));
        callee_cdecl!(1, u32, relocated(0x00e704a0))
    }
});
