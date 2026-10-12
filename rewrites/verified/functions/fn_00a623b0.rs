// original: 0x00A623B0 ped_group_match_either

/// Tests task compatibility in both directions. It first calls the helper
/// with the first object's member at +0x224 and the second object as its sole
/// argument. If that low-byte result is zero, it calls the helper in reverse.
/// The function returns one in AL when either direction succeeds. Both object
/// pointers are cdecl stack arguments; each compatibility helper is a
/// thiscall taking one stack word.
lf_checker_rt::export!(cdecl, rw_00A623B0(first_object: u32, second_object: u32) -> u32 {
    const TASK_MEMBER: u32 = 0x224;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    let first_member = unsafe { read_u32(first_object.wrapping_add(TASK_MEMBER)) };
    let first_matches = lf_checker_rt::callee_thiscall!(
        1, u32, first_member, second_object,
    ) & 0xff != 0;
    if first_matches {
        return 1;
    }

    let second_member = unsafe { read_u32(second_object.wrapping_add(TASK_MEMBER)) };
    let second_matches = lf_checker_rt::callee_thiscall!(
        2, u32, second_member, first_object,
    ) & 0xff != 0;
    u32::from(second_matches)
});
