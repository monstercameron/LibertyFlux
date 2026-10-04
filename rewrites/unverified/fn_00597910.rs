// original: 0x00597910 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_269, player_schema::LeaderboardInfo, 10>::vf6

/// Find a column id in this leaderboard's column table (index lookup).
///
/// `this` (ECX, thiscall) is the leaderboard-info object; it is never read:
/// the board is identified solely by `BOARD_ID`. `want` is the column id to
/// find. The schema callee (fastcall: ECX = board id, EDX = out-struct) is
/// asked for the table; it reports success in AL and fills `COUNT_OFF`
/// (element count, signed) and `ARRAY_OFF` (pointer to the id array).
///
/// On success the array is scanned linearly and the first index whose element
/// equals `want` is returned. `NOT_FOUND` (-1) is returned when the callee
/// reports failure, when the count is not positive, or when no element
/// matches. A negative count takes the not-found path without reading the
/// array.
///
/// Original: 0x00597910 (thiscall, one stack word; ECX ignored).
lf_checker_rt::export!(thiscall, rw_00597910(_this: u32, want: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x1e1;
        const CALLEE_SCHEMA: u32 = 1;
        const COUNT_OFF: usize = 0x0c;
        const ARRAY_OFF: usize = 0x10;
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
        let count = schema[COUNT_OFF / 4] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let items = schema[ARRAY_OFF / 4] as *const u32;
        let mut i = 0i32;
        while i < count {
            if *items.offset(i as isize) == want {
                return i as u32;
            }
            i += 1;
        }
        NOT_FOUND
    }
});
