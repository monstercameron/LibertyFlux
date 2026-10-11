// original: 0x00AA0D00 stream_submit_update
/// Forwards the single stack argument as the target method's `this` pointer
/// and returns that method's full 32-bit result. The wrapper removes its one
/// stack argument on return.
lf_checker_rt::export!(stdcall, rw_00AA0D00(target: u32) -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, target)
});
