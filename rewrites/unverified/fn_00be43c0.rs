// original: 0x00be43c0 CTaskComplexUseSequence::vf18 (symbols)

/// Advance the indexed sequence task and return the helper's answer.
///
/// `this + INDEX_OFF` (0x14) holds the sequence index, or -1 when there is
/// nothing to advance (returns zero at once). Otherwise runs the shared
/// advance helper over the task-table row `TABLE_BASE + index * ROW_STRIDE`
/// (0x6c-byte rows) with the incoming argument and this task's two inline
/// cursors (`+0x18`, `+0x20`), and returns whatever the helper answers. No
/// other reads, no writes.
///
/// Original: 0x00be43c0 (thiscall, one stack word: the incoming argument).
lf_checker_rt::export!(thiscall, rw_00be43c0(this: u32, arg: u32) -> u32 {
    unsafe {
        const INDEX_OFF: u32 = 0x14;
        const CURSOR_B: u32 = 0x18;
        const CURSOR_A: u32 = 0x20;
        const TABLE_BASE: u32 = 0x0167f780;
        const ROW_STRIDE: u32 = 0x6c;
        const NONE: u32 = 0xffffffff;
        const ADVANCE: u32 = 1;
        let index = (this.wrapping_add(INDEX_OFF) as *const u32).read_unaligned();
        if index == NONE {
            return 0;
        }
        let row = lf_checker_rt::relocated(TABLE_BASE)
            .wrapping_add(index.wrapping_mul(ROW_STRIDE));
        lf_checker_rt::callee_thiscall!(
            ADVANCE,
            u32,
            row,
            arg,
            this.wrapping_add(CURSOR_B),
            this.wrapping_add(CURSOR_A)
        )
    }
});
