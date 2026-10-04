// original: 0x00E60B20 timer_obj_init_thunk (proposed)

/// Tail-jump forwarder to an object initialiser.
///
/// Behaviour: calls callee 1 (thiscall, `this` = `OBJECT`, no stack
/// arguments) and returns its answer. The original reaches the callee
/// with a jump, not a call; the rewrite performs the same transfer as a
/// call, which the checker's tail-jump patching observes identically.
///
/// Original: no stack arguments; `this` is an immediate moved into ECX.
/// Return value is the callee's answer.
lf_checker_rt::export!(cdecl, rw_00e60b20() -> u32 {
    unsafe { lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(0x0019F8168)) }
});

