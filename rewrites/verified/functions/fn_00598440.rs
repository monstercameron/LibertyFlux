// original: 0x00598440 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Standard_Episodic_2, player_schema::LeaderboardInfo, 10>::vf13

/// Map a column id to its value through this leaderboard's tables.
///
/// `this` (ECX, thiscall) is the leaderboard-info object; it is never read:
/// the board is identified solely by `BOARD_ID`. `want` is the column id to
/// find. The schema callee (fastcall: ECX = board id, EDX = out-struct) is
/// asked for the tables; it reports success in AL and fills `COUNT_OFF`
/// (element count, signed), `KEYS_OFF` (pointer to the id array) and
/// `VALS_OFF` (pointer to the parallel value array).
///
/// On success the id array is scanned linearly; the value at the first index
/// whose id equals `want` is returned. `NOT_FOUND` (-1) is returned when the
/// callee reports failure, the count is not positive, or no id matches. The
/// original re-checks the found index against -1 after the search; that guard
/// is dead (the search starts at 0 and only increments) and is mirrored here.
///
/// Original: 0x00598440 (thiscall, one stack word; ECX ignored).
lf_checker_rt::export!(thiscall, rw_00598440(_this: u32, want: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x155;
        const CALLEE_SCHEMA: u32 = 1;
        const COUNT_OFF: usize = 0x0c;
        const KEYS_OFF: usize = 0x10;
        const VALS_OFF: usize = 0x14;
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
        let keys = schema[KEYS_OFF / 4] as *const u32;
        let mut i = 0i32;
        while i < count {
            if *keys.offset(i as isize) == want {
                break;
            }
            i += 1;
        }
        if i >= count {
            return NOT_FOUND;
        }
        if i == -1 {
            return NOT_FOUND;
        }
        let vals = schema[VALS_OFF / 4] as *const u32;
        *vals.offset(i as isize)
    }
});
