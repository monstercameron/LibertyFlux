// original: 0x0057a560 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_162, player_schema::LeaderboardInfo, 10>::vf7
/// Cell load: one entry of this race's data array.
///
/// Asks the leaderboard-data callee (contract callee 1) for this race's
/// tables, passing LEADERBOARD_ID and a scratch record the callee fills with
/// the data array base (+16). Only the low byte of the callee's answer is
/// significant: zero means no data, answered with -1. Otherwise the word at
/// BASE[index] is returned. There is no bounds check: a wild index faults on
/// the read.
///
/// Original: thiscall, object in ECX (ignored), one stack word.
lf_checker_rt::export!(thiscall, rw_0057a560(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x17a;
        const CALLEE_DATA: u32 = 1;
        const NONE: u32 = 0xFFFF_FFFF;

        #[inline(always)]
        unsafe fn rd32(addr: u32) -> u32 {
            unsafe { (addr as *const u32).read_unaligned() }
        }

        /// Scratch record the data callee fills: the data array base at the
        /// offset the original's frame used.
        #[repr(C)]
        struct Tables {
            _pad: [u32; 4],
            base: u32,
        }

        let mut tables = Tables { _pad: [0; 4], base: 0 };
        let answer: u32 = lf_checker_rt::callee_fastcall!(
            CALLEE_DATA, u32, LEADERBOARD_ID, &mut tables as *mut Tables as u32);
        // Only the low byte is the callee's boolean; the upper bytes are
        // whatever the caller left in the register on each side.
        if (answer & 0xFF) == 0 {
            return NONE;
        }
        rd32(tables.base.wrapping_add(index.wrapping_mul(4)))
    }
});
