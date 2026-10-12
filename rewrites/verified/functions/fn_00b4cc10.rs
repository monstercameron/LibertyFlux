// original: 0x00b4cc10 group_mark_free

/// Clear bit 0 and set bit 2 in the group's 16-bit state word at +0x56,
/// preserving every other bit. The method has no stack arguments and does
/// not produce a meaningful return value.
lf_checker_rt::export!(thiscall, rw_00b4cc10(group: u32) -> u32 {
    const STATE_FLAGS: u32 = 0x56;
    let value = unsafe { (group.wrapping_add(STATE_FLAGS) as *const u16).read_unaligned() };
    let updated = (value & !1u16) | 4u16;
    unsafe { (group.wrapping_add(STATE_FLAGS) as *mut u16).write_unaligned(updated) };
    0
});
