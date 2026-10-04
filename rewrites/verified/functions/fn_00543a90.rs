// original: 0x00543a90 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_7, player_schema::LeaderboardInfo, 10>::vf6

/// Leaderboard table lookup for one ranked episodic board.
/// 
/// The original calls the shared lookup helper (ECX = board id, EDX = scratch
/// record) which reports success in AL and fills the record with a key count
/// and table pointers; the method then searches or indexes those tables.
/// Calling convention: stdcall with one stack argument; ECX is scratch.
/// ///
/// /// Find KEY in the board's key table, return its index or -1.
/// /// 
/// /// Reads the count at record+0x0c and the key-table pointer at record+0x10.
/// /// Returns -1 when the lookup fails, when the count is zero or negative, or
/// /// when KEY is absent from the first COUNT slots (signed bound).
///
/// Original: 0x00543a90 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00543a90(key: u32) -> u32 {
    unsafe {
        /// Board id passed to the lookup helper in ECX.
        const LEADERBOARD_ID: u32 = 0x00a8;
        /// Intercepted table-lookup helper (fills the scratch record).
        const LOOKUP_CALLEE: u32 = 1;
        /// Scratch record layout, byte offsets from the buffer start.
        const COUNT_OFF: u32 = 0x0c;
        const KEYS_OFF: u32 = 0x10;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;

        #[inline(always)]
        unsafe fn rd32(addr: u32) -> u32 {
            unsafe { (addr as *const u32).read_unaligned() }
        }

        let mut scratch = [0u32; 5];
        let base = scratch.as_mut_ptr() as u32;
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(LOOKUP_CALLEE, u32, LEADERBOARD_ID, base);
        if ok & 0xFF == 0 {
            return NOT_FOUND;
        }
        let count = rd32(base.wrapping_add(COUNT_OFF)) as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let keys = rd32(base.wrapping_add(KEYS_OFF));
        let mut index = 0i32;
        loop {
            let slot = keys.wrapping_add((index as u32).wrapping_mul(4));
            if rd32(slot) == key {
                return index as u32;
            }
            index += 1;
            if index >= count {
                return NOT_FOUND;
            }
        }
    }
});
