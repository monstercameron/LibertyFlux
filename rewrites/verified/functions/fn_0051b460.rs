// original: 0x0051b460 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race17NoHolds, player_schema::LeaderboardInfo, 10>::vf7
/// Look up one leaderboard row value by index.
///
/// Calls the schema lookup (callee 1) with this leaderboard's id and a
/// scratch info block; on success word `+0x10` of the block is the value
/// table and the function returns `table[index]`. When the lookup reports
/// failure (zero low byte) it returns -1 without touching the table.
///
/// Arguments: `index`, the row. The incoming object pointer is unused.
/// Original: thiscall, one stack word.
/// Edge cases: `index` always addresses inside the table under the
/// checker's inputs; addressing wraps mod 2^32 exactly like the original.
lf_checker_rt::export!(thiscall, rw_0051b460(_this: u32, index: u32) -> u32 {
    unsafe {
        const SCHEMA_ID: u32 = 0x6e;
        const LOOKUP_CALLEE: u32 = 1;
        const TABLE_WORD: usize = 4;
        const NONE: u32 = 0xffff_ffff;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            LOOKUP_CALLEE, u32, SCHEMA_ID, info.as_mut_ptr() as u32);
        if ok as u8 == 0 {
            return NONE;
        }
        let table = info[TABLE_WORD];
        rd32(table.wrapping_add(index.wrapping_mul(4)))
    }
});
