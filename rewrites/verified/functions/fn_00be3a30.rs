// original: 0x00be3a30 CTaskComplexUseSequence::vf19 (symbols)

/// Step the indexed sequence task, adopting a pending child when one waits.
///
/// `this + INDEX_OFF` (0x14) holds the sequence index, or -1 when there is
/// nothing to step (returns zero at once). Otherwise runs the shared step
/// helper over the task-table row `TABLE_BASE + index * ROW_STRIDE`
/// (0x6c-byte rows) with the incoming argument and this task's two inline
/// cursors (`+0x18`, `+0x20`), and keeps the helper's answer.
///
/// When a pending child id sits at `this + PENDING_OFF` (0x1c, not -1) and
/// the helper produced a child whose kind query (its third virtual) answers
/// `ADOPT_KIND` (0x111), the pending id is moved into the child (`+0x18`)
/// and the pending slot is cleared to -1. Returns the helper's answer in all
/// cases.
///
/// Original: 0x00be3a30 (thiscall, one stack word: the incoming argument).
lf_checker_rt::export!(thiscall, rw_00be3a30(this: u32, arg: u32) -> u32 {
    unsafe {
        const INDEX_OFF: u32 = 0x14;
        const PENDING_OFF: u32 = 0x1c;
        const CURSOR_B: u32 = 0x18;
        const CURSOR_A: u32 = 0x20;
        const TABLE_BASE: u32 = 0x0167f780;
        const ROW_STRIDE: u32 = 0x6c;
        const NONE: u32 = 0xffffffff;
        const KIND_SLOT: u32 = 0x0c;
        const ADOPT_KIND: u32 = 0x111;
        const CHILD_ID_OFF: u32 = 0x18;
        const CHILD_NEXT_OFF: u32 = 0x1c;
        const STEP: u32 = 1;
        // The kind query runs through the child's own vtable (stub id 2 in
        // the contract), exactly like the original: no ctable use here.
        let index = (this.wrapping_add(INDEX_OFF) as *const u32).read_unaligned();
        if index == NONE {
            return 0;
        }
        let row = lf_checker_rt::relocated(TABLE_BASE)
            .wrapping_add(index.wrapping_mul(ROW_STRIDE));
        let child = lf_checker_rt::callee_thiscall!(
            STEP,
            u32,
            row,
            arg,
            this.wrapping_add(CURSOR_B),
            this.wrapping_add(CURSOR_A)
        );
        let pending = (this.wrapping_add(PENDING_OFF) as *const u32).read_unaligned();
        if pending == NONE || child == 0 {
            return child;
        }
        let vtable = (child as *const u32).read_unaligned();
        let target = (vtable.wrapping_add(KIND_SLOT) as *const u32).read_unaligned();
        let kind_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        if kind_of(child) != ADOPT_KIND {
            return child;
        }
        (child.wrapping_add(CHILD_ID_OFF) as *mut u32).write_unaligned(pending);
        (child.wrapping_add(CHILD_NEXT_OFF) as *mut u32).write_unaligned(NONE);
        child
    }
});
