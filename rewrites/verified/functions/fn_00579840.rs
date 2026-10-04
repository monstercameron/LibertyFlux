// original: 0x00579840 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_159, player_schema::LeaderboardInfo, 10>::vf7

/// Read one entry of the leaderboard id table.
///
/// `index` selects an entry of the id table the lookup callee returns;
/// that entry is the result, or -1 (as u32) when the lookup fails. No
/// bounds check: a wild index reads past the table, exactly like the
/// original.
///
/// Layout read (words into the out-block): id-table pointer at +4.
///
/// Original: 0x00579840 (declared stdcall/1; incoming ECX ignored; schema id
/// 0x177).
lf_checker_rt::export!(stdcall, rw_00579840(index: u32) -> u32 {
    unsafe {        const SCHEMA_ID: u32 = 0x177;
        const LOOKUP: u32 = 1;
        const MISSING: u32 = 0xFFFF_FFFF;
        const TABLE_SLOT: usize = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut out = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(LOOKUP, u32, SCHEMA_ID, out.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return MISSING;
        }
        let table = out[TABLE_SLOT];
        rd32(table.wrapping_add((index).wrapping_mul(4)))

    }
});
