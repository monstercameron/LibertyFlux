// original: 0x009e6ff0 task_flag_classify (proposed)

/// Classify this object through the task-flag table, minus one.
///
/// Calls the cdecl classifier with (this, 0), zero-extends its byte
/// result and subtracts one (`thiscall`, no stack words; the callee pops
/// nothing). A zero byte result therefore yields 0xffffffff.
lf_checker_rt::export!(thiscall, rw_009e6ff0(this: u32) -> u32 {
    unsafe {
        const CLASSIFY: u32 = 1;
        let b = lf_checker_rt::callee_cdecl!(CLASSIFY, u32, this, 0u32);
        (b & 0xff).wrapping_sub(1)
    }
});
