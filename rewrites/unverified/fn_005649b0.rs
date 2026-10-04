// original: 0x005649b0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_83, player_schema::LeaderboardInfo, 10>::vf13
/// Look up leaderboard column 0x11e and map `key` through its id array to
/// the value array, returning the mapped value or -1.
///
/// Calls the shared column lookup with id `LOOKUP_ID`, passing a six-word
/// out-struct by frame pointer (words 3, 4 and 5 receive the signed
/// `count`, the id-array pointer and the value-array pointer). A false
/// return, or a count of zero or less, means "not found". Otherwise the
/// first `count` ids are scanned for `key` and the value at the first
/// matching position is returned, or -1 when no element matches. (The
/// original re-tests the found index against -1, which can never hit; that
/// dead check is not reproduced.)
///
/// Original: stdcall of one stack word (the sought id); incoming ECX is
/// ignored; callee-saved registers preserved.
lf_checker_rt::export!(stdcall, rw_005649b0(key: u32) -> u32 {
    unsafe {
        /// Column id selected by this instantiation.
        const LOOKUP_ID: u32 = 0x11e;
        const NONE: u32 = 0xFFFF_FFFF;
        const OUT_COUNT: usize = 3;
        const OUT_IDS: usize = 4;
        const OUT_VALUES: usize = 5;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut out = [0u32; 6];
        let answered: u32 =
            lf_checker_rt::callee_fastcall!(1, u32, LOOKUP_ID, out.as_mut_ptr() as u32);
        if answered & 0xFF == 0 {
            return NONE;
        }
        let count = out[OUT_COUNT] as i32;
        if count <= 0 {
            return NONE;
        }
        let ids = out[OUT_IDS];
        let mut i = 0u32;
        while i < count as u32 {
            if rd32(ids.wrapping_add(i.wrapping_mul(4))) == key {
                let values = out[OUT_VALUES];
                return rd32(values.wrapping_add(i.wrapping_mul(4)));
            }
            i = i.wrapping_add(1);
        }
        NONE
    }
});
