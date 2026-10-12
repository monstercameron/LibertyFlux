// original: 0x00CF2040 CPlayStatBase::serialize

/// Forward a serialized value and this object to the play-stat publishing
/// helper.
///
/// Thiscall receives `this` in ECX and one stack argument, `serialized_value`.
/// The original pushes `this` and then the value, so the cdecl helper receives
/// `(serialized_value, this)` in that order. Its 32-bit EAX answer is returned
/// unchanged; thiscall removes the one stack argument.
lf_checker_rt::export!(thiscall, rw_fn_00cf2040(this: u32, serialized_value: u32) -> u32 {
    const PUBLISH_CALLEE: u32 = 1;
    lf_checker_rt::callee_cdecl!(PUBLISH_CALLEE, u32, serialized_value, this)
});
