// original: 0x00584080 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_198, player_schema::LeaderboardInfo, 10>::vf13

/// Look up one leaderboard row id and return its value column entry.
///
/// `needle` is the row id to find. The fetch callee (id 1) is called with
/// this instantiation's leaderboard id and a pointer to an eight-word stack
/// buffer; on success it leaves a signed row count at `+0x0c`, the key column
/// pointer at `+0x10` and the value column pointer at `+0x14`. Only the low
/// byte of the fetch result is significant: zero means failure and yields -1.
///
/// The keys are scanned in order (signed `i32` loop bound, like vf6) and the
/// value at the first matching row is returned, or -1 when the count is not
/// positive or no row matches. The original re-checks the found index against
/// -1 after the search; that branch is dead (a found index is never
/// negative) and is not reproduced.
///
/// Original: 0x00584080 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00584080(needle: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x19e;
        const FETCH_CALLEE: u32 = 1;
        const COUNT_WORD: usize = 3;
        const KEYS_WORD: usize = 4;
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
        let count = out[COUNT_WORD] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let keys = out[KEYS_WORD];
        let mut i = 0i32;
        while i < count {
            if read_u32(keys.wrapping_add((i as u32).wrapping_mul(4))) == needle {
                let vals = out[VALS_WORD];
                return read_u32(vals.wrapping_add((i as u32).wrapping_mul(4)));
            }
            i += 1;
        }
        NOT_FOUND
    }
});
