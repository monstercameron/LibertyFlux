// original: 0x00C4A710 timing_node_detach_and_attach_free

/// Replace the node at this timing slot and attach the replacement to the
/// free-list manager.
///
/// The receiver is in ECX and the stack argument is the replacement pointer.
/// The routine stores whether that pointer is non-null in byte 0x181, clears
/// the word at 0x182, and clears the status dword at 0x164 when it was already
/// nonzero. A non-null old node at 0x168 is detached through a helper call.
/// The replacement is then stored at 0x168 and passed with the field address
/// to the attach helper, whose 32-bit result is returned. Both calls remain
/// visible to the contract.
lf_checker_rt::export!(thiscall, rw_c4a710(this: u32, replacement: u32) -> u32 {
    const STATUS_FIELD: u32 = 0x164;
    const NODE_POINTER_FIELD: u32 = 0x168;
    const ACTIVE_FLAG_FIELD: u32 = 0x181;
    const CLEARED_WORD_FIELD: u32 = 0x182;
    const DETACH_HELPER: u32 = 1;
    const ATTACH_HELPER: u32 = 2;

    let slot = this.wrapping_add(NODE_POINTER_FIELD);
    let old_node = unsafe { (slot as *const u32).read_unaligned() };
    let was_active = unsafe { (this.wrapping_add(STATUS_FIELD) as *const u32).read_unaligned() };

    unsafe {
        (this.wrapping_add(ACTIVE_FLAG_FIELD) as *mut u8)
            .write_unaligned(u8::from(replacement != 0));
        (this.wrapping_add(CLEARED_WORD_FIELD) as *mut u16).write_unaligned(0);
        if was_active != 0 {
            (this.wrapping_add(STATUS_FIELD) as *mut u32).write_unaligned(0);
        }
    }

    if old_node != 0 {
        let _ = lf_checker_rt::callee_thiscall!(DETACH_HELPER, u32, old_node, slot);
    }

    unsafe { (slot as *mut u32).write_unaligned(replacement) };
    lf_checker_rt::callee_thiscall!(ATTACH_HELPER, u32, replacement, slot)
});
