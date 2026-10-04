// original: 0x00587BB0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_211, player_schema::LeaderboardInfo, 10>::vf7

/// Board query vf7 of one ranked-race leaderboard instantiation.
///
/// Returns the table entry at `index`, or all-ones when the helper
/// fails. There is no bounds check: the caller guarantees the index.
///
/// TABLE_SLOT (word 4): value table for the index argument.
/// Board id 0x1ab, passed to the helper in ecx.
///
/// Original: stdcall, one stack word, no register inputs (ecx is set to the
/// board id before the helper call), return value in eax, no heap or global
/// writes.
lf_checker_rt::export!(stdcall, rw_00587BB0(index: u32) -> u32 {
    unsafe {
        /// Board id passed to the schema helper in ecx.
        const BOARD_ID: u32 = 0x1ab;
        /// Helper callee id in the proof contract.
        const HELPER: u32 = 1;
        /// Helper out-slot holding the value table.
        const TABLE_SLOT: usize = 4;
        /// Failure marker, also the table-miss value.
        const NOT_FOUND: u32 = 0xffff_ffff;

        let mut out = [0u32; 5];
        let ok: u8 = lf_checker_rt::callee_fastcall!(HELPER, u8, BOARD_ID, out.as_mut_ptr() as u32);
        if ok == 0 {
            return NOT_FOUND;
        }
        let table = out[TABLE_SLOT];
        let entry = ((table.wrapping_add(index.wrapping_mul(4))) as *const u32).read_unaligned();
        entry
    }
});

