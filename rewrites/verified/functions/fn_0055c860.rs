// original: 0x0055c860 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_53, player_schema::LeaderboardInfo, 10>::vf6
/// Find a leaderboard key's position in the key list.
///
/// Fetches this race's schema (id `RACE_ID`) through the schema callee, which
/// fills a frame struct with the entry count and the key list. The argument is
/// searched for in the key list; a failed fetch, a non-positive count, or no
/// match returns -1, else the match index. The count comparison is signed.
///
/// Original: 0x0055c860 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_0055c860(key: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        const RACE_ID: u32 = 0x100;
        const SCHEMA_CALLEE: u32 = 0;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut frame = [0u32; 5];
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(SCHEMA_CALLEE, u32, RACE_ID, frame.as_mut_ptr() as u32);
        if (ok & 0xff) == 0 {
            return NOT_FOUND;
        }
        let count = frame[3] as i32;
        let keys = frame[4];
        if count <= 0 {
            return NOT_FOUND;
        }
        let mut i = 0i32;
        while i < count {
            if rd32(keys.wrapping_add((i as u32).wrapping_mul(4))) == key {
                return i as u32;
            }
            i += 1;
        }
        NOT_FOUND
    }
});
