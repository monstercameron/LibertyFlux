// original: 0x00598B30 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Standard_Episodic_3, player_schema::LeaderboardInfo, 10>::vf8

/// Size class of one entry of this leaderboard's id table.
///
/// `this` (ECX, thiscall) is the leaderboard-info object; it is never read:
/// the board is identified solely by `BOARD_ID`. `index` counts in 4-byte
/// elements. The schema callee (fastcall: ECX = board id, EDX = out-struct)
/// is asked for the table; it reports success in AL and fills `ARRAY_OFF`.
/// The indexed entry (a column key) is passed in ECX to the kind callee,
/// whose 1-based answer selects a row of a five-way switch:
///
/// | kind | 1 | 2 | 3 | 4 | 5 |
/// |------|---|---|---|---|---|
/// | size | 4 | 8 | 8 | 0 | 4 |
///
/// Zero is returned when the schema callee reports failure, when the kind is
/// -1 or 0, or when it falls outside 1..=5.
///
/// Original: 0x00598B30 (thiscall, one stack word; ECX ignored).
lf_checker_rt::export!(thiscall, rw_00598B30(_this: u32, index: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x154;
        const CALLEE_SCHEMA: u32 = 1;
        const CALLEE_KIND: u32 = 2;
        const ARRAY_OFF: usize = 0x14;
        const KIND_NONE: u32 = 0xffff_ffff;

        let mut schema = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            CALLEE_SCHEMA,
            u32,
            BOARD_ID,
            schema.as_mut_ptr() as u32
        );
        if ok & 0xff == 0 {
            return 0;
        }
        let items = schema[ARRAY_OFF / 4] as *const u32;
        let key = *items.add(index as usize);
        let kind: u32 = lf_checker_rt::callee_thiscall!(CALLEE_KIND, u32, key);
        if kind == KIND_NONE {
            return 0;
        }
        let slot = kind.wrapping_sub(1);
        if slot > 4 {
            return 0;
        }
        match slot {
            0 => 4,
            1 => 8,
            2 => 8,
            3 => 0,
            _ => 4,
        }
    }
});
