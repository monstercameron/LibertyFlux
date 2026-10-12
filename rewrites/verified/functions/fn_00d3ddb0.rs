// original: 0x00D3DDB0 anim_request_manager_init

/// Reset the animation request manager through its thiscall helper. The
/// manager arrives in ECX, the helper receives the same pointer in ECX, and
/// the wrapper returns the original manager pointer in EAX. The helper has no
/// stack arguments; the wrapper makes no other memory changes.
lf_checker_rt::export!(thiscall, rw_D3DDB0(this: u32) -> u32 {
    let _reset_result = lf_checker_rt::callee_thiscall!(1, u32, this);
    this
});
