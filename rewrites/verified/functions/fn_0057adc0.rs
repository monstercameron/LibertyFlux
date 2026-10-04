// original: 0x0057adc0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_164, player_schema::LeaderboardInfo, 10>::vf6
/// Key-index lookup: position of one key in this race's tables.
///
/// Asks the leaderboard-data callee (contract callee 1) for this race's
/// tables, passing LEADERBOARD_ID and a scratch record the callee fills with
/// the key count (+12) and the key array (+16). Only the low byte of the
/// callee's answer is significant: zero means no data. `wanted` is then
/// searched for in KEYS (at most `count` entries, the bound compared signed)
/// and its position returned, or -1 when absent, when the count is not
/// positive, or when there is no data.
///
/// Original: thiscall, object in ECX (ignored), one stack word.
lf_checker_rt::export!(thiscall, rw_0057adc0(_this: u32, wanted: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x17c;
        const CALLEE_DATA: u32 = 1;
        const NONE: u32 = 0xFFFF_FFFF;

        #[inline(always)]
        unsafe fn rd32(addr: u32) -> u32 {
            unsafe { (addr as *const u32).read_unaligned() }
        }

        /// Scratch record the data callee fills: key count and key array at
        /// the offsets the original's frame used.
        #[repr(C)]
        struct Tables {
            _pad: [u32; 3],
            count: u32,
            keys: u32,
        }

        let mut tables = Tables { _pad: [0; 3], count: 0, keys: 0 };
        let answer: u32 = lf_checker_rt::callee_fastcall!(
            CALLEE_DATA, u32, LEADERBOARD_ID, &mut tables as *mut Tables as u32);
        // Only the low byte is the callee's boolean; the upper bytes are
        // whatever the caller left in the register on each side.
        if (answer & 0xFF) == 0 {
            return NONE;
        }
        let count = tables.count as i32;
        if count <= 0 {
            return NONE;
        }
        let mut at = 0i32;
        loop {
            if rd32(tables.keys.wrapping_add((at as u32).wrapping_mul(4))) == wanted {
                return at as u32;
            }
            at = at.wrapping_add(1);
            if at >= count {
                return NONE;
            }
        }
    }
});
