// original: 0x00E6FF00 timer_pending_deliver
/// Run the timer tick and deliver a pending value to the installed sink.
///
/// Ticks the timer context, then, when a value is pending, hands it to the
/// sink's deliver slot and clears the pending word. Returns the tick answer,
/// or the deliver answer when a delivery ran.
export!(cdecl, rw_00e6ff00() -> u32 {
    unsafe {
        const CTX: u32 = 0x01A006C8;
        const PENDING: u32 = 0x01A006D4;
        const SINK_SLOT: u32 = 0x018B8304;
        const DELIVER_OFF: u32 = 0x0C;
        let first = callee_thiscall!(1, u32, relocated(CTX));
        let pending = *global::<u32>(PENDING);
        if pending == 0 {
            return first;
        }
        let sink = *global::<u32>(SINK_SLOT);
        if sink != 0 {
            let vtable = *(sink as *const u32);
            let deliver: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(*((vtable + DELIVER_OFF) as *const u32) as usize);
            let second = deliver(sink, pending);
            *global::<u32>(PENDING) = 0;
            second
        } else {
            *global::<u32>(PENDING) = 0;
            first
        }
    }
});
