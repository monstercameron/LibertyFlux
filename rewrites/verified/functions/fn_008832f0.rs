// original: 0x008832f0 stream_state_dispatch (proposed)
/// Dispatch on a streaming state code.
///
/// Code 1 reports ready (returns 1); code 2 tail-calls the code-2 handler
/// (intercepted callee 1, cdecl, one argument: the code itself) and returns
/// its result; any other code returns the code minus two with its low byte
/// cleared (the original falls through two `dec` instructions into `xor
/// al, al`).
///
/// Original: cdecl, one stack argument, returns in `eax`. The listed size
/// runs past the true end (two `ret` instructions and a tail jump); only the
/// instructions above belong to this function.
lf_checker_rt::export!(cdecl, rw_008832f0(code: u32) -> u32 {
    const HANDLER_CALLEE: u32 = 1;
    if code == 1 {
        1
    } else if code == 2 {
        unsafe { lf_checker_rt::callee_cdecl!(HANDLER_CALLEE, u32, code) }
    } else {
        code.wrapping_sub(2) & 0xFFFF_FF00
    }
});
