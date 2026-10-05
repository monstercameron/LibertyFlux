// original: 0x005461e0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_16, player_schema::LeaderboardInfo, 10>::vf6

/// Index of a row id in this leaderboard's row array, or -1 when absent.
///
/// Fetches the board's row list through the info callee (id `0xd3`, out-words
/// at the frame pointer: count, array), then scans the array with a signed
/// comparison for `wanted` and returns the first matching index. Returns -1
/// when the callee reports failure (low byte of its answer is zero) or the
/// count is not positive. stdcall, one stack argument; entry ECX ignored.
lf_checker_rt::export!(stdcall, rw_005461e0(wanted: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0xd3;
        const INFO_CALLEE: u32 = 1;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut info = [0u32; 7];
        let ok: u32 = lf_checker_rt::callee_fastcall!(INFO_CALLEE, u32,
            LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if (ok & 0xff) == 0 {
            return NOT_FOUND;
        }
        let count = info[3];
        let items = info[4];
        if (count as i32) <= 0 {
            return NOT_FOUND;
        }
        let mut i = 0u32;
        while (i as i32) < (count as i32) {
            let v = ((items.wrapping_add(i.wrapping_mul(4))) as *const u32)
                .read_unaligned();
            if v == wanted {
                return i;
            }
            i += 1;
        }
        NOT_FOUND
    }
});
