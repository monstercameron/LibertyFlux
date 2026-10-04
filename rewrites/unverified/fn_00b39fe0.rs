// original: 0x00b39fe0 task_param_scaler_pair (proposed)

/// Copy the two task-parameter scaler floats from their globals to two
/// caller buffers: the first scaler to `dst_first`, the second to
/// `dst_second`. The values are moved as raw words (no arithmetic, so NaN
/// payloads survive bit-exactly). Returns `dst_second`, matching the
/// original's exit register. Original: 0x00b39fe0 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00b39fe0(dst_first: u32, dst_second: u32) -> u32 {
    unsafe {
        const SCALER_FIRST: u32 = 0x0104592c;
        const SCALER_SECOND: u32 = 0x01045930;
        let src_first = lf_checker_rt::global::<u32>(SCALER_FIRST).read();
        let src_second = lf_checker_rt::global::<u32>(SCALER_SECOND).read();
        (dst_first as *mut u32).write_unaligned(src_first);
        (dst_second as *mut u32).write_unaligned(src_second);
        dst_second
    }
});
