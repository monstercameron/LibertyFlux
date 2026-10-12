// original: 0x00b4c120 group_remove_all_members

/// Visit the group's eight member records. A record is active when its
/// member pointer is nonzero or its signed marker at +4 is nonnegative; each
/// active slot is passed with its index and the stack flag to the helper.
/// The group arrives in ECX, the flag is one thiscall stack argument, and
/// the helper's return value is ignored.
lf_checker_rt::export!(thiscall, rw_00B4C120(group: u32, flag: u32) -> () {
    const FIRST_MEMBER: u32 = 0x04;
    const MEMBER_COUNT: u32 = 8;
    const MEMBER_STRIDE: u32 = 8;

    for index in 0..MEMBER_COUNT {
        let slot = group.wrapping_add(FIRST_MEMBER + index * MEMBER_STRIDE);
        // SAFETY: the group object contains all eight pointer-marker records.
        let member = unsafe { (slot as *const u32).read_unaligned() };
        let marker = unsafe { (slot.wrapping_add(4) as *const i32).read_unaligned() };
        if member != 0 || marker >= 0 {
            let _ignored = lf_checker_rt::callee_thiscall!(1, u32, group, index, flag);
        }
    }
});
