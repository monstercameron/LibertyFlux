// original: 0x00567330 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_92, player_schema::LeaderboardInfo, 10>::vf7

/// Leaderboard entry at a given index, or -1 when the fetch fails.
///
/// Arguments: the object (unused) and the index.
///
/// Calls the leaderboard fetch helper (fastcall: id 0x127 in ecx,
/// out struct in edx, no stack arguments) and reads the entry array
/// pointer at `+0x10`. A zero status byte means failure.
///
/// On success returns the indexed word; the index is trusted, so a
/// wild one faults exactly like the original.
///
/// Edge cases: fetch failure returns -1 (0xffffffff).
///
/// Original: thiscall, one stack word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00567330(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x127;
        const ARRAY_OFF: usize = 0x10 / 4;
        const FETCH_CALLEE: u32 = 1;
        let mut info = [0u32; 8];
        let answered: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if answered & 0xff == 0 {
            return 0xffff_ffff;
        }
        let at = info[ARRAY_OFF].wrapping_add(index.wrapping_mul(4));
        (at as *const u32).read_unaligned()
    }
});
