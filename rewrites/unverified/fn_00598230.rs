// original: 0x00598230 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Standard_Episodic_1, player_schema::LeaderboardInfo, 10>::vf7

/// Read one entry of this leaderboard's id table by index.
///
/// `this` (ECX, thiscall) is the leaderboard-info object; it is never read:
/// the board is identified solely by `BOARD_ID`. `index` counts in 4-byte
/// elements. The schema callee (fastcall: ECX = board id, EDX = out-struct)
/// is asked for the table; it reports success in AL and fills `ARRAY_OFF`
/// (pointer to the id array).
///
/// On success the element at `index` is returned. `NOT_FOUND` (-1) is
/// returned only when the callee reports failure; the index itself is not
/// bounds-checked, matching the original.
///
/// Original: 0x00598230 (thiscall, one stack word; ECX ignored).
lf_checker_rt::export!(thiscall, rw_00598230(_this: u32, index: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x153;
        const CALLEE_SCHEMA: u32 = 1;
        const ARRAY_OFF: usize = 0x10;
        const NOT_FOUND: u32 = 0xffff_ffff;

        let mut schema = [0u32; 5];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            CALLEE_SCHEMA,
            u32,
            BOARD_ID,
            schema.as_mut_ptr() as u32
        );
        if ok & 0xff == 0 {
            return NOT_FOUND;
        }
        let items = schema[ARRAY_OFF / 4] as *const u32;
        *items.add(index as usize)
    }
});
