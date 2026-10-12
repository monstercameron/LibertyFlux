// original: 0x00DF3320 UIMontageText::refresh

/// Refresh a montage text view in order: reset its text colors, update its
/// display value, then forward the same `this` pointer to the base frame's
/// child-refresh routine as a tail call. The three calls are independent
/// virtual behavior boundaries and remain separately observable in the call
/// log.
lf_checker_rt::export!(thiscall, rw_00df3320(this: u32) -> u32 {
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
    let _ = lf_checker_rt::callee_thiscall!(2, u32, this);
    lf_checker_rt::callee_thiscall!(3, u32, this)
});
