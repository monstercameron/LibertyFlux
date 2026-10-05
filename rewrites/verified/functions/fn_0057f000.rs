// original: 0x0057F000 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_179, player_schema::LeaderboardInfo, 10>::vf8
/// Classify a ranked entry: fetch slot `index`, probe its category, map it.
///
/// Calls the leaderboard lookup callee (fastcall: `LEADERBOARD_ID` in ECX, a
/// caller-owned out-struct in EDX) which reports success in AL and fills the
/// struct's slot-array pointer at `+0x14`. A failed lookup returns 0, as
/// does a probe answer of -1 or anything outside 1..=5: all of those paths
/// share the table's fourth case body. Otherwise the slot word is passed (in
/// ECX) to the category probe callee and its answer minus one indexes a
/// five-way table: 1 maps to 4, 2 and 3 to 8, 4 to 0, 5 to 4.
///
/// Original: 0x0057F000 (thiscall, one stack word; ECX is ignored).
lf_checker_rt::export!(thiscall, rw_0057F000(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x18b;
        const LOOKUP_CALLEE: u32 = 1;
        const CATEGORY_CALLEE: u32 = 2;
        const OUT_SLOTS: usize = 5;
        const ZERO: u32 = 0;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut out = [0u32; 8];
        let ok = lf_checker_rt::callee_fastcall!(
            LOOKUP_CALLEE, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32
        );
        if ok & 0xFF == 0 {
            return ZERO;
        }
        let slots = out[OUT_SLOTS];
        let entry = rd32(slots.wrapping_add(index.wrapping_mul(4)));
        let cat: u32 = lf_checker_rt::callee_thiscall!(CATEGORY_CALLEE, u32, entry);
        if cat == 0xFFFF_FFFF {
            return ZERO;
        }
        let w = cat.wrapping_sub(1);
        if w > 4 {
            return ZERO;
        }
        match w {
            0 => 4,
            1 => 8,
            2 => 8,
            3 => 0,
            _ => 4,
        }
    }
});
