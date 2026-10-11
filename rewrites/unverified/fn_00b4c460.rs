// original: 0x00b4c460 NativeImpl_IS_GROUP_LEADER

/// Return one when a non-null candidate equals the group's leader pointer
/// at byte offset 0x3C. The group arrives in ECX, the candidate is one stack
/// argument, and AL carries the boolean result.
lf_checker_rt::export!(thiscall, rw_00B4C460(group: u32, candidate: u32) -> u32 {
    const LEADER_PTR: u32 = 0x3C;
    if candidate == 0 { return 0; }
    // SAFETY: the supplied group object covers its leader pointer.
    let leader = unsafe { (group.wrapping_add(LEADER_PTR) as *const u32).read_unaligned() };
    u32::from(leader == candidate)
});
