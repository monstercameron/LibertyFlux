// original: 0x00b4c420 group_contains_member

/// Return one when the non-null candidate occurs in any of the group's
/// seven member pointer fields. Entries start at offset 0x04 with stride
/// eight; the group arrives in ECX and the candidate is a stack argument.
lf_checker_rt::export!(thiscall, rw_00B4C420(group: u32, candidate: u32) -> u32 {
    const FIRST_MEMBER: u32 = 0x04;
    const MEMBER_COUNT: u32 = 7;
    const MEMBER_STRIDE: u32 = 8;
    if candidate == 0 { return 0; }

    for index in 0..MEMBER_COUNT {
        let slot = group.wrapping_add(FIRST_MEMBER + index * MEMBER_STRIDE);
        // SAFETY: each entry begins with its member pointer.
        if unsafe { (slot as *const u32).read_unaligned() } == candidate {
            return 1;
        }
    }
    0
});
