// original: 0x00be4b20 task_find_or_request_bus_stop (proposed)

/// Find the current bus-stop task, or ask the stop to make one.
///
/// `obj + LINK_OFF` (0x224) leads to a holder with two candidate slots,
/// `SLOT_A` (0x50) first and `SLOT_B` (0x54) as fallback; a null link or two
/// null slots return zero. The first non-null candidate is asked its kind
/// through its third virtual: when it answers `STOP_KIND` (0x2de) it is
/// returned as is. Otherwise the candidate is asked to produce the stop task
/// through its fifteenth virtual (`MAKE_SLOT`, 0x3c) with argument
/// `STOP_KIND`, and that answer is returned.
///
/// The original stages the `STOP_KIND` argument by overwriting its own
/// incoming stack slot and tail-jumps; the rewrite forwards the value with a
/// plain call. The calls and result match; the stack pointer and the staging
/// slot legitimately differ (see the contract).
///
/// Original: 0x00be4b20 (stdcall, one stack word: the object).
lf_checker_rt::export!(stdcall, rw_00be4b20(obj: u32) -> u32 {
    unsafe {
        const LINK_OFF: u32 = 0x224;
        const SLOT_A: u32 = 0x50;
        const SLOT_B: u32 = 0x54;
        const KIND_SLOT: u32 = 0x0c;
        const MAKE_SLOT: u32 = 0x3c;
        const STOP_KIND: u32 = 0x2de;
        // Both queries run through the candidate's own vtable (stub ids 1
        // and 2 in the contract), exactly like the original.
        let holder = (obj.wrapping_add(LINK_OFF) as *const u32).read_unaligned();
        let mut cand = (holder.wrapping_add(SLOT_A) as *const u32).read_unaligned();
        if cand == 0 {
            cand = (holder.wrapping_add(SLOT_B) as *const u32).read_unaligned();
            if cand == 0 {
                return 0;
            }
        }
        let vtable = (cand as *const u32).read_unaligned();
        let kind_at = (vtable.wrapping_add(KIND_SLOT) as *const u32).read_unaligned();
        let kind_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(kind_at as usize);
        if kind_of(cand) == STOP_KIND {
            return cand;
        }
        let make_at = (vtable.wrapping_add(MAKE_SLOT) as *const u32).read_unaligned();
        let make: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(make_at as usize);
        make(cand, STOP_KIND)
    }
});
