// original: 0x00c7fcb0 CTaskComplexSeatedScenario::vf6

/// Scenario blend weight: 25.0 when a seat node of kind `0xdd` is chained.
///
/// Bit 1 of `this+0xc` set yields 25.0 at once; an inactive object (flag byte
/// at `this+0x20` set with word at `this+0x1c` clear) yields -1.0. Otherwise
/// the seat list headed at `[[arg+0x224]+0x2e0]` is walked through `next` at
/// `+0xc`: 25.0 when a node with kind `0xdd` at `+4` is found, -1.0 when the
/// list ends or the head is null. (The original also derives two identical
/// 3-bit fields from each node's word at `+8` and compares them; the compare
/// always finds them equal, so only the kind check decides.) Returned in ST0.
///
/// Original: thiscall, one stack word (the callee pops 4 bytes), float result in ST0.
lf_checker_rt::export!(thiscall, rw_00c7fcb0(this: u32, arg: u32) -> f32 {
    unsafe {
        const FLAG_OFF: u32 = 0x0c;
        const ACTIVE_OFF: u32 = 0x20;
        const STATE_OFF: u32 = 0x1c;
        const REC_OFF: u32 = 0x224;
        const HEAD_OFF: u32 = 0x2e0;
        const KIND_OFF: u32 = 4;
        const NEXT_OFF: u32 = 0x0c;
        const WANT_KIND: u32 = 0xdd;
        const ACTIVE_W: f32 = 25.0;
        const IDLE_W: f32 = -1.0;
        let flags = ((this + FLAG_OFF) as *const u32).read_unaligned();
        if (flags >> 1) & 1 != 0 {
            return ACTIVE_W;
        }
        let flag = ((this + ACTIVE_OFF) as *const u8).read();
        let state = ((this + STATE_OFF) as *const u32).read_unaligned();
        if flag != 0 && state == 0 {
            return IDLE_W;
        }
        let rec = ((arg + REC_OFF) as *const u32).read_unaligned();
        let mut node = ((rec + HEAD_OFF) as *const u32).read_unaligned();
        while node != 0 {
            if ((node + KIND_OFF) as *const u32).read_unaligned() == WANT_KIND {
                return ACTIVE_W;
            }
            node = ((node + NEXT_OFF) as *const u32).read_unaligned();
        }
        IDLE_W
    }
});
