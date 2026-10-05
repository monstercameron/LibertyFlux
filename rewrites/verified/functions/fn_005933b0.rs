// original: 0x005933b0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_253, player_schema::LeaderboardInfo, 10>::vf8

/// Classify one leaderboard row: look it up, pass it through the kind callee,
/// map the kind to a size (1 or 5 to 4, 2 or 3 to 8, anything else to 0).
///
/// Calls the leaderboard-info callee for board `LEADERBOARD_ID`; word `+5` (row
/// array) selects the row by `index`, the kind callee maps the row to a kind,
/// and the switch maps the kind to the result. Kept for a later re-run: the
/// original's jump table has no relocations, so the checker cannot run this
/// function from a relocated image.
/// Original: stdcall, one stack word, the callee pops 4 bytes.
lf_checker_rt::export!(stdcall, rw_005933b0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1d5;
        const INFO_CALLEE: u32 = 1;
        const KIND_CALLEE: u32 = 2;
        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(INFO_CALLEE, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return 0;
        }
        let rows = info[5];
        let v = (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        let kind: u32 = lf_checker_rt::callee_thiscall!(KIND_CALLEE, u32, v);
        match kind {
            1 => 4,
            2 | 3 => 8,
            5 => 4,
            _ => 0,
        }
    }
});
