// original: 0x00a9c9e0 stream_thunk_plus_8d830 (proposed)

/// Adjust the object pointer by a fixed bias and tail-call the shared
/// worker routine.
///
/// `this` is biased by `+0x8d830` (selecting a sub-object further into the
/// same allocation) and control passes to the common routine with no
/// additional arguments; its result is the result. Written as a forwarding
/// call: the checker's tail patch observes the same biased pointer and the
/// same answer on both sides.
///
/// Original: 0x00a9c9e0 (thiscall, no stack arguments; body is
/// `(an instruction of the original); jmp`).
lf_checker_rt::export!(thiscall, rw_00a9c9e0(this: u32) -> u32 {
    unsafe {
        const SUBOBJECT_BIAS: u32 = 0x8d830;
        const WORKER: u32 = 1;
        lf_checker_rt::callee_thiscall!(WORKER, u32, this.wrapping_add(SUBOBJECT_BIAS))
    }
});
