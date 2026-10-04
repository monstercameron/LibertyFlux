// original: 0x00577BB0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_153, player_schema::LeaderboardInfo, 10>::vf13

/// Find a leaderboard row by key, then map it through the id table.
///
/// `key` is searched for (unsigned word comparison, signed loop bound) in
/// the row array the lookup callee returns; on a match at position `i` the
/// function returns entry `i` of the callee's id table, else -1 (as u32).
/// A non-positive count returns -1 without reading either array.
///
/// Layout read (words into the out-block): count at +3, row array at +4,
/// id table at +5. The table is read only after a match, so a null table
/// faults only on the found path.
///
/// Original: 0x00577BB0 (declared stdcall/1; incoming ECX ignored; schema id
/// 0x171).
lf_checker_rt::export!(stdcall, rw_00577BB0(key: u32) -> u32 {
    unsafe {        const SCHEMA_ID: u32 = {sid:#X};
        const LOOKUP: u32 = 1;
        const MISSING: u32 = 0xFFFF_FFFF;
        const COUNT_SLOT: usize = 3;
        const ROWS_SLOT: usize = 4;
        const TABLE_SLOT: usize = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe {{ (a as *const u32).read_unaligned() }}
        }

        let mut out = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(LOOKUP, u32, SCHEMA_ID, out.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {{
            return MISSING;
        }}
        let n = out[COUNT_SLOT] as i32;
        if n <= 0 {{
            return MISSING;
        }}
        let rows = out[ROWS_SLOT];
        let mut i = 0i32;
        while i < n {{
            if rd32(rows.wrapping_add((i as u32).wrapping_mul(4))) == key {{
                let table = out[TABLE_SLOT];
                return {ret};
            }}
            i += 1;
        }}
        MISSING

    }
});
