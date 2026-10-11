// original: 0X00B4C330 group_has_leader

/// Return one when the leader pointer at offset 0x3C is non-null or the
/// signed leader marker at offset 0x40 is nonnegative. The object arrives
/// in ECX and the boolean is returned in AL.
lf_checker_rt::export!(thiscall, rw_00B4C330(group: u32) -> u32 {
    const LEADER_PTR: u32 = 0x3C;
    const LEADER_MARKER: u32 = 0x40;
    // SAFETY: the supplied group object covers both leader fields.
    let leader = unsafe { (group.wrapping_add(LEADER_PTR) as *const u32).read_unaligned() };
    // SAFETY: the marker is interpreted as a signed 32-bit value.
    let marker = unsafe { (group.wrapping_add(LEADER_MARKER) as *const i32).read_unaligned() };
    u32::from(leader != 0 || marker >= 0)
});
