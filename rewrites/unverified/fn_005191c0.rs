// original: 0x005191C0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race9NoHolds, player_schema::LeaderboardInfo, 10>::vf8

/// Classify one entry of this leaderboard's value table into a width.
///
/// `index` selects the entry. A scratch row is handed to the schema
/// callee (id `0x66`) which fills in the value-table pointer (row word
/// at `+0x14`); a zero answer means no table and yields 0. The entry
/// is classified by the class callee; kind `NOT_FOUND`, or a kind
/// whose predecessor falls outside the five-row table, yields 0.
/// Otherwise the row gives the width: row k holds `4, 8, 8, 0, 4`.
///
/// Original: 0x005191C0 (stdcall, one stack word; incoming registers ignored).
lf_checker_rt::export!(stdcall, rw_005191c0(index: u32) -> u32 {
    unsafe {        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        const SCHEMA_ID: u32 = 0x66;
        const SCHEMA_CALLEE: u32 = 1;
        const CLASS_CALLEE: u32 = 2;
        const NOT_FOUND: u32 = 0xffff_ffff;
        const WIDTHS: [u32; 5] = [4, 8, 8, 0, 4];

        // Scratch row the schema callee fills: value table at +0x14.
        let mut row = [0u32; 8];
        row[5] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(SCHEMA_CALLEE, u32, SCHEMA_ID, row.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return 0;
        }
        let table = row[5];
        let entry = rd32(table.wrapping_add(index.wrapping_mul(4)));
        let kind: u32 = lf_checker_rt::callee_thiscall!(CLASS_CALLEE, u32, entry);
        if kind == NOT_FOUND {
            return 0;
        }
        let arm = kind.wrapping_sub(1);
        if arm > 4 {
            return 0;
        }
        WIDTHS[arm as usize]
    }
});
