// original: 0x00b4bf20 group_count_members

/// Count member records whose pointer is nonzero or whose signed marker is
/// nonnegative. The record array starts at byte 0x04, uses eight-byte entries,
/// and contains 8 entries. The object arrives in ECX and the count is
/// returned in EAX.
lf_checker_rt::export!(thiscall, rw_00B4BF20(object: u32) -> u32 {
    const FIRST_MEMBER: u32 = 0x04;
    const MEMBER_COUNT: u32 = 8;
    const MEMBER_STRIDE: u32 = 8;

    let mut occupied = 0u32;
    for index in 0..MEMBER_COUNT {
        let slot = object.wrapping_add(FIRST_MEMBER + index * MEMBER_STRIDE);
        // SAFETY: the input object contains the complete member array.
        let member = unsafe { (slot as *const u32).read_unaligned() };
        // SAFETY: each record's marker follows its pointer by one word.
        let marker = unsafe { (slot.wrapping_add(4) as *const i32).read_unaligned() };
        if member != 0 || marker >= 0 { occupied += 1; }
    }
    occupied
});
