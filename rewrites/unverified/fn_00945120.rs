// original: 0x00945120 streaming_load_request (proposed)

/// Run the streaming load request for the current lane, if one is pending.
///
/// Returns at once when the pending flag at `this + 0x1927` is clear. The
/// lane byte at `+0x1917` selects a 0xbd0-byte lane record; a set initialised
/// byte at record `+0xbca` is initialised first. Then the request id is
/// looked up in the fixed registry (thiscall, one stack argument: the id);
/// a null answer ends the call. Otherwise the answer plus 0x40, the id and
/// the two extra arguments are issued through the loader call (thiscall,
/// four stack arguments), the lane's table entry and marker bytes through
/// the emit call (thiscall, four stack arguments), and the record through
/// the completion call (thiscall, no stack arguments). Finally the previous
/// head id is shifted down and the record id becomes the head. Returns the
/// new head id, or leaves `eax` untouched when nothing was pending.
///
/// Original: 0x00945120 (thiscall, three stack arguments; callee pops 12).
lf_checker_rt::export!(thiscall, rw_00945120(this: u32, id: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const PENDING: u32 = 0x1927;
        const LANE: u32 = 0x1917;
        const MARK_B: u32 = 0x191A;
        const MARK_C: u32 = 0x191B;
        const HEAD: u32 = 0x190C;
        const PREV_HEAD: u32 = 0x1908;
        const TABLE: u32 = 0x17A0;
        const STRIDE: u32 = 0xBD0;
        const INIT_FLAG: u32 = 0xBCA;
        const REC_ID: u32 = 0xBC4;
        const REGISTRY: u32 = 0x0115D9A0;
        const INIT: u32 = 1;
        const LOOKUP: u32 = 2;
        const LOAD: u32 = 3;
        const EMIT: u32 = 4;
        const DONE: u32 = 5;
        if ((this + PENDING) as *const u8).read() == 0 {
            return 0;
        }
        let lane = ((this + LANE) as *const u8).read() as u32;
        let rec = this + lane.wrapping_mul(STRIDE);
        if ((rec + INIT_FLAG) as *const u8).read() != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(INIT, u32, rec);
        }
        let found: u32 = lf_checker_rt::callee_thiscall!(LOOKUP, u32, REGISTRY, id);
        if found == 0 {
            return 0;
        }
        let lane2 = ((this + LANE) as *const u8).read() as u32;
        let rec2 = this + lane2.wrapping_mul(STRIDE);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            LOAD,
            u32,
            rec2,
            a1,
            a2,
            found.wrapping_add(0x40),
            0u32
        );
        let lane3 = ((this + LANE) as *const u8).read() as u32;
        let entry = ((this + lane3.wrapping_mul(4) + TABLE) as *const u32).read_unaligned();
        let mb = ((this + MARK_B) as *const u8).read() as u32;
        let mc = ((this + MARK_C) as *const u8).read() as u32;
        let rec3 = this + lane3.wrapping_mul(STRIDE);
        let _: u32 =
            lf_checker_rt::callee_thiscall!(EMIT, u32, rec3, entry, 0u32, mb, mc);
        let lane4 = ((this + LANE) as *const u8).read() as u32;
        let rec4 = this + lane4.wrapping_mul(STRIDE);
        let _: u32 = lf_checker_rt::callee_thiscall!(DONE, u32, rec4);
        let head = ((this + HEAD) as *const u32).read_unaligned();
        ((this + PREV_HEAD) as *mut u32).write_unaligned(head);
        let lane5 = ((this + LANE) as *const u8).read() as u32;
        let new_head =
            ((this + lane5.wrapping_mul(STRIDE) + REC_ID) as *const u32).read_unaligned();
        ((this + HEAD) as *mut u32).write_unaligned(new_head);
        new_head
    }
});
