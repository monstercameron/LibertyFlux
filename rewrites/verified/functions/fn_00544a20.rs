// original: 0x00544a20 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_11, player_schema::LeaderboardInfo, 10>::vf13

/// Leaderboard table lookup for one ranked episodic board.
/// 
/// The original calls the shared lookup helper (ECX = board id, EDX = scratch
/// record) which reports success in AL and fills the record with a key count
/// and table pointers; the method then searches or indexes those tables.
/// Calling convention: stdcall with one stack argument; ECX is scratch.
/// ///
/// /// Map VALUE through the board's key table to its payload.
/// /// 
/// /// Reads the count at record+0x0c, the key-table pointer at record+0x10 and
/// /// the payload-table pointer at record+0x14. Returns the payload at the slot
/// /// where VALUE is found, or -1 when the lookup fails, the count is zero or
/// /// negative, or VALUE is absent from the first COUNT slots (signed bound).
/// /// (The original re-checks the found index against -1; it cannot be -1.)
///
/// Original: 0x00544a20 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00544a20(value: u32) -> u32 {
    unsafe {
        /// Board id passed to the lookup helper in ECX.
        const LEADERBOARD_ID: u32 = 0x00ab;
        /// Intercepted table-lookup helper (fills the scratch record).
        const LOOKUP_CALLEE: u32 = 1;
        /// Scratch record layout, byte offsets from the buffer start.
        const COUNT_OFF: u32 = 0x0c;
        const KEYS_OFF: u32 = 0x10;
        const PAYLOADS_OFF: u32 = 0x14;
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
        let count = rd32(base.wrapping_add(COUNT_OFF)) as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let keys = rd32(base.wrapping_add(KEYS_OFF));
        let mut index = 0i32;
        loop {
            let slot = keys.wrapping_add((index as u32).wrapping_mul(4));
            if rd32(slot) == value {
                let payloads = rd32(base.wrapping_add(PAYLOADS_OFF));
                return rd32(payloads.wrapping_add((index as u32).wrapping_mul(4)));
            }
            index += 1;
            if index >= count {
                return NOT_FOUND;
            }
        }
    }
});
