// original: 0x00543f90 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_8, player_schema::LeaderboardInfo, 10>::vf8

/// Leaderboard table lookup for one ranked episodic board.
/// 
/// The original calls the shared lookup helper (ECX = board id, EDX = scratch
/// record) which reports success in AL and fills the record with a key count
/// and table pointers; the method then searches or indexes those tables.
/// Calling convention: stdcall with one stack argument; ECX is scratch.
/// ///
/// /// Map the kind of key-table slot INDEX to a size code.
/// /// 
/// /// Reads the key-table pointer at record+0x14, passes slot INDEX through the
/// /// kind query (thiscall, key in ECX), then maps kind 1 -> 4, kinds 2 and 3 ->
/// /// 8, kind 5 -> 4, and anything else (including -1 and 0) -> 0. The original
/// /// implements the map as a five-way jump table.
///
/// Original: 0x00543f90 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00543f90(index: u32) -> u32 {
    unsafe {
        /// Board id passed to the lookup helper in ECX.
        const LEADERBOARD_ID: u32 = 0x00a7;
        /// Intercepted table-lookup helper (fills the scratch record).
        const LOOKUP_CALLEE: u32 = 1;
        /// Intercepted kind query (thiscall, key in ECX).
        const KIND_CALLEE: u32 = 2;
        /// Scratch record layout, byte offsets from the buffer start.
        const KEYS_OFF: u32 = 0x14;

        #[inline(always)]
        unsafe fn rd32(addr: u32) -> u32 {
            unsafe { (addr as *const u32).read_unaligned() }
        }

        let mut scratch = [0u32; 6];
        let base = scratch.as_mut_ptr() as u32;
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(LOOKUP_CALLEE, u32, LEADERBOARD_ID, base);
        if ok & 0xFF == 0 {
            return 0;
        }
        let keys = rd32(base.wrapping_add(KEYS_OFF));
        let key = rd32(keys.wrapping_add(index.wrapping_mul(4)));
        let kind: u32 = lf_checker_rt::callee_thiscall!(KIND_CALLEE, u32, key);
        if kind == 0xFFFF_FFFF {
            return 0;
        }
        match kind.wrapping_sub(1) {
            0 => 4,
            1 | 2 => 8,
            4 => 4,
            _ => 0,
        }
    }
});
