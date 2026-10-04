// original: 0x00550410 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_8, player_schema::LeaderboardInfo, 10>::vf7
/// Read one slot of a leaderboard value array.
///
/// Asks the table-fill callee for leaderboard `LEADERBOARD_ID`, which writes
/// an info block holding the `VALUES` array pointer at `+0x10`. Returns -1
/// when the fill failed, else `VALUES[idx]`.
///
/// Original: one stack word (`idx`), callee pops 4; incoming `ecx` is
/// overwritten before use, so the call behaves as stdcall.
lf_checker_rt::export!(stdcall, rw_00550410(idx: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        const LEADERBOARD_ID: u32 = 0xC5;
        const FILL_CALLEE: u32 = 1;
        const VALUES: usize = 4;
        const MISSING: u32 = 0xFFFF_FFFF;
        let mut info = [0u32; 5];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FILL_CALLEE, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return MISSING;
        }
        rd32(info[VALUES].wrapping_add(idx.wrapping_mul(4)))
    }
});
