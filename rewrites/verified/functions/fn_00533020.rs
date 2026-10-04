// original: 0x00533020 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race44Standard, player_schema::LeaderboardInfo, 10>::vf6

/// Find a value in this leaderboard's row table, returning its index.
///
/// Calls the leaderboard fetch callee (fastcall: board id in ECX, out-struct
/// pointer in EDX) with this board's id (`BOARD_ID`). The callee answers in
/// AL and fills the out-struct's count slot (word 3) and table slot (word 4).
/// On success scans `table[0..count]` for `want` with a signed count and
/// returns the first matching index, or `NOT_FOUND` (-1) when the callee
/// reports failure, the count is not positive, or nothing matches.
///
/// Original: 0x00533020 (thiscall, one stack word; the entry ECX is
/// overwritten before any read, so the object pointer is ignored; the frame
/// is 8-byte aligned, an unobservable codegen detail).
lf_checker_rt::export!(thiscall, rw_00533020(_this: u32, want: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x8b;
        const FETCH_CALLEE: u32 = 1;
        const COUNT_SLOT: usize = 3;
        const TABLE_SLOT: usize = 4;
        const NOT_FOUND: u32 = 0xffff_ffff;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut out = [0u32; 8];
        out[COUNT_SLOT] = 0;
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
        let count = out[COUNT_SLOT] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let table = out[TABLE_SLOT];
        let mut i = 0i32;
        while i < count {
            let cell = rd32(table.wrapping_add((i as u32).wrapping_mul(4)));
            if cell == want {
                return i as u32;
            }
            i += 1;
        }
        NOT_FOUND
    }
});
