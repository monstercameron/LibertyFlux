// original: 0x0057E910 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_178, player_schema::LeaderboardInfo, 10>::vf13
/// Map a ranked value to its payload: search the key list, return the paired slot.
///
/// Calls the leaderboard lookup callee (fastcall: `LEADERBOARD_ID` in ECX, a
/// caller-owned out-struct in EDX) which reports success in AL and fills the
/// struct: entry count at `+0x0c`, key-array pointer at `+0x10`, value-array
/// pointer at `+0x14`. Returns -1 when the lookup fails or the count is not
/// positive (signed). Otherwise linearly scans the key array for `wanted`
/// (signed bound) and returns the value-array word at the hit position, or
/// -1 on a miss. (The original re-tests the hit index against -1 before the
/// load; the index is always >= 0 there, so the check never fires.)
///
/// Original: 0x0057E910 (thiscall, one stack word; ECX is ignored).
lf_checker_rt::export!(thiscall, rw_0057E910(_this: u32, wanted: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x18a;
        const LOOKUP_CALLEE: u32 = 1;
        const OUT_COUNT: usize = 3;
        const OUT_KEYS: usize = 4;
        const OUT_VALUES: usize = 5;
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
        let count = out[OUT_COUNT];
        if (count as i32) <= 0 {
            return MISSING;
        }
        let keys = out[OUT_KEYS];
        let mut i = 0u32;
        while (i as i32) < (count as i32) {
            if rd32(keys.wrapping_add(i.wrapping_mul(4))) == wanted {
                let values = out[OUT_VALUES];
                return rd32(values.wrapping_add(i.wrapping_mul(4)));
            }
            i = i.wrapping_add(1);
        }
        MISSING
    }
});
