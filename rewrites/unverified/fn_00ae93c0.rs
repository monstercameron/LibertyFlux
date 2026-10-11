// original: 0x00AE93C0 set_timing_category_mask

/// Sets one of 32 category bits through the timing-mask helper, but only when
/// both signed category indexes are in the half-open range `0..30`. Negative
/// indexes and indexes at or above 30 return without calling the helper. The
/// third input supplies the shift count, reduced to its low five bits.
///
/// Calling convention: cdecl with two signed 32-bit indexes and one unsigned
/// shift value. The helper receives the first and second indexes followed by
/// the mask.
lf_checker_rt::export!(cdecl, rw_00ae93c0(first_index: i32, second_index: i32, shift_index: u32) -> () {
    if first_index < 0 || second_index < 0 || first_index >= 30 || second_index >= 30 {
        return;
    }

    const APPLY_MASK_CALLEE: u32 = 1;
    let bit = 1u32.wrapping_shl(shift_index & 31);
    lf_checker_rt::callee_cdecl!(
        APPLY_MASK_CALLEE,
        u32,
        first_index as u32,
        second_index as u32,
        bit,
    );
});
