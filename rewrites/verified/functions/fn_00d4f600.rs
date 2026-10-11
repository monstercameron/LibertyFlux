// original: 0x00D4F600 CTaskComplexPickUpAndCarryObject::vf18
/// Query the next pickup state for this task. A null target pointer ends the
/// query immediately. Otherwise the related task at `+0x08` supplies a mode
/// through virtual slot `+0x0c`: mode `0x11d` delegates to one helper, mode
/// `0x139` returns zero, and mode `0x13a` delegates only when the target's
/// low flag nibble is at least two and its identifier at `+0x1bc` equals the
/// requested identifier. Other modes return zero. The compared fields are
/// unsigned bit patterns; the identifier comparison is equality.
lf_checker_rt::export!(thiscall, rw_00d4f600(this: u32, requested_id: u32) -> u32 {
    const RELATED_TASK: u32 = 0x08;
    const TARGET: u32 = 0x14;
    const MODE_SLOT: u32 = 0x0c;
    const TARGET_FLAGS: u32 = 0x1e2;
    const TARGET_ID: u32 = 0x1bc;
    const FIRST_MODE: u32 = 0x11d;
    const EMPTY_MODE: u32 = 0x139;
    const MATCHING_MODE: u32 = 0x13a;
    const MATCH_HELPER: u32 = 2;
    const FIRST_HELPER: u32 = 3;

    unsafe {
        let target = ((this + TARGET) as *const u32).read_unaligned();
        if target == 0 {
            return 0;
        }

        let related_task = ((this + RELATED_TASK) as *const u32).read_unaligned();
        let vtable = (related_task as *const u32).read_unaligned();
        let mode_address = ((vtable + MODE_SLOT) as *const u32).read_unaligned();
        let mode_query: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(mode_address as usize);
        let mode = mode_query(related_task);

        if mode == FIRST_MODE {
            return lf_checker_rt::callee_thiscall!(FIRST_HELPER, u32, this);
        }
        if mode == EMPTY_MODE {
            return 0;
        }
        if mode != MATCHING_MODE {
            return 0;
        }

        let target_mode = ((target + TARGET_FLAGS) as *const u8).read() & 0x0f;
        if target_mode < 2 {
            return 0;
        }
        let target_id = ((target + TARGET_ID) as *const u32).read_unaligned();
        if target_id != requested_id {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(MATCH_HELPER, u32, this)
    }
});
