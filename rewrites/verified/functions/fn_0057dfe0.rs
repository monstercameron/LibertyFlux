// original: 0x0057DFE0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_176, player_schema::LeaderboardInfo, 10>::vf12
/// Find a ranked entry's position: fetch slot `index`, then search the key list.
///
/// Calls the leaderboard lookup callee (fastcall: `LEADERBOARD_ID` in ECX, a
/// caller-owned out-struct in EDX) which reports success in AL and fills the
/// struct: entry count at `+4`, key-array pointer at `+8`, slot-array pointer
/// at `+0x14`. Returns -1 when the lookup fails, when slot `index` holds -1,
/// or when the count is 0. Otherwise linearly scans the key array (unsigned
/// bound) for the slot's key and returns its position, or -1 on a miss.
///
/// Original: 0x0057DFE0 (thiscall, one stack word; ECX is ignored).
lf_checker_rt::export!(thiscall, rw_0057DFE0(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x188;
        const LOOKUP_CALLEE: u32 = 1;
        const OUT_COUNT: usize = 1;
        const OUT_KEYS: usize = 2;
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
        let key = rd32(slots.wrapping_add(index.wrapping_mul(4)));
        if key == MISSING {
            return MISSING;
        }
        let count = out[OUT_COUNT];
        if count == 0 {
            return MISSING;
        }
        let keys = out[OUT_KEYS];
        let mut i = 0u32;
        while i < count {
            if rd32(keys.wrapping_add(i.wrapping_mul(4))) == key {
                return i;
            }
            i = i.wrapping_add(1);
        }
        MISSING
    }
});
