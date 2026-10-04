// original: 0x0054d9c0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_26, player_schema::LeaderboardInfo, 10>::vf2

/// Publish this board's id through `out` when the caller knows the current id.
///
/// Reads the object's virtual table, calls slot 1 (which takes no stack
/// arguments) to get the current id, and compares it with `key`. When they
/// match and `out` is non-null, writes this board's descriptor address
/// (0xfd47cc in the file image, rebased at load; it points into read-only data)
/// to `*out` and returns `out`. Returns 0 when the ids differ or when
/// `out` is null (nothing is written then).
///
/// Original: thiscall, ECX = object, two stack words (out, key).
lf_checker_rt::export!(thiscall, rw_0054d9c0(this: u32, out: u32, key: u32) -> u32 {
    unsafe {
        const BOARD_FILE_VA: u32 = 0xfd47cc;
        let board = lf_checker_rt::relocated(BOARD_FILE_VA);
        let vtable = (this as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(4) as *const u32).read_unaligned();
        let current_id: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let got = current_id(this);
        if got != key {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(board);
        out
    }
});
