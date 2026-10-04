// original: 0x00555050 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_26, player_schema::LeaderboardInfo, 10>::vf13

/// Map one leaderboard id through the ranked list to its stored value.
///
/// `want` is the id to find. The info callee (id 1, called with
/// `LEADERBOARD_ID` in ecx and a pointer to a six-word out-struct in edx) fills
/// `COUNT` (struct word 3), `LIST` (word 4, the ranked id list) and `MAP`
/// (word 5, the parallel value list); the three words are zeroed before the
/// call. When the callee reports failure (al == 0) the result is `NOT_FOUND`,
/// as it is when `COUNT` is zero or negative (compared signed) or when `want`
/// is absent from the first `COUNT` list words. Otherwise the result is the
/// `MAP` word at the found position. The original re-tests the found position
/// against -1 before indexing `MAP`; that branch never fires (a found position
/// is never negative) and is not reproduced.
///
/// Original: 0x00555050 (stdcall, one stack word; ecx is ignored).
lf_checker_rt::export!(stdcall, rw_00555050(want: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0xe9;
        const INFO_CALLEE: u32 = 1;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut info = [0u32; 6];
        info[3] = 0; // COUNT
        info[4] = 0; // LIST
        info[5] = 0; // MAP
        let ok: u32 = lf_checker_rt::callee_fastcall!(INFO_CALLEE, u32,
            LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if ok as u8 == 0 {
            return NOT_FOUND;
        }
        let count = info[3] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let list = info[4];
        let mut i = 0u32;
        loop {
            let v = (list.wrapping_add(i.wrapping_mul(4)) as *const u32)
                .read_unaligned();
            if v == want {
                let map = info[5];
                return (map.wrapping_add(i.wrapping_mul(4)) as *const u32)
                    .read_unaligned();
            }
            i = i.wrapping_add(1);
            if (i as i32) >= count {
                return NOT_FOUND;
            }
        }
    }
});
