// original: 0x00c43ee0 CIntermezzoEventIntro::vf1
/// Count intro-event children in state 0. For signed counts at or below zero,
/// resolve the active game root and clear bit 0 at root+0x228+0x4c8 when that
/// link exists. Return the signed-positive predicate as a full EAX value.
lf_checker_rt::export!(thiscall, rw_00c43ee0(this: u32) -> u32 {
    const CHILD_LIST: u32 = 4;
    const COUNT_CHILDREN: u32 = 1;
    const GET_ACTIVE_ROOT: u32 = 2;
    const ROOT_TARGET: u32 = 0x228;
    const TARGET_FLAGS: u32 = 0x4c8;
    let list = unsafe { ((this + CHILD_LIST) as *const u32).read_unaligned() };
    let count = lf_checker_rt::callee_thiscall!(COUNT_CHILDREN, u32, list, 0);
    if (count as i32) <= 0 {
        let root = lf_checker_rt::callee_cdecl!(GET_ACTIVE_ROOT, u32, );
        if root != 0 {
            unsafe {
                let target = ((root + ROOT_TARGET) as *const u32).read_unaligned();
                let flags = (target + TARGET_FLAGS) as *mut u32;
                flags.write_unaligned(flags.read_unaligned() & !1);
            }
        }
    }
    u32::from((count as i32) > 0)
});
