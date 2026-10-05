// original: 0x00b1e810 dispatch_stream_event (proposed)

/// Dispatches a stream event id to the registered listeners.
///
/// Cdecl of one stack word: the event id. When the state flag word has
/// bit 0x400 set the id first goes to the primary listener; it always
/// goes to the recorder next. When bit 0x10 is clear that is all.
/// Otherwise, when the active byte is clear, or the secondary object is
/// missing or idle, or its idle byte is set, the id goes to the fallback
/// listener; only when the secondary is present and busy with its idle
/// byte clear does the id go to the primary listener a second time
/// instead. All listeners are cdecl of one word. Returns the last
/// listener's result, or the state pointer when dispatch stopped early.
lf_checker_rt::export!(cdecl, rw_00b1e810(event: u32) -> u32 {
    unsafe {
        const STATE_PTR: u32 = 0x012fb1b8;
        const SECONDARY_PTR: u32 = 0x016dd67c;
        const IDLE_FLAG: u32 = 0x012bd0f2;
        const STATE_FLAGS: u32 = 0x8e8;
        const PRIMARY_BIT: u32 = 0x400;
        const EXTRA_BIT: u32 = 0x10;
        const ACTIVE_OFF: u32 = 0x19;
        const BUSY_OFF: u32 = 0x22;
        let state = (lf_checker_rt::global::<u32>(STATE_PTR) as *const u32).read_unaligned();
        if ((state + STATE_FLAGS) as *const u32).read_unaligned() & PRIMARY_BIT != 0 {
            lf_checker_rt::callee_cdecl!(1, u32, event);
        }
        lf_checker_rt::callee_cdecl!(2, u32, event);
        let state2 = (lf_checker_rt::global::<u32>(STATE_PTR) as *const u32).read_unaligned();
        if ((state2 + STATE_FLAGS) as *const u8).read() & (EXTRA_BIT as u8) == 0 {
            return state2;
        }
        if ((state2 + ACTIVE_OFF) as *const u8).read() & 1 == 0 {
            return lf_checker_rt::callee_cdecl!(3, u32, event);
        }
        let secondary =
            (lf_checker_rt::global::<u32>(SECONDARY_PTR) as *const u32).read_unaligned();
        if secondary != 0 && ((secondary + BUSY_OFF) as *const u8).read() & 1 != 0 {
            return lf_checker_rt::callee_cdecl!(3, u32, event);
        }
        if (lf_checker_rt::global::<u8>(IDLE_FLAG) as *const u8).read() != 0 {
            return lf_checker_rt::callee_cdecl!(3, u32, event);
        }
        lf_checker_rt::callee_cdecl!(1, u32, event)
    }
});
