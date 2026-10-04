// original: 0x00be4de0 task_reset_aux_block_lo (proposed)

/// Reset the low auxiliary block, then clear three trailing state words.
///
/// Runs the block-reset callee on the sub-object at `this + BLOCK_OFF`
/// (0x14), then writes zero to the three trailing words at `this + 0x58`,
/// `+0x5c` and `+0x60`, in ascending order. Returns the callee's answer
/// unchanged. Same shape as the high-block twin, differing only in the four
/// offsets.
///
/// Original: 0x00be4de0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00be4de0(this: u32) -> u32 {
    unsafe {
        const BLOCK_OFF: u32 = 0x14;
        const TRAIL0: u32 = 0x58;
        const TRAIL1: u32 = 0x5c;
        const TRAIL2: u32 = 0x60;
        const RESET_BLOCK: u32 = 1;
        let r = lf_checker_rt::callee_thiscall!(RESET_BLOCK, u32, this.wrapping_add(BLOCK_OFF));
        (this.wrapping_add(TRAIL0) as *mut u32).write_unaligned(0);
        (this.wrapping_add(TRAIL1) as *mut u32).write_unaligned(0);
        (this.wrapping_add(TRAIL2) as *mut u32).write_unaligned(0);
        r
    }
});
