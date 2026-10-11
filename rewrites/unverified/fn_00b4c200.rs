// original: 0X00B4C200 group_member_dense_index

/// Return the dense index of a candidate in seven group slots. A slot is
/// active when its member pointer is nonzero or its signed marker at +4 is
/// negative. The group arrives in ECX; the candidate is a stack argument.
lf_checker_rt::export!(thiscall, rw_00B4C200(group: u32, candidate: u32) -> u32 {
    const FIRST_MEMBER: u32 = 0x04;
    const MEMBER_COUNT: u32 = 7;
    const MEMBER_STRIDE: u32 = 8;
    let mut dense_index = 0u32;

    for index in 0..MEMBER_COUNT {
        let slot = group.wrapping_add(FIRST_MEMBER + index * MEMBER_STRIDE);
        // SAFETY: each entry contains a pointer followed by its signed marker.
        let member = unsafe { (slot as *const u32).read_unaligned() };
        if member == candidate { return dense_index; }
        // SAFETY: the marker is the second word of the current entry.
        let marker = unsafe { (slot.wrapping_add(4) as *const i32).read_unaligned() };
        if member != 0 || marker < 0 { dense_index += 1; }
    }
    u32::MAX
});
