// original: 0x0057EB60 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_178, player_schema::LeaderboardInfo, 10>::vf7
/// Load one ranked slot by index.
///
/// Calls the leaderboard lookup callee (fastcall: `LEADERBOARD_ID` in ECX, a
/// caller-owned out-struct in EDX) which reports success in AL and fills the
/// struct's slot-array pointer at `+0x10`. Returns -1 when the lookup fails,
/// otherwise the slot-array word at `index` (which itself may be -1).
///
/// Original: 0x0057EB60 (thiscall, one stack word; ECX is ignored).
lf_checker_rt::export!(thiscall, rw_0057EB60(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x18a;
        const LOOKUP_CALLEE: u32 = 1;
        const OUT_SLOTS: usize = 4;
        const MISSING: u32 = 0xFFFF_FFFF;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut out = [0u32; 6];
        let ok = lf_checker_rt::callee_fastcall!(
            LOOKUP_CALLEE, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32
        );
        if ok & 0xFF == 0 {
            return MISSING;
        }
        let slots = out[OUT_SLOTS];
        rd32(slots.wrapping_add(index.wrapping_mul(4)))
    }
});
