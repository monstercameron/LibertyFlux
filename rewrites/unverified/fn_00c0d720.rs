// original: 0x00C0D720 UILayoutFrame::is_child_value_mismatched

/// Reports a mismatch when the child pointer at offset 0xEC is non-null and
/// its state word at +4 is one, then compares the child's value at +8 with the
/// result of virtual slot 19. A null child or a different state returns false;
/// a matching value also returns false. The method returns its boolean in AL.
lf_checker_rt::export!(thiscall, rw_c0d720(this: u32, expected_value: u32) -> u32 {
    const CHILD_FIELD: u32 = 0xEC;
    const CHILD_STATE: u32 = 4;
    const CHILD_VALUE: u32 = 8;
    const VALUE_SLOT: u32 = 1;

    let child = unsafe { (this.wrapping_add(CHILD_FIELD) as *const u32).read_unaligned() };
    if child == 0 {
        return 0;
    }
    let state = unsafe { (child.wrapping_add(CHILD_STATE) as *const u32).read_unaligned() };
    if state != 1 {
        return 1;
    }
    let current_value = unsafe { (child.wrapping_add(CHILD_VALUE) as *const u32).read_unaligned() };
    let reported_value = unsafe { lf_checker_rt::callee_thiscall!(VALUE_SLOT, u32, this, expected_value) };
    u32::from(current_value != reported_value)
});
