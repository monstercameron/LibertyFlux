// original: 0x0057F4D0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_180, player_schema::LeaderboardInfo, 10>::vf9
/// Rank of a ranked entry: fetch slot `index`, probe it, map the answer.
///
/// Calls the leaderboard lookup callee (fastcall: `LEADERBOARD_ID` in ECX, a
/// caller-owned out-struct in EDX) which reports success in AL and fills the
/// struct's slot-array pointer at `+0x14`. Returns -1 when the lookup fails.
/// Otherwise the slot word is passed (in ECX) to the probe callee; its
/// answer minus one indexes a five-way table: 1 maps to 0, 2 to 1, 3 to 3,
/// 4 to -1, 5 to 2. Any other answer, including -1, returns -1.
///
/// Original: 0x0057F4D0 (thiscall, one stack word; ECX is ignored).
lf_checker_rt::export!(thiscall, rw_0057F4D0(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x18c;
        const LOOKUP_CALLEE: u32 = 1;
        const PROBE_CALLEE: u32 = 2;
        const OUT_SLOTS: usize = 5;
        const MISSING: u32 = 0xFFFF_FFFF;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut out = [0u32; 8];
        let ok = lf_checker_rt::callee_fastcall!(
            LOOKUP_CALLEE, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32
        );
        if ok & 0xFF == 0 {
            return MISSING;
        }
        let slots = out[OUT_SLOTS];
        let entry = rd32(slots.wrapping_add(index.wrapping_mul(4)));
        let cat: u32 = lf_checker_rt::callee_thiscall!(PROBE_CALLEE, u32, entry);
        if cat == MISSING {
            return MISSING;
        }
        let w = cat.wrapping_sub(1);
        if w > 4 {
            return MISSING;
        }
        match w {
            0 => 0,
            1 => 1,
            2 => 3,
            3 => MISSING,
            _ => 2,
        }
    }
});
