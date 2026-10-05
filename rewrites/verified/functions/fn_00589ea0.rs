// original: 0x00589ea0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_219, player_schema::LeaderboardInfo, 10>::vf7

/// Column-handle fetch for one ranked episodic-race leaderboard.
///
/// Asks the leaderboard helper (callee 1) for the column list of leaderboard
/// id 0x1b3, then returns element `index` of the returned array, or -1 when
/// the helper reports failure. There is no bounds check: an out-of-range
/// index reads past the array (or faults), exactly like the original.
///
/// The helper takes the leaderboard id in ECX and an out-buffer in EDX and
/// answers in AL; on success it fills the array pointer at buffer byte 16.
/// This method ignores its `this` pointer and takes one stack argument
/// (`index`). Original is stdcall (the callee pops 4 bytes).

lf_checker_rt::export!(stdcall, rw_00589ea0(index: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        const LEADERBOARD_ID: u32 = 0x1b3;
        const HELPER: u32 = 1;
        const ARRAY_WORD: usize = 4;
        const FAILED: u32 = 0xffff_ffff;

        let mut info = [0u32; 6];
        info[ARRAY_WORD] = 0;
        let ok: u8 = lf_checker_rt::callee_fastcall!(HELPER, u8, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if ok == 0 {
            return FAILED;
        }
        let cell = info[ARRAY_WORD].wrapping_add(index.wrapping_mul(4));
        rd32(cell)
    }
});
