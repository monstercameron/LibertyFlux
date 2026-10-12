// original: 0x005C0A10 game_mode_2_7_flag_check

/// Return whether the mode selector is 2 or 7 and the associated byte flag
/// is nonzero. The mode is read as an unsigned 32-bit word; the flag is read
/// as one byte. This zero-argument cdecl helper returns its boolean in AL.
lf_checker_rt::export!(cdecl, rw_005c0a10() -> u32 {
    const GAME_MODE_VA: u32 = 0x01037720;
    const MODE_2: u32 = 2;
    const MODE_7: u32 = 7;
    const MODE_2_7_FLAG_VA: u32 = 0x011F701E;

    let mode = unsafe { lf_checker_rt::global::<u32>(GAME_MODE_VA).read_unaligned() };
    let flagged_mode = mode == MODE_2 || mode == MODE_7;
    let enabled = unsafe { lf_checker_rt::global::<u8>(MODE_2_7_FLAG_VA).read_unaligned() } != 0;
    u32::from(flagged_mode && enabled)
});
