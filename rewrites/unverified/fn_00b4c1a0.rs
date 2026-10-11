// original: 0x00b4c1a0 group_find_member_index

/// Find a member pointer in the seven eight-byte records beginning at
/// object offset 0x04. Return its zero-based slot, or u32::MAX if absent.
/// The group object arrives in ECX and the candidate is one stack argument.
lf_checker_rt::export!(thiscall, rw_00B4C1A0(group: u32, candidate: u32) -> u32 {
    const FIRST_MEMBER: u32 = 0x04;
    const MEMBER_COUNT: u32 = 7;
    const MEMBER_STRIDE: u32 = 8;

    for index in 0..MEMBER_COUNT {
        let slot = group.wrapping_add(FIRST_MEMBER + index * MEMBER_STRIDE);
        // SAFETY: each entry begins with its member pointer.
        if unsafe { (slot as *const u32).read_unaligned() } == candidate {
            return index;
        }
    }
    u32::MAX
});
