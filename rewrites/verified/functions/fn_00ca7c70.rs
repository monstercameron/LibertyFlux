// original: 0x00CA7C70 CEventHandler::vf34

/// Clear the pending task when the event carries a recognised reset kind.
///
/// `event` (first stack argument) points to an event record with a kind word
/// at `+0x10` and a payload pointer at `+0x18`. When the payload is null the
/// handler does nothing. Otherwise, when the kind is `RESET_A` (0xC8) or
/// `RESET_B` (0x3A7), the handler's pending-task slot at `this+0x0C` is set
/// to null. The other two stack arguments are not read. No calls and no
/// meaningful return value.
///
/// Original: 0x00CA7C70 (thiscall, three stack words, only the first read).
lf_checker_rt::export!(thiscall, rw_00ca7c70(this: u32, event: u32, _a1: u32, _a2: u32) -> u32 {
    unsafe {
        const EV_KIND: u32 = 0x10;
        const EV_PAYLOAD: u32 = 0x18;
        const PENDING_TASK: u32 = 0x0C;
        const RESET_A: u32 = 0xC8;
        const RESET_B: u32 = 0x3A7;
        let payload = (event.wrapping_add(EV_PAYLOAD) as *const u32).read_unaligned();
        if payload == 0 {
            return 0;
        }
        let kind = (event.wrapping_add(EV_KIND) as *const u32).read_unaligned();
        if kind == RESET_A || kind == RESET_B {
            (this.wrapping_add(PENDING_TASK) as *mut u32).write_unaligned(0);
        }
        0
    }
});
