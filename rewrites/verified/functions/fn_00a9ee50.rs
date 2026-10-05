// original: 0x00a9ee50 stream_global_thunk (proposed)

/// Load the shared context pointer from its global and tail-call the
/// shared worker routine with it.
///
/// The global at file address 0x01bb6674 holds the context pointer; it is
/// passed as the object pointer to the common routine with no additional
/// arguments, and its result is the result. Written as a forwarding call.
///
/// Original: 0x00a9ee50 (no arguments; body is `(an instruction of the original); jmp`).
lf_checker_rt::export!(stdcall, rw_00a9ee50() -> u32 {
    unsafe {
        const CONTEXT_GLOBAL: u32 = 0x01bb6674;
        const WORKER: u32 = 1;
        let ctx = lf_checker_rt::global::<u32>(CONTEXT_GLOBAL).read_unaligned();
        lf_checker_rt::callee_thiscall!(WORKER, u32, ctx)
    }
});
