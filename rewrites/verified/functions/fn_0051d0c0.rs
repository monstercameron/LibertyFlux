// original: 0x0051d0c0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race24NoHolds, player_schema::LeaderboardInfo, 10>::vf13
/// Value paired with a row key in this leaderboard.
///
/// Calls the schema lookup (callee 1) with this leaderboard's id; on
/// success info words `+0x0c`/`+0x10`/`+0x14` are the key count (signed),
/// the key table and the parallel value table. Finds the first index
/// whose key equals `want` and returns the value at the same index, or -1
/// when the lookup fails, the count is not positive, or no key matches.
/// (The original re-checks the found index against -1, which cannot hit
/// since indices start at 0; that dead check is not reproduced.)
///
/// Arguments: `want`, the key to find. The incoming object pointer is
/// unused. Original: thiscall, one stack word; compare and loop bound are
/// signed, matching the original's conditional jumps.
lf_checker_rt::export!(thiscall, rw_0051d0c0(_this: u32, want: u32) -> u32 {
    unsafe {
        const SCHEMA_ID: u32 = 0xd;
        const LOOKUP_CALLEE: u32 = 1;
        const COUNT_WORD: usize = 3;
        const KEYS_WORD: usize = 4;
        const VALUES_WORD: usize = 5;
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
        let count = info[COUNT_WORD] as i32;
        if count <= 0 {
            return NONE;
        }
        let keys = info[KEYS_WORD];
        let values = info[VALUES_WORD];
        let mut i = 0i32;
        while i < count {
            if rd32(keys.wrapping_add((i as u32).wrapping_mul(4))) == want {
                return rd32(values.wrapping_add((i as u32).wrapping_mul(4)));
            }
            i += 1;
        }
        NONE
    }
});
