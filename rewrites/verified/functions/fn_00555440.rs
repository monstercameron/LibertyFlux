// original: 0x00555440 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_27, player_schema::LeaderboardInfo, 10>::vf12

/// Look up one leaderboard entry and return its position in the ranked list.
///
/// `index` selects a row of the entry table. The info callee (id 1, called with
/// `LEADERBOARD_ID` in ecx and a pointer to a six-word out-struct in edx) fills
/// `COUNT` (struct word 1), `LIST` (word 2, the ranked id list) and `TABLE`
/// (word 5, the entry table); the three words are zeroed before the call. When
/// the callee reports failure (al == 0) the result is `NOT_FOUND`, as it is when
/// the selected entry is `NOT_FOUND` itself, when the list is empty, or when the
/// entry is absent from the first `COUNT` list words (compared unsigned). The
/// found position is a zero-based index.
///
/// Original: 0x00555440 (stdcall, one stack word; ecx is ignored).
lf_checker_rt::export!(stdcall, rw_00555440(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0xed;
        const INFO_CALLEE: u32 = 1;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut info = [0u32; 6];
        info[1] = 0; // COUNT
        info[2] = 0; // LIST
        info[5] = 0; // TABLE
        let ok: u32 = lf_checker_rt::callee_fastcall!(INFO_CALLEE, u32,
            LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if ok as u8 == 0 {
            return NOT_FOUND;
        }
        let key = (info[5].wrapping_add(index.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        if key == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = info[1];
        if count == 0 {
            return NOT_FOUND;
        }
        let list = info[2];
        let mut i = 0u32;
        loop {
            let v = (list.wrapping_add(i.wrapping_mul(4)) as *const u32)
                .read_unaligned();
            if v == key {
                return i;
            }
            i = i.wrapping_add(1);
            if i >= count {
                return NOT_FOUND;
            }
        }
    }
});
