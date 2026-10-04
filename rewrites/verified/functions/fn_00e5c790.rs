// original: 0x00e5c790 timing_tick_then_notify_e6e440
/// Runs the slot tick step, then forwards a handler to the dispatcher.
///
/// Passes the incoming register argument to the tick step (thiscall/1: the
/// original pushes the register and the callee pops one word, so the stack
/// stays balanced), discards its answer, then calls the dispatcher (cdecl/1)
/// with the handler slot address and returns the dispatcher's answer.
export!(thiscall, rw_00e5c790(ecx_in: u32) -> u32 {
    unsafe {
        const HANDLER: u32 = 0x00E6E440;
        callee_thiscall!(1, u32, ecx_in, ecx_in);
        callee_cdecl!(2, u32, relocated(HANDLER))
    }
});

