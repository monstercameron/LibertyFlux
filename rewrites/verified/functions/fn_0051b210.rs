// original: 0x0051B210 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race17NoHolds, player_schema::LeaderboardInfo, 10>::vf13

/// Look a key up in this leaderboard's key table and return the value
/// stored for it in the parallel value table.
///
/// `key` is the value to find. A scratch row is handed to the schema
/// callee (id `0x6e`) which fills in the entry count (row word at
/// `+0x0c`), the key-table pointer (row word at `+0x10`) and the
/// value-table pointer (row word at `+0x14`); a zero answer means no
/// table and yields `NOT_FOUND`. The key table is scanned from index 0
/// while the signed index stays below the signed count, and the value
/// at the matching index is returned.
///
/// Edge cases: a non-positive count finds nothing; a key absent from
/// the table yields `NOT_FOUND` (`0xffff_ffff`).
///
/// Original: 0x0051B210 (stdcall, one stack word; incoming registers ignored).
lf_checker_rt::export!(stdcall, rw_0051b210(key: u32) -> u32 {
    unsafe {        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        const SCHEMA_ID: u32 = 0x6e;
        const SCHEMA_CALLEE: u32 = 1;
        const NOT_FOUND: u32 = 0xffff_ffff;

        // Scratch row: count at +0x0c, key table at +0x10, value table at +0x14.
        let mut row = [0u32; 8];
        row[3] = 0;
        row[4] = 0;
        row[5] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(SCHEMA_CALLEE, u32, SCHEMA_ID, row.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return NOT_FOUND;
        }
        let count = row[3] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let keys = row[4];
        let values = row[5];
        let mut i: i32 = 0;
        while i < count {
            if rd32(keys.wrapping_add((i as u32).wrapping_mul(4))) == key {
                break;
            }
            i += 1;
        }
        if i >= count {
            return NOT_FOUND;
        }
        let idx = i as u32;
        if idx == NOT_FOUND {
            return NOT_FOUND;
        }
        rd32(values.wrapping_add(idx.wrapping_mul(4)))
    }
});
