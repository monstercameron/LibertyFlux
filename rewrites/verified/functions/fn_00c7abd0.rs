// original: 0x00c7abd0 CTaskComplexWaitForMyCarToStop::vf21

/// True when the waited-for car needs no further stopping logic.
/// Walks owner -> link (`+0xb30`) -> record (`+0xf50`) -> state
/// (`+0x224`); a null at any level ends the wait (returns 1). Then
/// scans the node list at `[state+0x2e0]` (next at `+0xc`): a node
/// tagged `0x2e2` at `+0x4` ends the wait too. Otherwise the stop
/// helper callee runs on the list head with (0x164, 0) and its full
/// EAX result is returned (only the low byte matters on the early
/// paths). ECX (`this`) is ignored. A duplicated shift-and-mask pair
/// in the original always compares equal and is omitted: it sets no
/// compared state.
/// Original: 0x00c7abd0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c7abd0(this: u32, owner: u32) -> u32 {
    unsafe {
        const OFF_LINK: u32 = 0xb30;
        const OFF_RECORD: u32 = 0xf50;
        const OFF_STATE: u32 = 0x224;
        const OFF_HEAD: u32 = 0x2e0;
        const OFF_TAG: u32 = 4;
        const OFF_NEXT: u32 = 0xc;
        const TAG_STOPPED: u32 = 0x2e2;
        const STOP_ARG: u32 = 0x164;
        const STOP: u32 = 1;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let link = rd32(owner.wrapping_add(OFF_LINK));
        if link == 0 {
            return 1;
        }
        let record = rd32(link.wrapping_add(OFF_RECORD));
        if record == 0 {
            return 1;
        }
        let state = rd32(record.wrapping_add(OFF_STATE));
        let mut node = rd32(state.wrapping_add(OFF_HEAD));
        while node != 0 {
            if rd32(node.wrapping_add(OFF_TAG)) == TAG_STOPPED {
                return 1;
            }
            node = rd32(node.wrapping_add(OFF_NEXT));
        }
        lf_checker_rt::callee_thiscall!(STOP, u32, state.wrapping_add(OFF_HEAD), STOP_ARG, 0)
    }
});
