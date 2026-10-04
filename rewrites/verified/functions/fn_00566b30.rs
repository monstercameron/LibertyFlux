// original: 0x00566b30 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_90, player_schema::LeaderboardInfo, 10>::vf9

/// Sort code of a leaderboard column: 0, 1, 2, 3, or -1.
///
/// Arguments: the object (unused) and the column index.
///
/// Calls the leaderboard fetch helper (fastcall: id 0x125 in ecx,
/// out struct in edx, no stack arguments), reads the column array at
/// `+0x14`, looks the indexed column up through a second helper
/// (thiscall, value in ecx), and maps its kind: 1 yields 0, 2
/// yields 1, 3 yields 3, 4 yields -1, 5 yields 2.
///
/// A zero fetch status, an unknown column (-1), or a kind outside
/// 1..=5 all yield -1.
///
/// Edge cases: the index is trusted and a wild one faults exactly
/// like the original.
///
/// Original: thiscall, one stack word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00566b30(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x125;
        const COLUMNS_OFF: usize = 0x14 / 4;
        const FETCH_CALLEE: u32 = 1;
        const KIND_CALLEE: u32 = 2;
        const CODE_BY_KIND: [u32; 5] = [0, 1, 3, 0xffff_ffff, 2];
        let mut info = [0u32; 8];
        let answered: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if answered & 0xff == 0 {
            return 0xffff_ffff;
        }
        let at = info[COLUMNS_OFF].wrapping_add(index.wrapping_mul(4));
        let column = (at as *const u32).read_unaligned();
        let kind: u32 = lf_checker_rt::callee_thiscall!(KIND_CALLEE, u32, column);
        if kind == 0xffff_ffff {
            return 0xffff_ffff;
        }
        let slot = kind.wrapping_sub(1);
        if slot > 4 {
            return 0xffff_ffff;
        }
        CODE_BY_KIND[slot as usize]
    }
});
