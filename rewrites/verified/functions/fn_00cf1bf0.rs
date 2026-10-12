// original: 0x00CF1BF0 stat_entry_is_displayable

/// Decide whether a statistics row should be shown in the frontend.
///
/// The cdecl inputs are a group index, a row index, and a byte-valued flag
/// that enables the `-99` sentinel check. The group lookup helper supplies the
/// row array. When requested, the sentinel is tested before the null-array
/// check, matching the original's read order. A non-null row then passes three
/// helper gates in order: mode eligibility for the signed row id, row-specific
/// displayability for the row pointer, and frontend visibility for the signed
/// row id. Each Boolean is observed through AL; the result is returned in AL.
lf_checker_rt::export!(cdecl, rw_fn_00cf1bf0(group_index: u32, row_index: u32, check_sentinel: u32) -> u32 {
    const ROW_STRIDE: u32 = 8;
    const EMPTY_SENTINEL: i16 = -99;
    const LIST_CALLEE: u32 = 1;
    const MODE_CALLEE: u32 = 2;
    const DISPLAY_CALLEE: u32 = 3;
    const FRONTEND_CALLEE: u32 = 4;

    let rows = lf_checker_rt::callee_cdecl!(LIST_CALLEE, u32, group_index);
    let row = rows.wrapping_add(row_index.wrapping_mul(ROW_STRIDE));
    if check_sentinel as u8 != 0 {
        let candidate_id = unsafe { (row as *const i16).read_unaligned() };
        if candidate_id == EMPTY_SENTINEL {
            return 0;
        }
    }
    if rows == 0 {
        return 0;
    }
    let statistic_id = unsafe { (row as *const i16).read_unaligned() } as i32 as u32;
    if lf_checker_rt::callee_cdecl!(MODE_CALLEE, u32, statistic_id) as u8 == 0 {
        return 0;
    }
    if lf_checker_rt::callee_cdecl!(DISPLAY_CALLEE, u32, row) as u8 == 0 {
        return 0;
    }
    if lf_checker_rt::callee_cdecl!(FRONTEND_CALLEE, u32, statistic_id) as u8 == 0 {
        return 0;
    }
    1
});
