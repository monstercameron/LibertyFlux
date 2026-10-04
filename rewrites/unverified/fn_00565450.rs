// original: 0x00565450 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_85, player_schema::LeaderboardInfo, 10>::vf6
/// Look up leaderboard column 0x120 and return the position of `key` in its
/// id array, or -1 when absent.
///
/// Calls the shared column lookup with id `LOOKUP_ID`, passing a five-word
/// out-struct by frame pointer (words 3 and 4 receive the signed `count`
/// and the id-array pointer). A false return, or a count of zero or less,
/// means "not found". Otherwise the first `count` ids are scanned in order
/// and the index of the first element equal to `key` is returned, or -1.
///
/// Original: stdcall of one stack word (the sought id); incoming ECX is
/// ignored; callee-saved registers preserved.
lf_checker_rt::export!(stdcall, rw_00565450(key: u32) -> u32 {
    unsafe {
        /// Column id selected by this instantiation.
        const LOOKUP_ID: u32 = 0x120;
        /// Not found.
        const NONE: u32 = 0xFFFF_FFFF;
        const OUT_COUNT: usize = 3;
        const OUT_IDS: usize = 4;
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
        let count = out[OUT_COUNT] as i32;
        if count <= 0 {
            return NONE;
        }
        let ids = out[OUT_IDS];
        let mut i = 0u32;
        while i < count as u32 {
            if rd32(ids.wrapping_add(i.wrapping_mul(4))) == key {
                return i;
            }
            i = i.wrapping_add(1);
        }
        NONE
    }
});
