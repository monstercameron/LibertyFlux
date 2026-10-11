// original: 0x00c42050 CIntermezzoEventPlaceBomb::vf5
/// Return true for a missing event, a blocked root state, or a zero low byte
/// from the final query helper. The helper receives the root list at +0x2e0
/// and query arguments 7 and 0. The method's observable result is AL.
lf_checker_rt::export!(thiscall, rw_00c42050(this: u32) -> u32 {
    const EVENT_LINK: u32 = 0x18;
    const EVENT_READY: u32 = 0x14;
    const ROOT_READY_BYTE: u32 = 0x210;
    const ROOT_STATE: u32 = 0xa74;
    const ROOT_LINK: u32 = 0x224;
    const ROOT_MODE: u32 = 0xa70;
    const QUERY_LIST: u32 = 0x2e0;
    const QUERY_EVENT: u32 = 1;
    let linked = unsafe { ((this + EVENT_LINK) as *const u32).read_unaligned() };
    if linked == 0 {
        return 1;
    }
    let ready = unsafe { ((this + EVENT_READY) as *const u32).read_unaligned() };
    if ready == 0 {
        return 1;
    }
    unsafe {
        let linked_bytes = linked as *const u8;
        if linked_bytes.add(ROOT_READY_BYTE as usize).read_unaligned() != 0 {
            return 1;
        }
        let state = ((linked + ROOT_STATE) as *const u32).read_unaligned();
        if state == 1 || state == 2 {
            return 1;
        }
        if ((linked + ROOT_LINK) as *const u32).read_unaligned() == 0 {
            return 1;
        }
        if ((linked + ROOT_MODE) as *const u32).read_unaligned() == 1 {
            return 1;
        }
    }
    let query_base = unsafe { ((linked + ROOT_LINK) as *const u32).read_unaligned() };
    let found = lf_checker_rt::callee_thiscall!(QUERY_EVENT, u32, query_base + QUERY_LIST, 7, 0);
    u32::from(found as u8 == 0)
});
