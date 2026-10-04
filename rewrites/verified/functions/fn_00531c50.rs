// original: 0x00531c50 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race40Standard, player_schema::LeaderboardInfo, 10>::vf12

/// Two-stage leaderboard lookup: resolve a key, then find it in the row table.
///
/// Calls the leaderboard fetch callee (fastcall: board id in ECX, out-struct
/// pointer in EDX) with this board's id (`BOARD_ID`). The callee answers in
/// AL and fills the out-struct's count slot (word 1), row-table slot
/// (word 2) and key-table slot (word 5). On success reads `key =
/// keys[index]` (32-bit wraparound addressing, no bounds check); a key of
/// -1, a zero count, or a callee failure yields `NOT_FOUND` (-1). Otherwise
/// scans `rows[0..count]` for the key with an unsigned count and returns the
/// first matching index, or `NOT_FOUND` when absent.
///
/// Original: 0x00531c50 (thiscall, one stack word; the entry ECX is
/// overwritten before any read, so the object pointer is ignored).
lf_checker_rt::export!(thiscall, rw_00531c50(_this: u32, index: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x98;
        const FETCH_CALLEE: u32 = 1;
        const COUNT_SLOT: usize = 1;
        const ROWS_SLOT: usize = 2;
        const KEYS_SLOT: usize = 5;
        const NOT_FOUND: u32 = 0xffff_ffff;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut out = [0u32; 8];
        out[COUNT_SLOT] = 0;
        out[ROWS_SLOT] = 0;
        out[KEYS_SLOT] = 0;
        let ok: u8 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE,
            u8,
            BOARD_ID,
            out.as_mut_ptr() as u32
        );
        if ok == 0 {
            return NOT_FOUND;
        }
        let keys = out[KEYS_SLOT];
        let key = rd32(keys.wrapping_add(index.wrapping_mul(4)));
        if key == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = out[COUNT_SLOT];
        if count == 0 {
            return NOT_FOUND;
        }
        let rows = out[ROWS_SLOT];
        let mut i = 0u32;
        while i < count {
            let cell = rd32(rows.wrapping_add(i.wrapping_mul(4)));
            if cell == key {
                return i;
            }
            i += 1;
        }
        NOT_FOUND
    }
});
