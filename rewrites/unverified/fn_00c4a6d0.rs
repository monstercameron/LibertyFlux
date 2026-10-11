// original: 0x00C4A6D0 timing_node_detach_from_owner

/// Detach a non-null timing node, then store and return a boolean flag.
///
/// The receiver is in ECX and the stack argument is an integer status. If the
/// node pointer at receiver offset 0x168 is non-null, the helper is called with
/// that node in ECX and the address of the pointer field as its stack argument.
/// The routine then stores whether the status is nonzero at byte offset 0x183,
/// clears the word at 0x181, copies the full status word to 0x164, and returns
/// the boolean in AL. The helper call is intercepted and compared by the
/// checker; a null node skips it.
lf_checker_rt::export!(thiscall, rw_c4a6d0(this: u32, status: u32) -> u32 {
    const NODE_POINTER_FIELD: u32 = 0x168;
    const STATUS_FIELD: u32 = 0x164;
    const RESET_WORD_FIELD: u32 = 0x181;
    const BOOLEAN_FIELD: u32 = 0x183;
    const DETACH_HELPER: u32 = 1;

    let pointer_field = this.wrapping_add(NODE_POINTER_FIELD);
    let node = unsafe { (pointer_field as *const u32).read_unaligned() };
    if node != 0 {
        let _ = lf_checker_rt::callee_thiscall!(DETACH_HELPER, u32, node, pointer_field);
    }

    let is_active = u32::from(status != 0);
    unsafe {
        (this.wrapping_add(BOOLEAN_FIELD) as *mut u8).write_unaligned(is_active as u8);
        (this.wrapping_add(RESET_WORD_FIELD) as *mut u16).write_unaligned(0);
        (this.wrapping_add(STATUS_FIELD) as *mut u32).write_unaligned(status);
    }
    is_active
});
