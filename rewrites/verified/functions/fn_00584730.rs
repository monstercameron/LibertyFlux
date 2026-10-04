// original: 0x00584730 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_199, player_schema::LeaderboardInfo, 10>::vf7

/// Read one entry of the fetched key column by row index.
///
/// `index` is a row number into the key column. The fetch callee (id 1) is
/// called with this instantiation's leaderboard id and a pointer to an
/// eight-word stack buffer; on success it leaves the key column pointer at
/// `+0x10`. Only the low byte of the fetch result is significant: zero means
/// failure and yields -1.
///
/// There is no bounds check: the word at `keys + index * 4` (32-bit
/// wraparound) is returned as is.
///
/// Original: 0x00584730 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00584730(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x19f;
        const FETCH_CALLEE: u32 = 1;
        const KEYS_WORD: usize = 4;
        const FAILED: u32 = 0xffff_ffff;
        let mut out = [0u32; 8];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return FAILED;
        }
        let keys = out[KEYS_WORD];
        (keys.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned()
    }
});
