// original: 0x00C50630 task_layout_record_address

/// Compute the address of a record in a task-owned fixed-stride table.
///
/// `this` is the beginning of the owning layout, `group` selects a group,
/// and `slot` selects a record within that group. The original uses 32-bit
/// wrapping arithmetic and returns `this + 0x10 + (group * 0x2a + slot) * 0x44`;
/// it does not dereference the resulting address.
lf_checker_rt::export!(thiscall, rw_00c50630(this: u32, group: u32, slot: u32) -> u32 {
    let ordinal = group.wrapping_mul(0x2a).wrapping_add(slot);
    this.wrapping_add(0x10).wrapping_add(ordinal.wrapping_mul(0x44))
});
