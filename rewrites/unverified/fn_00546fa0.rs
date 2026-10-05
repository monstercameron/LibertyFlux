// original: 0x00546fa0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_1, player_schema::LeaderboardInfo, 10>::vf8

/// Storage width of one leaderboard column, from its type code.
///
/// Fetches the board's column table through the info callee (id `0x9f`,
/// table at frame offset `+0x14`), resolves `table[index]` to a type code
/// through the second callee, and maps the code to a width: 1 and 5 give 4
/// bytes, 2 and 3 give 8 bytes, anything else (including -1) gives 0.
/// Returns 0 when the info callee reports failure. stdcall, one stack
/// argument; entry ECX ignored.
lf_checker_rt::export!(stdcall, rw_00546fa0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x9f;
        const INFO_CALLEE: u32 = 1;
        const TYPE_CALLEE: u32 = 2;
        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(INFO_CALLEE, u32,
            LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if (ok & 0xff) == 0 {
            return 0;
        }
        let col = ((info[5].wrapping_add(index.wrapping_mul(4)))
            as *const u32)
            .read_unaligned();
        let code: u32 = lf_checker_rt::callee_thiscall!(TYPE_CALLEE, u32, col);
        if code == 0xffff_ffff {
            return 0;
        }
        match code.wrapping_sub(1) {
            0 => 4,
            1 | 2 => 8,
            4 => 4,
            _ => 0,
        }
    }
});
