// original: 0x00e5c7b0 timing_tick_then_notify_e6e450
/// Runs the slot tick step, then forwards a handler to the dispatcher.
///
/// Calls the tick step with the incoming register argument (thiscall/0),
/// discards its answer, then calls the dispatcher (cdecl/1) with the handler
/// slot address and returns the dispatcher's answer.
export!(thiscall, rw_00e5c7b0(ecx_in: u32) -> u32 {
    unsafe {
        const HANDLER: u32 = 0x00E6E450;
        callee_thiscall!(1, u32, ecx_in);
        callee_cdecl!(2, u32, relocated(HANDLER))
    }
});

