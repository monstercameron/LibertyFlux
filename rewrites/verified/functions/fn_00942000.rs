// original: 0x00942000 streaming_wordlist_append_a (proposed)

/// Notify the lane watcher, then append the argument's low word to the lane.
///
/// Looks up the watcher for the argument in the global table and calls it
/// (thiscall, no stack arguments). Then appends the low 16 bits of the
/// argument to the word list of lane `n` (read at `this + 0x2040`): the
/// list base at lane base `+0x204c`, the count word at `+0x2050`, each lane
/// record 8 bytes apart. Returns the new count in `eax`.
///
/// Original: 0x00942000 (thiscall, one stack argument; callee pops 4).
lf_checker_rt::export!(thiscall, rw_00942000(this: u32, arg: u32) -> u32 {
    unsafe {
        const WATCHERS: u32 = 0x01295CD8;
        const LANE: u32 = 0x2040;
        const LANE_STRIDE: u32 = 8;
        const LIST_BASE: u32 = 0x204C;
        const LIST_COUNT: u32 = 0x2050;
        const CALLEE: u32 = 1;
        let table = lf_checker_rt::relocated(WATCHERS);
        let watcher = ((table + arg.wrapping_mul(4)) as *const u32).read_unaligned();
        let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE, u32, watcher);
        let lane = ((this + LANE) as *const u32).read_unaligned();
        let rec = this + lane.wrapping_mul(LANE_STRIDE);
        let count_at = (rec + LIST_COUNT) as *mut u16;
        let count = count_at.read_unaligned() as u32;
        let base = ((rec + LIST_BASE) as *const u32).read_unaligned();
        ((base + count.wrapping_mul(2)) as *mut u16).write_unaligned(arg as u16);
        let new_count = count.wrapping_add(1);
        count_at.write_unaligned(new_count as u16);
        new_count
    }
});
