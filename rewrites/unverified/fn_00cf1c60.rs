// original: 0x00CF1C60 stat_is_best_tracked

/// Test whether a statistic identifier belongs to the best-tracked set.
///
/// The cdecl argument supplies the identifier. The result is a Boolean in
/// AL; the original leaves the identifier's upper 24 bits in EAX, so the
/// rewrite preserves those bits and replaces only the low byte. No memory is
/// accessed.
lf_checker_rt::export!(cdecl, rw_fn_00cf1c60(stat_id: u32) -> u32 {
    const BEST_TRACKED_IDS: [u32; 9] = [0x1b3, 0x1b2, 0x1b4, 0x1a6, 0x1b8, 0x1b5, 0x1b6, 0x1b7, 0x73];
    let is_tracked = BEST_TRACKED_IDS.contains(&stat_id);
    (stat_id & 0xffff_ff00) | u32::from(is_tracked)
});
