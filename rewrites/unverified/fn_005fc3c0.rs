// original: 0x005FC3C0 forward_to_method (proposed)

/// Forward two stack arguments to a one-argument method.
///
/// Calls the method with the first stack argument as `this` and the second
/// on the stack, returning the method's result. The original takes two
/// stack arguments without cleaning them up itself.
lf_checker_rt::export!(cdecl, rw_005fc3c0(target: u32, value: u32) -> u32 {
    unsafe {
        const METHOD_CALLEE: u32 = 1;
        lf_checker_rt::callee_thiscall!(METHOD_CALLEE, u32, target, value)
    }
});
