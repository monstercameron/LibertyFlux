// original: 0x00564080 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_81, player_schema::LeaderboardInfo, 10>::vf12
/// Look up leaderboard column 0x11c, take element `index` of its key array,
/// and return the position of that key in the id array, or -1.
///
/// Calls the shared column lookup with id `LOOKUP_ID`, passing a six-word
/// out-struct by frame pointer (words 1, 2 and 5 receive the unsigned
/// `count`, the id-array pointer and the key-array pointer). A false
/// return, a key of -1, or a zero count all mean "not found". Otherwise the
/// first `count` ids are scanned for the key (compared unsigned) and the
/// first matching index is returned, or -1.
///
/// The index is used unchecked, in the original's order: the key is loaded
/// before the count is tested, so a wild index faults first.
///
/// Original: stdcall of one stack word (the index); incoming ECX ignored.
lf_checker_rt::export!(stdcall, rw_00564080(index: u32) -> u32 {
    unsafe {
        /// Column id selected by this instantiation.
        const LOOKUP_ID: u32 = 0x11c;
        const NONE: u32 = 0xFFFF_FFFF;
        const OUT_COUNT: usize = 1;
        const OUT_IDS: usize = 2;
        const OUT_KEYS: usize = 5;
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
        let wanted = rd32(out[OUT_KEYS].wrapping_add(index.wrapping_mul(4)));
        if wanted == NONE {
            return NONE;
        }
        let count = out[OUT_COUNT];
        if count == 0 {
            return NONE;
        }
        let ids = out[OUT_IDS];
        let mut i = 0u32;
        while i < count {
            if rd32(ids.wrapping_add(i.wrapping_mul(4))) == wanted {
                return i;
            }
            i = i.wrapping_add(1);
        }
        NONE
    }
});
