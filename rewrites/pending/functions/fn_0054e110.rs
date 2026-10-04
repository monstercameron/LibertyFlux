// original: 0x0054e110 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_0, player_schema::LeaderboardInfo, 10>::vf7

/// Return the leaderboard table entry at `index`.
///
/// Calls the info-fetch callee (fastcall: ECX = schema id 0xaf, EDX = info
/// buffer) which reports success in AL and fills the buffer: table pointer
/// at `+0x10`. On success returns `table[index]`; when the callee reports
/// failure returns `NOT_FOUND` (0xffffffff). The index is used as-is with
/// no bounds check.
///
/// Original: stdcall, one stack word; incoming ECX is unused (overwritten
/// with the schema id before the call).
lf_checker_rt::export!(stdcall, rw_0054e110(index: u32) -> u32 {
    unsafe {
        const SCHEMA_ID: u32 = 0xaf;
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
        let table = info[(TABLE_OFF / 4) as usize];
        rd32(table.wrapping_add(index.wrapping_mul(4)))
    }
});
