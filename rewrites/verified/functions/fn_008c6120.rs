// original: 0x008C6120 stream_fill_step8
/// Fill `count` streaming entries eight bytes at a time through a helper.
///
/// For `(count - 1) / 8 + 1` rounds (none when `count` is 0) asks the fill
/// callee to emit two dwords at `table + slot * 8`, adds `extra` into the
/// first of the two, advances the progress counter at `done` by 8 and the
/// slot at `cursor` by 1. `table` is the dword array at `[this]`, `token`
/// selects the fill mode. Returns 1 in the low byte.
/// Original: thiscall, five stack words.
lf_checker_rt::export!(thiscall, rw_008c6120(this: u32, token: u32,
                                              count: u32, done: u32,
                                              cursor: u32, extra: u32) -> u32 {
    unsafe {
        const FILL_CALLEE: u32 = 1;
        const STEP: u32 = 4;
        const SLOTS: u32 = 8;
        if count == 0 {
            return 1;
        }
        let table = (this as *const u32).read_unaligned();
        let mut rounds = count.wrapping_sub(1) >> 3;
        rounds = rounds.wrapping_add(1);
        while rounds != 0 {
            let slot = (cursor as *const u32).read_unaligned();
            let row = table.wrapping_add(slot.wrapping_mul(SLOTS));
            lf_checker_rt::callee_thiscall!(FILL_CALLEE, u32, token, row, STEP);
            (done as *mut u32).write_unaligned(
                (done as *const u32).read_unaligned().wrapping_add(STEP));
            (row as *mut u32).write_unaligned(
                (row as *const u32).read_unaligned().wrapping_add(extra));
            lf_checker_rt::callee_thiscall!(
                FILL_CALLEE, u32, token, row.wrapping_add(STEP), STEP);
            (done as *mut u32).write_unaligned(
                (done as *const u32).read_unaligned().wrapping_add(STEP));
            (cursor as *mut u32).write_unaligned(slot.wrapping_add(1));
            rounds -= 1;
        }
        1
    }
});
