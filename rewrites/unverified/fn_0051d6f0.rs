// original: 0x0051d6f0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race25NoHolds, player_schema::LeaderboardInfo, 10>::vf6
/// Index of a row key in this leaderboard's key table, or -1.
///
/// Calls the schema lookup (callee 1) with this leaderboard's id; on
/// success info words `+0x0c`/`+0x10` are the key count (signed) and the
/// key table. Returns the first index whose key equals `want`, or -1 when
/// the lookup fails, the count is not positive, or no key matches.
///
/// Arguments: `want`, the key to find. The incoming object pointer is
/// unused. Original: thiscall, one stack word; compare and loop bound are
/// signed, matching the original's conditional jumps.
lf_checker_rt::export!(thiscall, rw_0051d6f0(_this: u32, want: u32) -> u32 {
    unsafe {
        const SCHEMA_ID: u32 = 0xe;
        const LOOKUP_CALLEE: u32 = 1;
        const COUNT_WORD: usize = 3;
        const KEYS_WORD: usize = 4;
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
        let mut i = 0i32;
        while i < count {
            if rd32(keys.wrapping_add((i as u32).wrapping_mul(4))) == want {
                return i as u32;
            }
            i += 1;
        }
        NONE
    }
});
