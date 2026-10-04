// original: 0x0057ab60 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_164, player_schema::LeaderboardInfo, 10>::vf12
/// Column-index lookup: position of one column value in this race's key list.
///
/// Asks the leaderboard-data callee (contract callee 1) for this race's
/// tables, passing LEADERBOARD_ID and a scratch record the callee fills with
/// the key count (+4), the key array (+8) and the value array (+20). Only the
/// low byte of the callee's answer is significant: zero means no data. The
/// value at VALUES[index] is then the probe, unless it is the empty sentinel
/// (-1); the probe is searched for in KEYS and its position returned, or -1
/// when absent, when the count is zero, or when there is no data. The search
/// bound is compared unsigned.
///
/// Edge cases: zero count returns -1 without searching; a sentinel probe
/// returns -1 without searching; a wild index faults on the value read.
///
/// Original: thiscall, object in ECX (ignored), one stack word.
lf_checker_rt::export!(thiscall, rw_0057ab60(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x17c;
        const CALLEE_DATA: u32 = 1;
        const NONE: u32 = 0xFFFF_FFFF;

        #[inline(always)]
        unsafe fn rd32(addr: u32) -> u32 {
            unsafe { (addr as *const u32).read_unaligned() }
        }

        /// Scratch record the data callee fills: key count, key array and
        /// value array at the offsets the original's frame used.
        #[repr(C)]
        struct Tables {
            _pad0: u32,
            count: u32,
            keys: u32,
            _pad1: u32,
            _pad2: u32,
            _pad3: u32,
            values: u32,
        }

        let mut tables = Tables {
            _pad0: 0, count: 0, keys: 0, _pad1: 0, _pad2: 0, _pad3: 0, values: 0,
        };
        let answer: u32 = lf_checker_rt::callee_fastcall!(
            CALLEE_DATA, u32, LEADERBOARD_ID, &mut tables as *mut Tables as u32);
        // Only the low byte is the callee's boolean; the upper bytes are
        // whatever the caller left in the register on each side.
        if (answer & 0xFF) == 0 {
            return NONE;
        }
        let probe = rd32(tables.values.wrapping_add(index.wrapping_mul(4)));
        if probe == NONE {
            return NONE;
        }
        if tables.count == 0 {
            return NONE;
        }
        let mut at = 0u32;
        loop {
            if rd32(tables.keys.wrapping_add(at.wrapping_mul(4))) == probe {
                return at;
            }
            at = at.wrapping_add(1);
            if at >= tables.count {
                return NONE;
            }
        }
    }
});
