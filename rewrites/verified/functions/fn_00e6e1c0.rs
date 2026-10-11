// original: 0x00E6E1C0 mainloop_timing_dispatch_pending_buffer

/// Read the timing buffer pointer and its unsigned 16-bit length. A zero
/// length returns without a call; otherwise the buffer and zero-extended
/// length are passed to the stdcall processor, whose EAX value is returned.
lf_checker_rt::export!(cdecl, rw_00e6e1c0() -> u32 {
    const TIMING_BUFFER_VA: u32 = 0x018d_2168;
    const TIMING_LENGTH_VA: u32 = 0x018d_216e;
    const PROCESS_TIMING_CALLEE_ID: u32 = 1;

    let length = unsafe { lf_checker_rt::global::<u16>(TIMING_LENGTH_VA).read() };
    if length == 0 {
        return 0;
    }
    let buffer = unsafe { lf_checker_rt::global::<u32>(TIMING_BUFFER_VA).read() };
    lf_checker_rt::callee_stdcall!(
        PROCESS_TIMING_CALLEE_ID,
        u32,
        buffer,
        u32::from(length)
    )
});
