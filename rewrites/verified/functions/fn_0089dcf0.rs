// original: 0x0089dcf0 audio_forward_this_thunk (proposed)

/// Forwarding thunk: moves the incoming object pointer into ECX and tail-jumps.
///
/// Loads ECX from the single stack word and jumps to the shared thiscall
/// routine (which takes no stack arguments and returns its value in EAX).
/// The jump is intercepted as callee 1, so the rewrite is a plain forwarding
/// call returning the callee's answer; ECX is compared as the thiscall
/// argument on every trial.
///
/// Original: 0x0089dcf0 (cdecl-shaped, one stack word; ends in a tail jump).
lf_checker_rt::export!(cdecl, rw_0089dcf0(target_this: u32) -> u32 {
    unsafe { lf_checker_rt::callee_thiscall!(1, u32, target_this) }
});
