// original: 0x00E66F70 task_forward_coderef_00E66F70
/// Forward a fixed code reference to the task registry callee.
///
/// Pushes the file address `ARG` (relocated) and calls callee 1
/// (cdecl, one argument), discarding the result. The original
/// balances the pushed word with a pop; the rewrite uses the cdecl
/// call sequence.
///
/// Original: 0x00E66F70 (cdecl, no arguments, no return value).
lf_checker_rt::export!(cdecl, rw_00E66F70() -> u32 {
    unsafe {
        const ARG: u32 = 0x00E72170;
        lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(ARG));
        0
    }
});
