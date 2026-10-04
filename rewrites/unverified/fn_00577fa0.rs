// original: 0x00577FA0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_154, player_schema::LeaderboardInfo, 10>::vf12

/// Look up a leaderboard row id, then find which row holds it.
///
/// `index` selects an entry of the id table the lookup callee returns; the
/// function then linearly searches the callee's row array for that id and
/// returns its position, or -1 (as u32) when the lookup fails, the table
/// entry is -1, there are no rows, or no row matches.
///
/// Layout read (offsets into the callee's out-block, words): count at +1,
/// row-array pointer at +2, id-table pointer at +5. The original reads the
/// table entry before testing the count, so a null table faults even when
/// the count is zero; the rewrite keeps that order.
///
/// Original: 0x00577FA0 (declared stdcall/1: the incoming ECX is overwritten
/// before any use, so the implicit this pointer is not modeled; schema id
/// 0x172).
lf_checker_rt::export!(stdcall, rw_00577FA0(index: u32) -> u32 {
    unsafe {        const SCHEMA_ID: u32 = {sid:#X};
        const LOOKUP: u32 = 1;
        const MISSING: u32 = 0xFFFF_FFFF;
        const COUNT_SLOT: usize = 1;
        const ROWS_SLOT: usize = 2;
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
        // Table entry first: a null table faults before the count test.
        let table = out[TABLE_SLOT];
        let key = rd32(table.wrapping_add(index.wrapping_mul(4)));
        if key == MISSING {{
            return MISSING;
        }}
        let count = out[COUNT_SLOT];
        if count == 0 {{
            return MISSING;
        }}
        let rows = out[ROWS_SLOT];
        let mut i = 0u32;
        while i < count {{
            if rd32(rows.wrapping_add(i.wrapping_mul(4))) == key {{
                return {ret};
            }}
            i = i.wrapping_add(1);
        }}
        MISSING

    }
});
