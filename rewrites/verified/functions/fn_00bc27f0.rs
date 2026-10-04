// original: 0x00bc27f0 framed_task_76_0 (proposed)

/// Dispatch a ped task through a stack frame carrying words two to five.
///
/// Looks the handle up through the ped manager (`G_PEDMGR`, thiscall with
/// `arg0`). Four incoming words (`arg1`..`arg4`, the last transported as float
/// bits) are copied into a 16-byte stack frame with a zero flag word and a
/// zero pad word below it, and the frame is dispatched (fastcall: frame in
/// ecx, lookup in edx, lookup again on the stack; the callee pops nothing).
/// A null answer is returned directly; otherwise `(arg0, answer, 0x76)` is
/// reported to the result sink (cdecl, three words) and the sink's answer
/// returned. The twin at `0xBC2850` differs only in the flag word (one).
/// The proof skips the frame address and compares a six-word snapshot.
///
/// Original: 0x00BC27F0 (cdecl, five stack words).
lf_checker_rt::export!(cdecl, rw_00bc27f0(arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32) -> u32 {
    const G_PEDMGR: u32 = 0x018B6F1C;
    const KIND: u32 = 0x76;
    const FLAG: u32 = 0;
    let mgr: u32 =
        unsafe { (lf_checker_rt::relocated(G_PEDMGR) as *const u32).read_unaligned() };
    let ped: u32 = lf_checker_rt::callee_thiscall!(1, u32, mgr, arg0);
    let mut frame: [u32; 6] = [arg1, arg2, arg3, arg4, FLAG, 0];
    let ans: u32 =
        lf_checker_rt::callee_fastcall!(2, u32, frame.as_mut_ptr() as u32, ped, ped);
    if ans == 0 {
        return ans;
    }
    lf_checker_rt::callee_cdecl!(3, u32, arg0, ans, KIND)
});
