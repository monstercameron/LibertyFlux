// original: 0x00546ca0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_1, player_schema::LeaderboardInfo, 10>::vf12

/// Index of one leaderboard row id in the board's row array, or -1.
///
/// Fetches the board data through the info callee (id `0x9f`, out-words at
/// the frame pointer: count at `+4`, array at `+8`, id table at `+0x14`),
/// reads the row id `table[index]`, and returns the first position holding
/// it in the array (unsigned scan). Returns -1 when the callee reports
/// failure, the row id is -1, the count is zero, or the id is absent.
/// stdcall, one stack argument; entry ECX ignored.
lf_checker_rt::export!(stdcall, rw_00546ca0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x9f;
        const INFO_CALLEE: u32 = 1;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut info = [0u32; 7];
        let ok: u32 = lf_checker_rt::callee_fastcall!(INFO_CALLEE, u32,
            LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if (ok & 0xff) == 0 {
            return NOT_FOUND;
        }
        let count = info[1];
        let items = info[2];
        let table = info[5];
        let want = ((table.wrapping_add(index.wrapping_mul(4)))
            as *const u32)
            .read_unaligned();
        if want == NOT_FOUND {
            return NOT_FOUND;
        }
        if count == 0 {
            return NOT_FOUND;
        }
        let mut i = 0u32;
        while i < count {
            let v = ((items.wrapping_add(i.wrapping_mul(4))) as *const u32)
                .read_unaligned();
            if v == want {
                return i;
            }
            i += 1;
        }
        NOT_FOUND
    }
});
