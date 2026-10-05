// original: 0x00545740 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_14, player_schema::LeaderboardInfo, 10>::vf13

/// Leaderboard cell value for one row id, or -1 when the row is absent.
///
/// Fetches the board data through the info callee (id `0xae`, out-words at
/// the frame pointer: count at `+0xc`, array at `+0x10`, value table at
/// `+0x14`), scans the array with a signed comparison for `wanted`, and
/// returns the value-table word at the matching position. Returns -1 when
/// the callee reports failure, the count is not positive, or the id is
/// absent. (The original re-checks the found index against -1, which an
/// unsigned loop counter can never equal; that check is dead.) stdcall,
/// one stack argument; entry ECX ignored.
lf_checker_rt::export!(stdcall, rw_00545740(wanted: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0xae;
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
        let table = info[5];
        if (count as i32) <= 0 {
            return NOT_FOUND;
        }
        let mut i = 0u32;
        while (i as i32) < (count as i32) {
            let v = ((items.wrapping_add(i.wrapping_mul(4))) as *const u32)
                .read_unaligned();
            if v == wanted {
                return ((table.wrapping_add(i.wrapping_mul(4)))
                    as *const u32)
                    .read_unaligned();
            }
            i += 1;
        }
        NOT_FOUND
    }
});
