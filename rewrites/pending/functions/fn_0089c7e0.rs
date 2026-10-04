// original: 0x0089c7e0 worker_call_is_nonzero
/// Boolean wrapper: call the cdecl/3 worker with two global words and the
/// argument, then report whether its answer was nonzero.
///
/// The low byte holds the boolean; the upper three bytes pass through from
/// the worker's answer, matching what the original leaves in EAX.
export!(cdecl, rw_0089c7e0(arg: u32) -> u32 {
    unsafe {
        let ans: u32 = callee_cdecl!(
            1,
            u32,
            arg,
            *global::<u32>(0x115F830),
            *global::<u32>(0x115F82C)
        );
        (ans & 0xFFFF_FF00) | ((ans != 0) as u32)
    }
});
