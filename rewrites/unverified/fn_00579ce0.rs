// original: 0x00579CE0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_160, player_schema::LeaderboardInfo, 10>::vf8

/// Classify one leaderboard table entry through a helper call.
///
/// `index` selects an entry of the id table the lookup callee returns;
/// that value goes to a classifier callee whose result maps to a size:
/// 1 -> 4, 2 -> 8, 3 -> 8, 4 -> 0, 5 -> 4, anything else (including -1,
/// 0, and values above 5) -> 0. A failed lookup returns 0.
///
/// Layout read (words into the out-block): id-table pointer at +5. The
/// mapping above is the original's jump table, restated as a match so the
/// rewrite never reads the original's code pages.
///
/// Original: 0x00579CE0 (declared stdcall/1; incoming ECX ignored; schema id
/// 0x178).
lf_checker_rt::export!(stdcall, rw_00579CE0(index: u32) -> u32 {
    unsafe {        const SCHEMA_ID: u32 = 0x178;
        const LOOKUP: u32 = 1;
        const CLASSIFY: u32 = 2;
        const FAILED: u32 = 0xFFFF_FFFF;
        const TABLE_SLOT: usize = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut out = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(LOOKUP, u32, SCHEMA_ID, out.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return 0;
        }
        let table = out[TABLE_SLOT];
        let v = rd32(table.wrapping_add(index.wrapping_mul(4)));
        let r: u32 = lf_checker_rt::callee_thiscall!(CLASSIFY, u32, v);
        if r == FAILED {
            return 0;
        }
        match r.wrapping_sub(1) {
            0 => 4,
            1 => 8,
            2 => 8,
            3 => 0,
            4 => 4,
            _ => 0,
        }

    }
});
