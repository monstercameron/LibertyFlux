// original: 0x0054f890 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_6, player_schema::LeaderboardInfo, 10>::vf12
/// Find the rank slot of one entry: resolve the entry's key through the
/// leaderboard tables, then search the rank array for it.
///
/// Asks the table-fill callee for leaderboard `LEADERBOARD_ID`, which writes
/// an info block: `COUNT` at `+0x04`, `RANKS` (key array) at `+0x08`,
/// `KEYS` (per-slot key array) at `+0x14`. Reads `KEYS[idx]`; when that key
/// is -1, or the fill failed, or the count is zero, returns -1. Otherwise
/// scans `RANKS[0..COUNT]` (unsigned bound) and returns the first index whose
/// key matches, or -1 when absent.
///
/// Original: one stack word (`idx`), callee pops 4; the incoming `ecx` is
/// overwritten before use, so the call behaves as stdcall.
lf_checker_rt::export!(stdcall, rw_0054f890(idx: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        const LEADERBOARD_ID: u32 = 0xC3;
        const FILL_CALLEE: u32 = 1;
        const COUNT: usize = 1;
        const RANKS: usize = 2;
        const KEYS: usize = 5;
        const MISSING: u32 = 0xFFFF_FFFF;
        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FILL_CALLEE, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return MISSING;
        }
        let target = rd32(info[KEYS].wrapping_add(idx.wrapping_mul(4)));
        if target == MISSING {
            return MISSING;
        }
        let count = info[COUNT];
        if count == 0 {
            return MISSING;
        }
        let ranks = info[RANKS];
        let mut i = 0u32;
        while i < count {
            if rd32(ranks.wrapping_add(i.wrapping_mul(4))) == target {
                return i;
            }
            i += 1;
        }
        MISSING
    }
});
