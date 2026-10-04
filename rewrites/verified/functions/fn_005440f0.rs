// original: 0x005440f0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_9, player_schema::LeaderboardInfo, 10>::vf12

/// Leaderboard table lookup for one ranked episodic board.
/// 
/// The original calls the shared lookup helper (ECX = board id, EDX = scratch
/// record) which reports success in AL and fills the record with a key count
/// and table pointers; the method then searches or indexes those tables.
/// Calling convention: stdcall with one stack argument; ECX is scratch.
/// ///
/// /// Find the rank-table key for INDEX in the key table.
/// /// 
/// /// Reads rank-table pointer at record+0x14, key count at record+0x04 and
/// /// key-table pointer at record+0x08. Looks up slot INDEX of the rank table;
/// /// returns -1 when the lookup fails, the rank key is -1, the count is zero,
/// /// or the key is absent from the first COUNT slots (unsigned bound).
///
/// Original: 0x005440f0 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_005440f0(index: u32) -> u32 {
    unsafe {
        /// Board id passed to the lookup helper in ECX.
        const LEADERBOARD_ID: u32 = 0x00a9;
        /// Intercepted table-lookup helper (fills the scratch record).
        const LOOKUP_CALLEE: u32 = 1;
        /// Scratch record layout, byte offsets from the buffer start.
        const COUNT_OFF: u32 = 0x04;
        const KEYS_OFF: u32 = 0x08;
        const RANKS_OFF: u32 = 0x14;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;

        #[inline(always)]
        unsafe fn rd32(addr: u32) -> u32 {
            unsafe { (addr as *const u32).read_unaligned() }
        }

        let mut scratch = [0u32; 6];
        let base = scratch.as_mut_ptr() as u32;
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(LOOKUP_CALLEE, u32, LEADERBOARD_ID, base);
        if ok & 0xFF == 0 {
            return NOT_FOUND;
        }
        let ranks = rd32(base.wrapping_add(RANKS_OFF));
        let wanted = rd32(ranks.wrapping_add(index.wrapping_mul(4)));
        if wanted == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = rd32(base.wrapping_add(COUNT_OFF));
        if count == 0 {
            return NOT_FOUND;
        }
        let keys = rd32(base.wrapping_add(KEYS_OFF));
        let mut slot = 0u32;
        loop {
            if rd32(keys.wrapping_add(slot.wrapping_mul(4))) == wanted {
                return slot;
            }
            slot = slot.wrapping_add(1);
            if slot >= count {
                return NOT_FOUND;
            }
        }
    }
});
