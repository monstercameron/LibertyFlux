// original: 0x0054f4a0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_5, player_schema::LeaderboardInfo, 10>::vf13
/// Map one leaderboard key to its value: search the key array, index the
/// value array with the position found.
///
/// Asks the table-fill callee for leaderboard `LEADERBOARD_ID`, which writes
/// an info block: `COUNT` at `+0x0C`, `KEYS` at `+0x10`, `VALUES` at `+0x14`.
/// Returns -1 when the fill failed or the count is zero or negative (signed).
/// Otherwise scans `KEYS[0..COUNT]` (signed bound) for `want` and returns
/// `VALUES[i]` at the first match, or -1 when absent.
///
/// Original: one stack word (`want`), callee pops 4; incoming `ecx` is
/// overwritten before use, so the call behaves as stdcall. The frame is
/// 8-byte aligned on entry; only the net stack adjustment is observable.
lf_checker_rt::export!(stdcall, rw_0054f4a0(want: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        const LEADERBOARD_ID: u32 = 0xC2;
        const FILL_CALLEE: u32 = 1;
        const COUNT: usize = 3;
        const KEYS: usize = 4;
        const VALUES: usize = 5;
        const MISSING: u32 = 0xFFFF_FFFF;
        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FILL_CALLEE, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return MISSING;
        }
        let count = info[COUNT] as i32;
        if count <= 0 {
            return MISSING;
        }
        let keys = info[KEYS];
        let mut i = 0i32;
        while i < count {
            if rd32(keys.wrapping_add((i as u32).wrapping_mul(4))) == want {
                return rd32(info[VALUES].wrapping_add((i as u32).wrapping_mul(4)));
            }
            i += 1;
        }
        MISSING
    }
});
