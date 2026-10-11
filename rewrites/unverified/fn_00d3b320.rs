// original: 0x00D3B320 ped_task_replace_linked_reference

/// Replace the reference at `this`+0x1c; release and acquire changed references. When the new object at +0x28 has flags masked by 0x3c0 equal to 0x100, notify the linked object through helper 3. Set or clear bit 0x10 in the task byte at +0x48 according to whether the replacement is null. The callee removes its stack argument.
lf_checker_rt::export!(thiscall, rw_00d3b320(this: u32, replacement: u32) -> u32 {
    unsafe {
        const OLD_REFERENCE: u32 = 0x1c;
        const LINKED_OBJECT: u32 = 0x18;
        const RELEASE_OLD: u32 = 1;
        const ACQUIRE_NEW: u32 = 2;
        const NOTIFY_LINK: u32 = 3;
        const TASK_FLAG: u32 = 0x48;
        let slot = this.wrapping_add(OLD_REFERENCE);
        let old = (slot as *const u32).read_unaligned();
        if old != replacement {
            if old != 0 {
                let _ = lf_checker_rt::callee_thiscall!(RELEASE_OLD, u32, old, slot);
                (slot as *mut u32).write_unaligned(0);
            }
            (slot as *mut u32).write_unaligned(replacement);
            if replacement != 0 {
                let _ = lf_checker_rt::callee_thiscall!(ACQUIRE_NEW, u32, replacement, slot);
                let linked = (this.wrapping_add(LINKED_OBJECT) as *const u32).read_unaligned();
                let flags = (replacement.wrapping_add(0x28) as *const u32).read_unaligned();
                if linked != 0 && (flags & 0x3c0) == 0x100 {
                    let _ = lf_checker_rt::callee_thiscall!(NOTIFY_LINK, u32, replacement, linked, 1);
                }
                (this.wrapping_add(TASK_FLAG) as *mut u8).write_unaligned(
                    (this.wrapping_add(TASK_FLAG) as *const u8).read_unaligned() | 0x10);
            } else {
                (this.wrapping_add(TASK_FLAG) as *mut u8).write_unaligned(
                    (this.wrapping_add(TASK_FLAG) as *const u8).read_unaligned() & !0x10);
            }
        }
        0
    }
});
