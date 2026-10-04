// original: 0x00589330 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_217, player_schema::LeaderboardInfo, 10>::vf12

/// Board query vf12 of one ranked-race leaderboard instantiation.
///
/// Reads the table entry at `index` and returns the position of its
/// first occurrence in the search array, or all-ones when the helper fails,
/// the table entry itself is all-ones, the count is zero, or no element
/// matches. The count bound is unsigned.
///
/// COUNT_SLOT (word 1): entry count of the search array.
/// ARRAY_SLOT (word 2): search array of entry values.
/// TABLE_SLOT (word 5): value table for the index argument.
/// Board id 0x1b1, passed to the helper in ecx.
///
/// Original: stdcall, one stack word, no register inputs (ecx is set to the
/// board id before the helper call), return value in eax, no heap or global
/// writes.
lf_checker_rt::export!(stdcall, rw_00589330(index: u32) -> u32 {
    unsafe {
        /// Board id passed to the schema helper in ecx.
        const BOARD_ID: u32 = 0x1b1;
        /// Helper callee id in the proof contract.
        const HELPER: u32 = 1;
        /// Helper out-slots: entry count, search array, value table.
        const COUNT_SLOT: usize = 1;
        const ARRAY_SLOT: usize = 2;
        const TABLE_SLOT: usize = 5;
        /// Failure and table-miss marker.
        const NOT_FOUND: u32 = 0xffff_ffff;

        let mut out = [0u32; 6];
        let ok: u8 = lf_checker_rt::callee_fastcall!(HELPER, u8, BOARD_ID, out.as_mut_ptr() as u32);
        if ok == 0 {
            return NOT_FOUND;
        }
        let table = out[TABLE_SLOT];
        let entry = ((table.wrapping_add(index.wrapping_mul(4))) as *const u32).read_unaligned();
        if entry == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = out[COUNT_SLOT];
        let array = out[ARRAY_SLOT];
        // Unsigned count bound, first match wins.
        let mut i = 0u32;
        while i < count {
            let v = ((array.wrapping_add(i.wrapping_mul(4))) as *const u32).read_unaligned();
            if v == entry {
                return i;
            }
            i = i.wrapping_add(1);
        }
        NOT_FOUND
    }
});

