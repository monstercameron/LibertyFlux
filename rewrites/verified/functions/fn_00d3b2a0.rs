// original: 0x00D3B2A0 ped_task_replace_observed_pointer

/// If the current observer at `this`+0x0c is nonnull, invoke its first virtual method with mode 1. Then store the replacement pointer at +0x0c. The callee removes one stack argument and the function has no meaningful return value.
lf_checker_rt::export!(thiscall, rw_00d3b2a0(this: u32, replacement: u32) -> u32 {
    unsafe {
        const OBSERVER_SLOT: u32 = 0x0c;
        const RELEASE_OBSERVER: u32 = 1;
        let slot = this.wrapping_add(OBSERVER_SLOT);
        let old = (slot as *const u32).read_unaligned();
        if old != 0 {
            let _ = lf_checker_rt::callee_thiscall!(RELEASE_OBSERVER, u32, old, 1);
        }
        (slot as *mut u32).write_unaligned(replacement);
        0
    }
});
