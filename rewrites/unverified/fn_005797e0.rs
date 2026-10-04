// original: 0x005797E0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_159, player_schema::LeaderboardInfo, 10>::vf6

/// Find a leaderboard row by key and return its position.
///
/// `key` is searched for in the row array the lookup callee returns
/// (unsigned word comparison, signed loop bound); the function returns the
/// matching position, or -1 (as u32) when the lookup fails, the count is
/// not positive, or no row matches. Same search as vf13 without the id
/// table step.
///
/// Layout read (words into the out-block): count at +3, row array at +4.
///
/// Original: 0x005797E0 (declared stdcall/1; incoming ECX ignored; schema id
/// 0x177).
lf_checker_rt::export!(stdcall, rw_005797E0(key: u32) -> u32 {
    unsafe {        const SCHEMA_ID: u32 = {sid:#X};
        const LOOKUP: u32 = 1;
        const MISSING: u32 = 0xFFFF_FFFF;
        const COUNT_SLOT: usize = 3;
        const ROWS_SLOT: usize = 4;

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
                return {ret};
            }}
            i += 1;
        }}
        MISSING

    }
});
