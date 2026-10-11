// original: 0x00C4BDE0 CCamCinematicHeliChase::vf7

/// Detach the node at receiver offset 0x2a0, clear that field, and return true.
///
/// The receiver is in ECX. A non-null node is passed to the detach helper with
/// the pointer-field address as its stack argument. The field is cleared even
/// when it was already null. Only AL is a defined return value; the helper
/// call is intercepted and checked by the contract.
lf_checker_rt::export!(thiscall, rw_c4bde0(this: u32) -> u32 {
    const NODE_POINTER_FIELD: u32 = 0x2a0;
    const DETACH_HELPER: u32 = 1;

    let slot = this.wrapping_add(NODE_POINTER_FIELD);
    let old_node = unsafe { (slot as *const u32).read_unaligned() };
    if old_node != 0 {
        let _ = lf_checker_rt::callee_thiscall!(DETACH_HELPER, u32, old_node, slot);
    }
    unsafe { (slot as *mut u32).write_unaligned(0) };
    1
});
