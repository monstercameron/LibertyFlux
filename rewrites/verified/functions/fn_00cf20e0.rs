// original: 0x00CF20E0 CPlayStatBase::parse

/// Forward a parse destination and this object to the play-stat parse helper.
///
/// Thiscall receives `this` in ECX and one stack argument, `parse_target`.
/// The original pushes `this` and then the target, so the cdecl helper
/// receives `(parse_target, this)` in that order. Its EAX answer is returned
/// unchanged; thiscall removes the one stack argument.
lf_checker_rt::export!(thiscall, rw_fn_00cf20e0(this: u32, parse_target: u32) -> u32 {
    const PARSE_CALLEE: u32 = 1;
    lf_checker_rt::callee_cdecl!(PARSE_CALLEE, u32, parse_target, this)
});
