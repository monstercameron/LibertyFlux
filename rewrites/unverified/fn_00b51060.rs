// original: 0x00b51060 event_list_reset (proposed)

/// Reset an event-list header to the empty state.
///
/// If the payload pointer at `+0x0C` is non-null it is released through
/// callee 1 (thiscall on the payload, passed the address of the slot), then
/// the slot is cleared. Generation tag at `+0x04` and generation at `+0x08`
/// are cleared, state at `+0x10` becomes 3 (empty) and the head at `+0x00`
/// becomes -1. No meaningful return value.
///
/// Original: 0x00b51060 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00b51060(this: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0x00;
        const TAG: u32 = 0x04;
        const GENERATION: u32 = 0x08;
        const PAYLOAD: u32 = 0x0c;
        const STATE: u32 = 0x10;
        const STATE_EMPTY: u32 = 3;
        const RELEASE: u32 = 1;
        let payload = ((this + PAYLOAD) as *const u32).read_unaligned();
        if payload != 0 {
            lf_checker_rt::callee_thiscall!(RELEASE, u32, payload, this + PAYLOAD);
        }
        ((this + PAYLOAD) as *mut u32).write_unaligned(0);
        ((this + TAG) as *mut u32).write_unaligned(0);
        ((this + GENERATION) as *mut u32).write_unaligned(0);
        ((this + STATE) as *mut u32).write_unaligned(STATE_EMPTY);
        ((this + HEAD) as *mut u32).write_unaligned(0xffff_ffff);
        0
    }
});
