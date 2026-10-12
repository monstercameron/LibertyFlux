// original: 0x0086CC40 Script_MemcpyWrapA

/// Forward two arguments to the script-array helper with the current global
/// scratchpad pointer inserted between them, preserving the helper's return.
///
/// This cdecl wrapper reads the shared scratchpad pointer from the engine
/// data section, passes `(first, scratchpad, second)` to an intercepted cdecl
/// helper, and returns its complete EAX result.
lf_checker_rt::export!(cdecl, rw_0086CC40(first: u32, second: u32) -> u32 {
    const SCRATCHPAD_VA: u32 = 0x018B_9B2C;
    const SAVE_ARRAY_HELPER: u32 = 1;
    unsafe {
        let scratchpad = lf_checker_rt::global::<u32>(SCRATCHPAD_VA).read_unaligned();
        lf_checker_rt::callee_cdecl!(SAVE_ARRAY_HELPER, u32, first, scratchpad, second)
    }
});
