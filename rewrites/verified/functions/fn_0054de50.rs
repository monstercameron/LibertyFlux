// original: 0x0054de50 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_0, player_schema::LeaderboardInfo, 10>::vf12

/// Find the rank id of leaderboard slot `index` in the rank table.
///
/// Calls the info-fetch callee (fastcall: ECX = schema id 0xaf, EDX = info
/// buffer) which reports success in AL and fills the buffer: rank count at
/// `+0x04`, rank-table pointer at `+0x08`, slot-table pointer at `+0x14`.
/// On success reads the rank id `slot[index]`; if it is -1 the answer is
/// `NOT_FOUND`. Otherwise linearly scans the rank table (up to `count`
/// entries, compared unsigned) for that id and returns the first matching
/// position, or `NOT_FOUND` (0xffffffff) when the count is zero or no
/// entry matches. The index is used as-is with no bounds check.
///
/// Original: stdcall, one stack word; incoming ECX is unused (overwritten
/// with the schema id before the call).
lf_checker_rt::export!(stdcall, rw_0054de50(index: u32) -> u32 {
    unsafe {
        const SCHEMA_ID: u32 = 0xaf;
        const COUNT_OFF: u32 = 0x04;
        const RANKS_OFF: u32 = 0x08;
        const SLOTS_OFF: u32 = 0x14;
        const NOT_FOUND: u32 = 0xffff_ffff;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut info = [0u32; 8];
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(0, u32, SCHEMA_ID, info.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return NOT_FOUND;
        }
        let slots = info[(SLOTS_OFF / 4) as usize];
        let needle = rd32(slots.wrapping_add(index.wrapping_mul(4)));
        if needle == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = info[(COUNT_OFF / 4) as usize];
        if count == 0 {
            return NOT_FOUND;
        }
        let ranks = info[(RANKS_OFF / 4) as usize];
        let mut i: u32 = 0;
        while i < count {
            if rd32(ranks.wrapping_add(i.wrapping_mul(4))) == needle {
                return i;
            }
            i = i.wrapping_add(1);
        }
        NOT_FOUND
    }
});
