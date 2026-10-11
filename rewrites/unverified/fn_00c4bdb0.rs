// original: 0x00C4BDB0 CCamCinematicHeliChase::replace_timing_node

/// Replace the timing node stored at receiver offset 0x2a0.
///
/// The receiver is in ECX and the stack argument is the new node pointer. A
/// non-null old node is detached through the manager using the address of the
/// pointer field. The field is cleared in every case; a non-null replacement
/// is then installed and attached through the manager. The routine's EAX
/// value is an incidental helper result, so this rewrite returns a placeholder
/// and the contract compares the effects and calls instead.
lf_checker_rt::export!(thiscall, rw_c4bdb0(this: u32, replacement: u32) -> u32 {
    const NODE_POINTER_FIELD: u32 = 0x2a0;
    const DETACH_HELPER: u32 = 1;
    const ATTACH_HELPER: u32 = 2;

    let slot = this.wrapping_add(NODE_POINTER_FIELD);
    let old_node = unsafe { (slot as *const u32).read_unaligned() };
    if old_node != 0 {
        let _ = lf_checker_rt::callee_thiscall!(DETACH_HELPER, u32, old_node, slot);
    }
    unsafe { (slot as *mut u32).write_unaligned(0) };
    if replacement != 0 {
        unsafe { (slot as *mut u32).write_unaligned(replacement) };
        let _ = lf_checker_rt::callee_thiscall!(ATTACH_HELPER, u32, replacement, slot);
    }
    0
});
