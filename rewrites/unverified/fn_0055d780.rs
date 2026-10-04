// original: 0x0055d780 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_57, player_schema::LeaderboardInfo, 10>::vf12
/// Find a leaderboard column value's position in the key list.
///
/// Fetches this race's schema (id `RACE_ID`) through the schema callee, which
/// fills a frame struct with the key count, the key list and the value list.
/// The argument selects a value by index; an absent (-1) value there, a failed
/// fetch, an empty key list, or no match all return -1, else the match index.
/// The count comparison is unsigned.
///
/// Original: 0x0055d780 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_0055d780(index: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        const RACE_ID: u32 = 0x104;
        const SCHEMA_CALLEE: u32 = 0;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut frame = [0u32; 6];
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(SCHEMA_CALLEE, u32, RACE_ID, frame.as_mut_ptr() as u32);
        if (ok & 0xff) == 0 {
            return NOT_FOUND;
        }
        let count = frame[1];
        let keys = frame[2];
        let values = frame[5];
        let want = rd32(values.wrapping_add(index.wrapping_mul(4)));
        if want == NOT_FOUND {
            return NOT_FOUND;
        }
        if count == 0 {
            return NOT_FOUND;
        }
        let mut i = 0u32;
        while i < count {
            if rd32(keys.wrapping_add(i.wrapping_mul(4))) == want {
                return i;
            }
            i += 1;
        }
        NOT_FOUND
    }
});
