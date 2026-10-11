// original: 0x00D3B260 ped_task_replace_reference_and_cache_presence

/// Replace the reference at `this`+0x10. Release a different nonnull old reference, store the replacement, and acquire a different nonnull new reference. Return whether the resulting slot is nonnull in AL; the callee removes the stack argument.
lf_checker_rt::export!(thiscall, rw_00d3b260(this: u32, replacement: u32) -> u32 {
    unsafe {
        const OLD_REFERENCE: u32 = 0x10;
        const RELEASE_OLD: u32 = 1;
        const ACQUIRE_NEW: u32 = 2;
        let slot = this.wrapping_add(OLD_REFERENCE);
        let old = (this.wrapping_add(OLD_REFERENCE) as *const u32).read_unaligned();
        if old != replacement {
            if old != 0 {
                let _ = lf_checker_rt::callee_thiscall!(RELEASE_OLD, u32, old, slot);
                (slot as *mut u32).write_unaligned(0);
            }
            (slot as *mut u32).write_unaligned(replacement);
            if replacement != 0 {
                let _ = lf_checker_rt::callee_thiscall!(ACQUIRE_NEW, u32, replacement, slot);
            }
        }
        let current = (slot as *const u32).read_unaligned();
        let present = current != 0;
        (this.wrapping_add(0x3e) as *mut u8).write_unaligned(present as u8);
        present as u32
    }
});
