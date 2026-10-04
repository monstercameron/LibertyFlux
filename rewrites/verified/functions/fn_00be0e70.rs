// original: 0x00be0e70 task_maybe_reindex_then_classify (proposed)

/// Reindex this task when its slot is unset, then classify the slot.
///
/// Reads the slot word at `this + SLOT_OFF` (0x38). When it is -1 (unset),
/// runs the reindex callee on this task with argument 1 first. Then runs the
/// classify callee (cdecl) on the slot word and returns its answer. The
/// incoming stack word is unread; it exists only to be popped.
///
/// Original: 0x00be0e70 (thiscall, one stack word, unread).
lf_checker_rt::export!(thiscall, rw_00be0e70(this: u32, _arg: u32) -> u32 {
    unsafe {
        const SLOT_OFF: u32 = 0x38;
        const UNSET: u32 = 0xffffffff;
        const REINDEX_ARG: u32 = 1;
        const REINDEX: u32 = 1;
        const CLASSIFY: u32 = 2;
        let slot = (this.wrapping_add(SLOT_OFF) as *const u32).read_unaligned();
        if slot == UNSET {
            lf_checker_rt::callee_thiscall!(REINDEX, u32, this, REINDEX_ARG);
        }
        lf_checker_rt::callee_cdecl!(CLASSIFY, u32, slot)
    }
});
