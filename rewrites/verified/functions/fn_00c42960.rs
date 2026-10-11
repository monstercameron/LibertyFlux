// original: 0x00c42960 CIntermezzoEventFalling::vf4
/// Resolve the linked event list, then query it twice with state selectors 7
/// and 1. A null list returns the event pointer with AL cleared; a null query
/// result returns zero. If both queries succeed, tail-call the event method
/// on this and return its full EAX result.
lf_checker_rt::export!(thiscall, rw_00c42960(this: u32) -> u32 {
    const EVENT_POINTER: u32 = 4;
    const EVENT_LIST: u32 = 0x118;
    const FIND_FIRST: u32 = 1;
    const FIND_SECOND: u32 = 2;
    const EVENT_TAIL: u32 = 3;
    let event = unsafe { ((this + EVENT_POINTER) as *const u32).read_unaligned() };
    let list = unsafe { ((event + EVENT_LIST) as *const u32).read_unaligned() };
    if list == 0 {
        return event & 0xffff_ff00;
    }
    let first = lf_checker_rt::callee_thiscall!(FIND_FIRST, u32, list, 7, 0);
    if first == 0 {
        return 0;
    }
    let second = lf_checker_rt::callee_thiscall!(FIND_SECOND, u32, first, 1, 0);
    if second == 0 {
        return 0;
    }
    lf_checker_rt::callee_thiscall!(EVENT_TAIL, u32, this)
});
