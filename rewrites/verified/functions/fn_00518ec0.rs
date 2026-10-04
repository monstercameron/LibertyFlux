// original: 0x00518EC0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race9NoHolds, player_schema::LeaderboardInfo, 10>::vf12

/// Map a row index through this leaderboard's value table, then find the
/// resulting key's position in the key table.
///
/// `index` selects an entry of the value table. A scratch row is handed
/// to the schema callee (id `0x66`) which fills in the key count (row
/// word at `+0x04`), the key-table pointer (row word at `+0x08`) and
/// the value-table pointer (row word at `+0x14`); a zero answer means
/// no table and yields `NOT_FOUND`. The key is the indexed value-table
/// entry; the key table is then scanned from 0 while the unsigned
/// index stays below the count.
///
/// Edge cases: a looked-up key of `NOT_FOUND` or a zero count finds
/// nothing; an absent key yields `NOT_FOUND` (`0xffff_ffff`).
///
/// Original: 0x00518EC0 (stdcall, one stack word; incoming registers ignored).
lf_checker_rt::export!(stdcall, rw_00518ec0(index: u32) -> u32 {
    unsafe {        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        const SCHEMA_ID: u32 = 0x66;
        const SCHEMA_CALLEE: u32 = 1;
        const NOT_FOUND: u32 = 0xffff_ffff;

        // Scratch row: count at +0x04, key table at +0x08, value table at +0x14.
        let mut row = [0u32; 8];
        row[1] = 0;
        row[2] = 0;
        row[5] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(SCHEMA_CALLEE, u32, SCHEMA_ID, row.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return NOT_FOUND;
        }
        let values = row[5];
        let want = rd32(values.wrapping_add(index.wrapping_mul(4)));
        if want == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = row[1];
        if count == 0 {
            return NOT_FOUND;
        }
        let keys = row[2];
        let mut i: u32 = 0;
        while i < count {
            if rd32(keys.wrapping_add(i.wrapping_mul(4))) == want {
                return i;
            }
            i += 1;
        }
        NOT_FOUND
    }
});
