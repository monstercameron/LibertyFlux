// original: 0x00519120 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race9NoHolds, player_schema::LeaderboardInfo, 10>::vf6

/// Search this leaderboard's key table for a key, returning its index.
///
/// `key` is the value to find. A scratch row is handed to the schema
/// callee (id `0x66`) which fills in the entry count (row word at
/// `+0x0c`) and the key-table pointer (row word at `+0x10`); a zero
/// answer means no table and yields `NOT_FOUND`. The table is scanned
/// from index 0 while the signed index stays below the signed count.
///
/// Edge cases: a non-positive count finds nothing; a key absent from
/// the table yields `NOT_FOUND` (`0xffff_ffff`).
///
/// Original: 0x00519120 (stdcall, one stack word; incoming registers ignored).
lf_checker_rt::export!(stdcall, rw_00519120(key: u32) -> u32 {
    unsafe {        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        const SCHEMA_ID: u32 = 0x66;
        const SCHEMA_CALLEE: u32 = 1;
        const NOT_FOUND: u32 = 0xffff_ffff;

        // Scratch row the schema callee fills: count at +0x0c, table at +0x10.
        let mut row = [0u32; 8];
        row[3] = 0;
        row[4] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(SCHEMA_CALLEE, u32, SCHEMA_ID, row.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return NOT_FOUND;
        }
        let count = row[3] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let table = row[4];
        let mut i: i32 = 0;
        while i < count {
            if rd32(table.wrapping_add((i as u32).wrapping_mul(4))) == key {
                return i as u32;
            }
            i += 1;
        }
        NOT_FOUND
    }
});
