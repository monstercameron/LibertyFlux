// original: 0x00599000 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Standard_Episodic_4, player_schema::LeaderboardInfo, 10>::vf9

/// Rank of one entry of this leaderboard's id table.
///
/// Same call pattern as the sibling size-class lookup: `this` (ECX,
/// thiscall) is never read, `BOARD_ID` names the board, `index` counts in
/// 4-byte elements, the schema callee fills `ARRAY_OFF`, and the indexed key
/// is passed in ECX to the kind callee. Its 1-based answer selects a row of
/// a five-way switch:
///
/// | kind | 1 | 2 | 3 | 4 | 5 |
/// |------|---|---|---|---|---|
/// | rank | 0 | 1 | 3 | -1 | 2 |
///
/// -1 is returned when the schema callee reports failure, when the kind is
/// -1 or 0, or when it falls outside 1..=5.
///
/// Original: 0x00599000 (thiscall, one stack word; ECX ignored).
lf_checker_rt::export!(thiscall, rw_00599000(_this: u32, index: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x156;
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
            return KIND_NONE;
        }
        let items = schema[ARRAY_OFF / 4] as *const u32;
        let key = *items.add(index as usize);
        let kind: u32 = lf_checker_rt::callee_thiscall!(CALLEE_KIND, u32, key);
        if kind == KIND_NONE {
            return KIND_NONE;
        }
        let slot = kind.wrapping_sub(1);
        if slot > 4 {
            return KIND_NONE;
        }
        match slot {
            0 => 0,
            1 => 1,
            2 => 3,
            3 => KIND_NONE,
            _ => 2,
        }
    }
});
