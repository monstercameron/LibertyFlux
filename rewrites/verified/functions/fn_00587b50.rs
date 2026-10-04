// original: 0x00587B50 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_211, player_schema::LeaderboardInfo, 10>::vf6

/// Board query vf6 of one ranked-race leaderboard instantiation.
///
/// Returns the position of the first search-array element equal to
/// `want`, or all-ones when the helper fails, the signed count is not
/// positive, or no element matches.
///
/// COUNT_SLOT (word 3): signed entry count of the search array.
/// ARRAY_SLOT (word 4): search array of entry values.
/// Board id 0x1ab, passed to the helper in ecx.
///
/// Original: stdcall, one stack word, no register inputs (ecx is set to the
/// board id before the helper call), return value in eax, no heap or global
/// writes.
lf_checker_rt::export!(stdcall, rw_00587B50(want: u32) -> u32 {
    unsafe {
        /// Board id passed to the schema helper in ecx.
        const BOARD_ID: u32 = 0x1ab;
        /// Helper callee id in the proof contract.
        const HELPER: u32 = 1;
        /// Helper out-slots: signed entry count, search array.
        const COUNT_SLOT: usize = 3;
        const ARRAY_SLOT: usize = 4;
        /// Failure and search-miss marker.
        const NOT_FOUND: u32 = 0xffff_ffff;

        let mut out = [0u32; 5];
        let ok: u8 = lf_checker_rt::callee_fastcall!(HELPER, u8, BOARD_ID, out.as_mut_ptr() as u32);
        if ok == 0 {
            return NOT_FOUND;
        }
        let count = out[COUNT_SLOT] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let array = out[ARRAY_SLOT];
        let n = count as u32;
        let mut i = 0u32;
        while i < n {
            let v = ((array.wrapping_add(i.wrapping_mul(4))) as *const u32).read_unaligned();
            if v == want {
                return i;
            }
            i = i.wrapping_add(1);
        }
        NOT_FOUND
    }
});

