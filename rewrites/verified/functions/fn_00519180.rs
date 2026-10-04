// original: 0x00519180 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race9NoHolds, player_schema::LeaderboardInfo, 10>::vf7

/// Read one entry of this leaderboard's value table by index.
///
/// `index` selects the entry. A scratch row is handed to the schema
/// callee (id `0x66`) which fills in the value-table pointer (row word
/// at `+0x10`); a zero answer means no table and yields `NOT_FOUND`.
///
/// Edge cases: none beyond the missing-table case; the index is used
/// as given (`NOT_FOUND` is `0xffff_ffff`).
///
/// Original: 0x00519180 (stdcall, one stack word; incoming registers ignored).
lf_checker_rt::export!(stdcall, rw_00519180(index: u32) -> u32 {
    unsafe {        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        const SCHEMA_ID: u32 = 0x66;
        const SCHEMA_CALLEE: u32 = 1;
        const NOT_FOUND: u32 = 0xffff_ffff;

        // Scratch row the schema callee fills: value table at +0x10.
        let mut row = [0u32; 8];
        row[4] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(SCHEMA_CALLEE, u32, SCHEMA_ID, row.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return NOT_FOUND;
        }
        let table = row[4];
        rd32(table.wrapping_add(index.wrapping_mul(4)))
    }
});
