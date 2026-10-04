// original: 0x0057a5a0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_162, player_schema::LeaderboardInfo, 10>::vf8
/// Column width: storage size of one column of this race's table.
///
/// Asks the leaderboard-data callee (contract callee 1) for this race's
/// tables, passing LEADERBOARD_ID and a scratch record the callee fills with
/// the column array base (+20). Only the low byte of the callee's answer is
/// significant: zero means no data, answered with 0. Otherwise the column id
/// at BASE[index] is classified by the kind callee (contract callee 2); an
/// answer of -1, or a kind whose predecessor falls outside 0..=4, is answered
/// with 0, and the five kinds map to widths 4/8/8/0/4.
///
/// Original: thiscall, object in ECX (ignored), one stack word.
lf_checker_rt::export!(thiscall, rw_0057a5a0(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x17a;
        const CALLEE_DATA: u32 = 1;
        const CALLEE_KIND: u32 = 2;
        const NONE: u32 = 0xFFFF_FFFF;

        #[inline(always)]
        unsafe fn rd32(addr: u32) -> u32 {
            unsafe { (addr as *const u32).read_unaligned() }
        }

        /// Scratch record the data callee fills: the column array base at
        /// the offset the original's frame used.
        #[repr(C)]
        struct Tables {
            _pad: [u32; 5],
            base: u32,
        }

        let mut tables = Tables { _pad: [0; 5], base: 0 };
        let answer: u32 = lf_checker_rt::callee_fastcall!(
            CALLEE_DATA, u32, LEADERBOARD_ID, &mut tables as *mut Tables as u32);
        // Only the low byte is the callee's boolean; the upper bytes are
        // whatever the caller left in the register on each side.
        if (answer & 0xFF) == 0 {
            return 0;
        }
        let cell = rd32(tables.base.wrapping_add(index.wrapping_mul(4)));
        let kind: u32 = lf_checker_rt::callee_thiscall!(CALLEE_KIND, u32, cell);
        if kind == NONE {
            return 0;
        }
        match kind.wrapping_sub(1) {
            0 => 4,
            1 => 8,
            2 => 8,
            3 => 0,
            4 => 4,
            _ => 0,
        }
    }
});
