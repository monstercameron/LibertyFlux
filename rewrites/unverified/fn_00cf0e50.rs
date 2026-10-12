// original: 0x00CF0E50 Stat_RegisterBest_2

/// Accept a statistic identifier when the registration helper reports no
/// blocking condition, or when the signed identifier lies in its allowed range.
///
/// The cdecl function takes one 32-bit identifier. A helper with no arguments
/// supplies an AL flag; only its low byte is tested. A zero helper byte accepts
/// the identifier directly. Otherwise the signed identifier must be from
/// `0x26f` through `0x283`, inclusive. The Boolean
/// result is returned in AL; the upper EAX bits are not part of the result.
lf_checker_rt::export!(cdecl, rw_fn_00cf0e50(stat_id: u32) -> u32 {
    const MIN_BEST_STAT_ID: i32 = 0x26f;
    const MAX_BEST_STAT_ID: i32 = 0x283;
    const REGISTERING_CALLEE: u32 = 1;
    let blocking_condition = lf_checker_rt::callee_cdecl!(REGISTERING_CALLEE, u32,) as u8 != 0;
    let signed_id = stat_id as i32;
    u32::from(!blocking_condition || (MIN_BEST_STAT_ID..=MAX_BEST_STAT_ID).contains(&signed_id))
});
