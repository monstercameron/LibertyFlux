// original: 0x005983D0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Standard_Episodic_2, player_schema::LeaderboardInfo, 10>::vf12

/// Find a leaderboard entry id in the board's key table (two-stage lookup).
///
/// `this` (ECX, thiscall) is the leaderboard-info object; it is never read:
/// the board is identified solely by `BOARD_ID`. `index` counts in 4-byte
/// elements. The schema callee (fastcall: ECX = board id, EDX = out-struct)
/// is asked for the tables; it reports success in AL and fills `IDS_OFF`
/// (pointer to the entry-id array), `COUNT_OFF` (key count, compared
/// unsigned) and `KEYS_OFF` (pointer to the key array).
///
/// On success the entry id at `index` is read; a -1 entry id, a zero key
/// count, or no matching key all return `NOT_FOUND` (-1), else the first
/// index whose key equals the entry id.
///
/// Original: 0x005983D0 (thiscall, one stack word; ECX ignored).
lf_checker_rt::export!(thiscall, rw_005983D0(_this: u32, index: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x155;
        const CALLEE_SCHEMA: u32 = 1;
        const COUNT_OFF: usize = 0x04;
        const KEYS_OFF: usize = 0x08;
        const IDS_OFF: usize = 0x14;
        const NOT_FOUND: u32 = 0xffff_ffff;

        let mut schema = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            CALLEE_SCHEMA,
            u32,
            BOARD_ID,
            schema.as_mut_ptr() as u32
        );
        if ok & 0xff == 0 {
            return NOT_FOUND;
        }
        let ids = schema[IDS_OFF / 4] as *const u32;
        let key = *ids.add(index as usize);
        if key == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = schema[COUNT_OFF / 4];
        if count == 0 {
            return NOT_FOUND;
        }
        let keys = schema[KEYS_OFF / 4] as *const u32;
        let mut i = 0u32;
        while i < count {
            if *keys.add(i as usize) == key {
                return i;
            }
            i += 1;
        }
        NOT_FOUND
    }
});
