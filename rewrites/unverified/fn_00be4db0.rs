// original: 0x00be4db0 task_reset_aux_block_hi (proposed)

/// Reset the high auxiliary block, then clear three trailing state words.
///
/// Runs the block-reset callee on the sub-object at `this + BLOCK_OFF`
/// (0x20), then writes zero to the three trailing words at `this + 0x64`,
/// `+0x68` and `+0x6c`, in ascending order. Returns the callee's answer
/// unchanged.
///
/// Original: 0x00be4db0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00be4db0(this: u32) -> u32 {
    unsafe {
        const BLOCK_OFF: u32 = 0x20;
        const TRAIL0: u32 = 0x64;
        const TRAIL1: u32 = 0x68;
        const TRAIL2: u32 = 0x6c;
        const RESET_BLOCK: u32 = 1;
        let r = lf_checker_rt::callee_thiscall!(RESET_BLOCK, u32, this.wrapping_add(BLOCK_OFF));
        (this.wrapping_add(TRAIL0) as *mut u32).write_unaligned(0);
        (this.wrapping_add(TRAIL1) as *mut u32).write_unaligned(0);
        (this.wrapping_add(TRAIL2) as *mut u32).write_unaligned(0);
        r
    }
});
