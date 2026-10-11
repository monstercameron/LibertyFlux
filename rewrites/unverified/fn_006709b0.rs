// original: 0x006709B0 streaming_forward_as_this

/// Forward the one stack argument as the callee's this pointer and return its
/// result unchanged. The wrapper's own incoming this pointer is not forwarded.
///
/// Original: 0x006709B0 (thiscall, one stack argument, one direct call).
lf_checker_rt::export!(thiscall, rw_006709b0(this: u32, target: u32) -> u32 {
    unsafe { lf_checker_rt::callee_thiscall!(1, u32, target) }
});
