// original: 0x00519230 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race9NoHolds, player_schema::LeaderboardInfo, 10>::vf9

/// Rank one entry of this leaderboard's value table through the class
/// callee's five-row rank table.
///
/// `index` selects the entry. A scratch row is handed to the schema
/// callee (id `0x66`) which fills in the value-table pointer (row word
/// at `+0x14`); a zero answer means no table and yields `NOT_FOUND`.
/// The entry is classified by the class callee; kind `NOT_FOUND`, or
/// a kind whose predecessor falls outside the five-row table, yields
/// `NOT_FOUND`. Otherwise the row gives the rank: row k holds `0, 1, 3, -1, 2`.
///
/// Original: 0x00519230 (stdcall, one stack word; incoming registers ignored).
lf_checker_rt::export!(stdcall, rw_00519230(index: u32) -> u32 {
    unsafe {        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        const SCHEMA_ID: u32 = 0x66;
        const SCHEMA_CALLEE: u32 = 1;
        const CLASS_CALLEE: u32 = 2;
        const NOT_FOUND: u32 = 0xffff_ffff;
        const RANKS: [u32; 5] = [0, 1, 3, 0xffff_ffff, 2];

        // Scratch row the schema callee fills: value table at +0x14.
        let mut row = [0u32; 8];
        row[5] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(SCHEMA_CALLEE, u32, SCHEMA_ID, row.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return NOT_FOUND;
        }
        let table = row[5];
        let entry = rd32(table.wrapping_add(index.wrapping_mul(4)));
        let kind: u32 = lf_checker_rt::callee_thiscall!(CLASS_CALLEE, u32, entry);
        if kind == NOT_FOUND {
            return NOT_FOUND;
        }
        let arm = kind.wrapping_sub(1);
        if arm > 4 {
            return NOT_FOUND;
        }
        RANKS[arm as usize]
    }
});
