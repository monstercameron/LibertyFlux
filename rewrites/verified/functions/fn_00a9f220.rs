// original: 0x00a9f220 stream_measure_sqrt (proposed)

/// Measure a value through two helpers and return its square root.
///
/// `arg` is passed (in ECX) to the selector, whose answer selects the input
/// of the measurer; the measurer returns a float on the x87 stack. The
/// square root is taken with scalar SSE (`sqrtss`, one lane of the
/// original's `sqrtps`, whose other lanes are discarded) and returned on
/// the x87 stack.
///
/// Original: 0x00a9f220 (cdecl, one stack word; float result in ST0).
lf_checker_rt::export!(cdecl, rw_00a9f220(arg: u32) -> f32 {
    unsafe {
        const SELECT: u32 = 1;
        const MEASURE: u32 = 2;
        let picked: u32 = lf_checker_rt::callee_thiscall!(SELECT, u32, arg);
        let measured: f32 = lf_checker_rt::callee_cdecl!(MEASURE, f32, picked);
        core::hint::black_box(measured).sqrt()
    }
});
