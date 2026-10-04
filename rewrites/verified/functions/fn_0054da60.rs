// original: 0x0054da60 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_26, player_schema::LeaderboardInfo, 10>::vf13

/// Find `value` in the leaderboard id table and return the mapped result.
///
/// Calls the info-fetch callee (fastcall: ECX = schema id 0xb6, EDX = info
/// buffer) which reports success in AL and fills the buffer: entry count at
/// `+0x0c`, id-table pointer at `+0x10`, result-table pointer at `+0x14`. On
/// success with a positive count, linearly scans the id table for `value`
/// and returns the result-table entry at the matching index. Returns
/// `NOT_FOUND` (0xffffffff) when the callee reports failure, when the count
/// is zero or negative (signed), or when no entry matches. (The original
/// re-checks the found index against -1 before the lookup; the index comes
/// from a loop over `0..count` so that check cannot fail and is omitted.)
///
/// Original: stdcall, one stack word; incoming ECX is unused (overwritten
/// with the schema id before the call).
lf_checker_rt::export!(stdcall, rw_0054da60(value: u32) -> u32 {
    unsafe {
        const SCHEMA_ID: u32 = 0xb6;
        const COUNT_OFF: u32 = 0x0c;
        const IDS_OFF: u32 = 0x10;
        const RESULTS_OFF: u32 = 0x14;
        const NOT_FOUND: u32 = 0xffff_ffff;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut info = [0u32; 8];
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(0, u32, SCHEMA_ID, info.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return NOT_FOUND;
        }
        let count = info[(COUNT_OFF / 4) as usize] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let ids = info[(IDS_OFF / 4) as usize];
        let mut i = 0i32;
        while i < count {
            if rd32(ids.wrapping_add((i as u32).wrapping_mul(4))) == value {
                let results = info[(RESULTS_OFF / 4) as usize];
                return rd32(results.wrapping_add((i as u32).wrapping_mul(4)));
            }
            i += 1;
        }
        NOT_FOUND
    }
});
