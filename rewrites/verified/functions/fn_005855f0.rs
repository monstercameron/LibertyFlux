// original: 0x005855f0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_203, player_schema::LeaderboardInfo, 10>::vf12

/// Find the row whose key equals the value looked up by index.
///
/// `index` is a row number into the value column. The fetch callee (id 1) is
/// called with this instantiation's leaderboard id and a pointer to an
/// eight-word stack buffer; on success it leaves a row count at `+0x04`, the
/// key column pointer at `+0x08` and the value column pointer at `+0x14`.
/// Only the low byte of the fetch result is significant: zero means failure
/// and yields -1. There is no bounds check on the index.
///
/// The value at `values + index * 4` is read first: -1 there yields -1
/// without touching the count. A zero count then yields -1 (only an exact
/// zero; the loop bound below is unsigned). Otherwise the keys are scanned in
/// order and the index of the first row equal to the value is returned, or -1
/// when no row matches.
///
/// Original: 0x005855f0 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_005855f0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1a3;
        const FETCH_CALLEE: u32 = 1;
        const COUNT_WORD: usize = 1;
        const KEYS_WORD: usize = 2;
        const VALS_WORD: usize = 5;
        const NOT_FOUND: u32 = 0xffff_ffff;
        #[inline(always)]
        unsafe fn read_u32(addr: u32) -> u32 {
            unsafe { (addr as *const u32).read_unaligned() }
        }
        let mut out = [0u32; 8];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return NOT_FOUND;
        }
        let vals = out[VALS_WORD];
        let want = read_u32(vals.wrapping_add(index.wrapping_mul(4)));
        if want == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = out[COUNT_WORD];
        if count == 0 {
            return NOT_FOUND;
        }
        let keys = out[KEYS_WORD];
        let mut i = 0u32;
        loop {
            if read_u32(keys.wrapping_add(i.wrapping_mul(4))) == want {
                return i;
            }
            i = i.wrapping_add(1);
            if !(i < count) {
                return NOT_FOUND;
            }
        }
    }
});
