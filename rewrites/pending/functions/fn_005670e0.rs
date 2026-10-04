// original: 0x005670e0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_92, player_schema::LeaderboardInfo, 10>::vf13

/// Leaderboard payload for a value, looked up by its id position.
///
/// Arguments: the object (unused) and the value to find.
///
/// Calls the leaderboard fetch helper (fastcall: id 0x127 in ecx,
/// out struct in edx, no stack arguments) and reads the signed count at
/// `+0x0c`, the id array at `+0x10`, and the payload array at
/// `+0x14`. A zero status byte means failure.
///
/// On success scans the id array in order and returns the payload
/// word at the first matching position.
///
/// Edge cases: fetch failure, count at or below zero, and a value
/// that never appears all return -1 (0xffffffff).
///
/// Original: thiscall, one stack word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_005670e0(_this: u32, want: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x127;
        const COUNT_OFF: usize = 0x0c / 4;
        const IDS_OFF: usize = 0x10 / 4;
        const DATA_OFF: usize = 0x14 / 4;
        const FETCH_CALLEE: u32 = 1;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut info = [0u32; 8];
        let answered: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if answered & 0xff == 0 {
            return NOT_FOUND;
        }
        let count = info[COUNT_OFF] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let ids = info[IDS_OFF];
        let mut index = 0i32;
        while index < count {
            let at = ids.wrapping_add((index as u32).wrapping_mul(4));
            if (at as *const u32).read_unaligned() == want {
                let dat = info[DATA_OFF].wrapping_add((index as u32).wrapping_mul(4));
                return (dat as *const u32).read_unaligned();
            }
            index += 1;
        }
        NOT_FOUND
    }
});
