// original: 0x0086CC60 Script_MemcpyWrapB

/// Forward the global scratchpad pointer and two arguments to the helper.
///
/// The argument order is `(scratchpad, first, second)`. The intercepted
/// helper's full EAX return is returned unchanged to this wrapper's caller.
lf_checker_rt::export!(cdecl, rw_0086CC60(first: u32, second: u32) -> u32 {
    const SCRATCHPAD_VA: u32 = 0x018B_9B2C;
    const SAVE_ARRAY_HELPER: u32 = 1;
    unsafe {
        let scratchpad = lf_checker_rt::global::<u32>(SCRATCHPAD_VA).read_unaligned();
        lf_checker_rt::callee_cdecl!(SAVE_ARRAY_HELPER, u32, scratchpad, first, second)
    }
});
