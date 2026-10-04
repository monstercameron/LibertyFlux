// original: 0x00532360 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race41Standard, player_schema::LeaderboardInfo, 10>::vf7

/// Look up one entry of this leaderboard's row table by index.
///
/// Calls the leaderboard fetch callee (fastcall: board id in ECX, out-struct
/// pointer in EDX) with this board's id (`BOARD_ID`). The callee answers in
/// AL and fills the out-struct's table slot (word 4, the only word read).
/// On success returns `table[index]` (32-bit wraparound addressing, no
/// bounds check); when the callee reports failure returns `NOT_FOUND` (-1).
///
/// Original: 0x00532360 (thiscall, one stack word; ECX is overwritten before
/// any read, so the entry object pointer is ignored).
lf_checker_rt::export!(thiscall, rw_00532360(_this: u32, index: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x9c;
        const FETCH_CALLEE: u32 = 1;
        const TABLE_SLOT: usize = 4;
        const NOT_FOUND: u32 = 0xffff_ffff;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut out = [0u32; 8];
        out[TABLE_SLOT] = 0;
        let ok: u8 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE,
            u8,
            BOARD_ID,
            out.as_mut_ptr() as u32
        );
        if ok == 0 {
            return NOT_FOUND;
        }
        let table = out[TABLE_SLOT];
        rd32(table.wrapping_add(index.wrapping_mul(4)))
    }
});
