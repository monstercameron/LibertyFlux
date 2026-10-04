// original: 0x0054d7f0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_25, player_schema::LeaderboardInfo, 10>::vf6

/// Find `value` in the leaderboard id table and return its position.
///
/// Calls the info-fetch callee (fastcall: ECX = schema id 0xd0, EDX = info
/// buffer) which reports success in AL and fills the buffer: entry count at
/// `+0x0c`, id-table pointer at `+0x10`. On success with a positive count,
/// linearly scans the table for `value` and returns the first matching
/// index. Returns `NOT_FOUND` (0xffffffff) when the callee reports failure,
/// when the count is zero or negative (signed), or when no entry matches.
///
/// Original: stdcall, one stack word; incoming ECX is unused (overwritten
/// with the schema id before the call).
lf_checker_rt::export!(stdcall, rw_0054d7f0(value: u32) -> u32 {
    unsafe {
        const SCHEMA_ID: u32 = 0xd0;
        const COUNT_OFF: u32 = 0x0c;
        const TABLE_OFF: u32 = 0x10;
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
        let table = info[(TABLE_OFF / 4) as usize];
        let mut i = 0i32;
        while i < count {
            if rd32(table.wrapping_add((i as u32).wrapping_mul(4))) == value {
                return i as u32;
            }
            i += 1;
        }
        NOT_FOUND
    }
});
