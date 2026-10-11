// original: 0x00a223a0 task_record_replace_reference

/// Replace the object pointer stored at offset 0x384. Release a non-null old
/// object first, write the replacement, then retain a non-null replacement.
/// Each helper receives the object pointer in ECX and the slot address on the
/// stack.
lf_checker_rt::export!(thiscall, rw_00a223a0(this: u32, replacement: u32) -> u32 {
    const OBJECT_SLOT: u32 = 0x384;
    unsafe {
        let slot = this.wrapping_add(OBJECT_SLOT);
        let old_object = (slot as *const u32).read_unaligned();
        if old_object != 0 {
            let _ = lf_checker_rt::callee_thiscall!(1, u32, old_object, slot);
        }
        (slot as *mut u32).write_unaligned(replacement);
        if replacement != 0 {
            let _ = lf_checker_rt::callee_thiscall!(2, u32, replacement, slot);
        }
    }
    0
});
