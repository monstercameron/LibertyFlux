// original: 0x00567070 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_92, player_schema::LeaderboardInfo, 10>::vf12

/// Position of an indexed leaderboard key inside the id array, or -1.
///
/// Arguments: the object (unused) and the index.
///
/// Calls the leaderboard fetch helper (fastcall: id 0x127 in ecx,
/// out struct in edx, no stack arguments) and reads the key array at
/// `+0x14`, the id count at `+0x04`, and the id array at `+0x08`.
/// A zero status byte means failure.
///
/// On success takes the key at the index and scans the id array
/// (unsigned length) for it, returning the first match.
///
/// Edge cases: fetch failure, a -1 key, an empty id array, and a
/// key that never appears all return -1 (0xffffffff). The index is
/// trusted and a wild one faults exactly like the original.
///
/// Original: thiscall, one stack word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00567070(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x127;
        const COUNT_OFF: usize = 0x04 / 4;
        const IDS_OFF: usize = 0x08 / 4;
        const KEYS_OFF: usize = 0x14 / 4;
        const FETCH_CALLEE: u32 = 1;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut info = [0u32; 8];
        let answered: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if answered & 0xff == 0 {
            return NOT_FOUND;
        }
        let kat = info[KEYS_OFF].wrapping_add(index.wrapping_mul(4));
        let key = (kat as *const u32).read_unaligned();
        if key == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = info[COUNT_OFF];
        if count == 0 {
            return NOT_FOUND;
        }
        let ids = info[IDS_OFF];
        let mut i = 0u32;
        while i < count {
            let at = ids.wrapping_add(i.wrapping_mul(4));
            if (at as *const u32).read_unaligned() == key {
                return i;
            }
            i += 1;
        }
        NOT_FOUND
    }
});
