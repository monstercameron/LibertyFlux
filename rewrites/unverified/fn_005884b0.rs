// original: 0x005884B0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_213, player_schema::LeaderboardInfo, 10>::vf8

/// Board query vf8 of one ranked-race leaderboard instantiation.
///
/// Reads the table entry at `index`, asks the type query for its kind,
/// and maps the kind to a field width: kind 1 or 5 gives 4, kind 2 or 3
/// gives 8, anything else (including a failed helper call or a failed
/// query) gives 0. The original dispatches through a five-entry jump table;
/// the mapping above is its content.
///
/// TABLE_SLOT (word 5): value table for the index argument.
/// Board id 0x1ad, passed to the helper in ecx.
///
/// Original: stdcall, one stack word, no register inputs (ecx is set to the
/// board id before the helper call), return value in eax, no heap or global
/// writes.
lf_checker_rt::export!(stdcall, rw_005884B0(index: u32) -> u32 {
    unsafe {
        /// Board id passed to the schema helper in ecx.
        const BOARD_ID: u32 = 0x1ad;
        /// Helper callee id in the proof contract.
        const HELPER: u32 = 1;
        /// Helper out-slot holding the value table.
        const TABLE_SLOT: usize = 5;
        /// Type-query callee id in the proof contract.
        const TYPE_OF: u32 = 2;

        let mut out = [0u32; 6];
        let ok: u8 = lf_checker_rt::callee_fastcall!(HELPER, u8, BOARD_ID, out.as_mut_ptr() as u32);
        if ok == 0 {
            return 0;
        }
        let table = out[TABLE_SLOT];
        let entry = ((table.wrapping_add(index.wrapping_mul(4))) as *const u32).read_unaligned();
        let kind: u32 = lf_checker_rt::callee_thiscall!(TYPE_OF, u32, entry);
        if kind == 0xffff_ffff {
            return 0;
        }
        // Five-entry jump table in the original: kinds 1..=5 map to
        // [4, 8, 8, 0, 4]; kind 0 underflows the decrement and misses.
        match kind.wrapping_sub(1) {
            0 => 4,
            1 | 2 => 8,
            4 => 4,
            _ => 0,
        }
    }
});

