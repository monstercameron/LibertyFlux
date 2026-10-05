// original: 0x008C60D0 stream_fill_step2
/// Fill `count` streaming entries two bytes at a time through a helper.
///
/// Zeroes the progress counter at `done`, then for `(count - 1) / 2 + 1`
/// rounds (none when `count` is 0) asks the fill callee to emit two bytes
/// at `cursor + base`, advancing `done` by 2 and `cursor` by 2 each round.
/// `token` selects the fill mode. Returns 1 in the low byte.
/// Original: thiscall, four stack words.
lf_checker_rt::export!(thiscall, rw_008c60d0(this: u32, token: u32,
                                              count: u32, done: u32,
                                              cursor: u32) -> u32 {
    unsafe {
        const FILL_CALLEE: u32 = 1;
        const STEP: u32 = 2;
        (done as *mut u32).write_unaligned(0);
        if count == 0 {
            return 1;
        }
        let mut rounds = count.wrapping_sub(1) >> 1;
        rounds = rounds.wrapping_add(1);
        let base = (this as *const u32).read_unaligned();
        while rounds != 0 {
            let at = (cursor as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(
                FILL_CALLEE, u32, token, base.wrapping_add(at), STEP);
            (done as *mut u32).write_unaligned(
                (done as *const u32).read_unaligned().wrapping_add(STEP));
            (cursor as *mut u32).write_unaligned(at.wrapping_add(STEP));
            rounds -= 1;
        }
        1
    }
});
