// original: 0x0055c000 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_51, player_schema::LeaderboardInfo, 10>::vf7
/// Read one leaderboard column entry by index.
///
/// Fetches this race's schema (id `RACE_ID`) through the schema callee, which
/// fills a frame slot with the column list. The argument indexes the list with
/// no bounds check; a failed fetch returns -1.
///
/// Original: 0x0055c000 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_0055c000(index: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        const RACE_ID: u32 = 0xfe;
        const SCHEMA_CALLEE: u32 = 0;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut frame = [0u32; 5];
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(SCHEMA_CALLEE, u32, RACE_ID, frame.as_mut_ptr() as u32);
        if (ok & 0xff) == 0 {
            return NOT_FOUND;
        }
        rd32(frame[4].wrapping_add(index.wrapping_mul(4)))
    }
});
