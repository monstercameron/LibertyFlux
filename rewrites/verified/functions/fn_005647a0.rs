// original: 0x005647a0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_82, player_schema::LeaderboardInfo, 10>::vf7
/// Look up leaderboard column 0x11d and return element `index` of its value
/// array, or -1 when the lookup fails.
///
/// Calls the shared column lookup with id `LOOKUP_ID`, passing a five-word
/// out-struct by frame pointer (word 4 receives the value-array pointer).
/// On success the word at `index` is returned with no bounds check: a wild
/// index faults, exactly like the original.
///
/// Original: stdcall of one stack word (the index); incoming ECX ignored.
lf_checker_rt::export!(stdcall, rw_005647a0(index: u32) -> u32 {
    unsafe {
        /// Column id selected by this instantiation.
        const LOOKUP_ID: u32 = 0x11d;
        const NONE: u32 = 0xFFFF_FFFF;
        const OUT_VALUES: usize = 4;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut out = [0u32; 5];
        let answered: u32 =
            lf_checker_rt::callee_fastcall!(1, u32, LOOKUP_ID, out.as_mut_ptr() as u32);
        if answered & 0xFF == 0 {
            return NONE;
        }
        rd32(out[OUT_VALUES].wrapping_add(index.wrapping_mul(4)))
    }
});
