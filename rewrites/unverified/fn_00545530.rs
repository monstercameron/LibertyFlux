// original: 0x00545530 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_13, player_schema::LeaderboardInfo, 10>::vf7

/// Leaderboard table lookup for one ranked episodic board.
/// 
/// The original calls the shared lookup helper (ECX = board id, EDX = scratch
/// record) which reports success in AL and fills the record with a key count
/// and table pointers; the method then searches or indexes those tables.
/// Calling convention: stdcall with one stack argument; ECX is scratch.
/// ///
/// /// Return the key-table slot INDEX, or -1 when the lookup fails.
/// /// 
/// /// Reads the key-table pointer at record+0x10 and returns slot INDEX of it
/// /// with no bounds check, like the original.
///
/// Original: 0x00545530 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00545530(index: u32) -> u32 {
    unsafe {
        /// Board id passed to the lookup helper in ECX.
        const LEADERBOARD_ID: u32 = 0x00ad;
        /// Intercepted table-lookup helper (fills the scratch record).
        const LOOKUP_CALLEE: u32 = 1;
        /// Scratch record layout, byte offsets from the buffer start.
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
        let keys = rd32(base.wrapping_add(KEYS_OFF));
        rd32(keys.wrapping_add(index.wrapping_mul(4)))
    }
});
