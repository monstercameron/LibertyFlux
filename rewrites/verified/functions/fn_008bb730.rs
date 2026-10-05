// original: 0x008bb730 files_memory_shutdown_chain_stub (proposed)

/// Run two files-memory helpers, then tail-jump to a third routine whose
/// result is this function's result.
///
/// The original takes no arguments and reads no registers: it calls the
/// no-argument helper at callee 1, then the no-argument helper at callee 2
/// (both end in a plain return and take all their inputs from globals),
/// ignoring both return values, and jumps to the shared routine at callee 3
/// with the stack untouched. The shared routine also takes no stack
/// arguments (its whole body references no incoming stack slot, and its two
/// direct callers pass nothing) and ends in a plain return, so the jump is a
/// true tail call: whatever stack cleanup the caller expects is the
/// callee-3 cleanup, which pops nothing.
///
/// There are no comparisons and no memory accesses of its own in this
/// function, so there is no signedness question and no edge case beyond the
/// scripted answers of the three callees.
///
/// Original: 0x008bb730 (cdecl, no stack words; 15 bytes: call, call, jump).
lf_checker_rt::export!(cdecl, rw_008bb730() -> u32 {
    const FIRST_HELPER: u32 = 1;
    const SECOND_HELPER: u32 = 2;
    const TAIL_ROUTINE: u32 = 3;
    let _: u32 = lf_checker_rt::callee_cdecl!(FIRST_HELPER, u32,);
    let _: u32 = lf_checker_rt::callee_cdecl!(SECOND_HELPER, u32,);
    lf_checker_rt::callee_cdecl!(TAIL_ROUTINE, u32,)
});
