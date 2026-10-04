// original: 0x00a65200 forward_first_and_third
/// Forwards the first and third arguments to the shared worker.
///
/// The middle argument is ignored: the push of the third argument shifts the
/// frame so the worker's register slot reads the first argument.
export!(cdecl, rw_00a65200(a: u32, b: u32, c: u32) -> u32 {
    let _ = b;
    unsafe { callee_thiscall!(1, u32, a, c) }
});
