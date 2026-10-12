// original: 0x00600C50 forward_if_live (proposed)

/// Forward to a one-argument method unless the target is null.
///
/// With a null target returns without calling (the original leaves EAX
/// untouched there; this proof fixes entry EAX to zero). Otherwise calls the
/// method with the target as `this` and the second stack argument on the
/// stack, returning the method's result. The original takes two stack
/// arguments without cleaning them up itself.
lf_checker_rt::export!(cdecl, rw_00600c50(target: u32, value: u32) -> u32 {
    unsafe {
        const METHOD_CALLEE: u32 = 1;
        if target == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(METHOD_CALLEE, u32, target, value)
    }
});
