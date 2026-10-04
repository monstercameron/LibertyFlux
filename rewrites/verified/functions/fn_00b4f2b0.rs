// original: 0x00B4F2B0 task_flag_is_singleton

/// Report whether a task-flag word holds exactly one recognised singleton flag.
///
/// `flags` is compared against the six singleton values 1, 2, 4, 0x10, 0x20
/// and 0x40 in turn; any other value, including 0 and combinations such as 3,
/// reports false. The return keeps the caller's upper 24 bits (the original
/// loads the word into eax and then only sets the low byte), so bit 0 alone
/// carries the answer.
///
/// Original: 0x00B4F2B0 (cdecl, one stack word).
export!(cdecl, rw_00b4f2b0(flags: u32) -> u32 {
    {
        const SINGLETONS: [u32; 6] = [1, 2, 4, 0x10, 0x20, 0x40];
        let hit = SINGLETONS.contains(&flags) as u32;
        (flags & 0xFFFF_FF00) | hit
    }
});
