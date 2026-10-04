// original: 0x00532110 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race41Standard, player_schema::LeaderboardInfo, 10>::vf13

/// Find a value in this leaderboard's row table and project it through an
/// output table.
///
/// Calls the leaderboard fetch callee (fastcall: board id in ECX, out-struct
/// pointer in EDX) with this board's id (`BOARD_ID`). The callee answers in
/// AL and fills the out-struct's count slot (word 3), row-table slot
/// (word 4) and output-table slot (word 5). On success scans
/// `rows[0..count]` for `want` with a signed count; a hit returns
/// `output[index]`, while a callee failure, a non-positive count, or a miss
/// returns `NOT_FOUND` (-1). (The original re-tests the found index against
/// -1 after the loop; a loop index is never negative, so the test is dead.)
///
/// Original: 0x00532110 (thiscall, one stack word; the entry ECX is
/// overwritten before any read, so the object pointer is ignored; the frame
/// is 8-byte aligned, an unobservable codegen detail).
lf_checker_rt::export!(thiscall, rw_00532110(_this: u32, want: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x9c;
        const FETCH_CALLEE: u32 = 1;
        const COUNT_SLOT: usize = 3;
        const ROWS_SLOT: usize = 4;
        const OUTPUT_SLOT: usize = 5;
        const NOT_FOUND: u32 = 0xffff_ffff;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut out = [0u32; 8];
        out[COUNT_SLOT] = 0;
        out[ROWS_SLOT] = 0;
        out[OUTPUT_SLOT] = 0;
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
        let rows = out[ROWS_SLOT];
        let output = out[OUTPUT_SLOT];
        let mut i = 0i32;
        while i < count {
            let cell = rd32(rows.wrapping_add((i as u32).wrapping_mul(4)));
            if cell == want {
                return rd32(output.wrapping_add((i as u32).wrapping_mul(4)));
            }
            i += 1;
        }
        NOT_FOUND
    }
});
