// original: 0x00c42280 CIntermezzoEventPlayerJackedVehicle::vf5
/// Check an associated root's timing/state gates, then call its event-query
/// helper. Return true if the root is absent, a gate blocks, or the helper's
/// low byte is zero; otherwise return whether the root's word at +0x7b8 is 6.
/// Only AL carries the result; the function leaves other EAX bytes incidental.
lf_checker_rt::export!(thiscall, rw_00c42280(this: u32) -> u32 {
    const ROOT_POINTER: u32 = 0x1c;
    const ROOT_READY_BYTE: u32 = 0x210;
    const ROOT_STATE: u32 = 0xa74;
    const ROOT_LINK: u32 = 0x224;
    const ROOT_MODE: u32 = 0xa70;
    const QUERY_LIST: u32 = 0x2e0;
    const FINAL_INDEX: u32 = 0x7b8;
    const QUERY_EVENT: u32 = 1;
    let root = unsafe { ((this + ROOT_POINTER) as *const u32).read_unaligned() };
    if root == 0 {
        return 1;
    }
    unsafe {
        let root_bytes = root as *const u8;
        if root_bytes.add(ROOT_READY_BYTE as usize).read_unaligned() != 0 {
            return 1;
        }
        let state = ((root + ROOT_STATE) as *const u32).read_unaligned();
        if state == 1 || state == 2 {
            return 1;
        }
        if ((root + ROOT_LINK) as *const u32).read_unaligned() == 0 {
            return 1;
        }
        if ((root + ROOT_MODE) as *const u32).read_unaligned() == 1 {
            return 1;
        }
    }
    let query_base = unsafe { ((root + ROOT_LINK) as *const u32).read_unaligned() };
    let found = lf_checker_rt::callee_thiscall!(QUERY_EVENT, u32, query_base + QUERY_LIST, 0x2e2, 0);
    if found as u8 == 0 {
        return 1;
    }
    let index = unsafe { ((root + FINAL_INDEX) as *const u32).read_unaligned() };
    u32::from(index == 6)
});
