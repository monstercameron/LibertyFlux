// original: 0x00565d70 race87_vf7_cell_by_index

/// One leaderboard cell by row index.
///
/// `index` selects a row of the leaderboard identified by `LEADERBOARD_ID`.
/// The loader callee fills `info` (row table pointer at `+0x10`); when it
/// reports failure the result is `NOT_FOUND`. There is no bounds check: the
/// row is read at `index * 4` with wrapping address arithmetic, exactly as
/// the original.
///
/// Edge cases: loader failure returns `NOT_FOUND`; an out-of-range index
/// reads (or faults on) whatever the wrapped address reaches.
///
/// Original: thiscall, one stack word; incoming ECX is ignored.
lf_checker_rt::export!(thiscall, rw_00565d70(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x122;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        const INFO_ROWS: usize = 4; // +0x10: row table
        const CALLEE_FILL: u32 = 1;

        let mut info = [0u32; 5];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            CALLEE_FILL, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if (ok & 0xFF) == 0 {
            return NOT_FOUND;
        }
        let rows = info[INFO_ROWS];
        (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned()
    }
});
