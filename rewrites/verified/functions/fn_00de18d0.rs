// original: 0x00DE18D0 scrolling_menu_get_clamped_child_count

/// Read the delegated object, then compare its unsigned child count with the wrapping bound `field_1f8 - 1 + field_200`. If the bound is below the count, return the bound; otherwise query the count again and return that result minus one with wrapping arithmetic. The delegated virtual call takes no stack arguments.
lf_checker_rt::export!(thiscall, rw_00de18d0(this_ptr: u32) -> u32 {
    const DELEGATE_OFFSET: u32 = 0x1E0;
    const BOUND_BASE_OFFSET: u32 = 0x1F8;
    const BOUND_ADDEND_OFFSET: u32 = 0x200;
    const CHILD_COUNT: u32 = 1;
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    let delegate = unsafe { read_u32(this_ptr.wrapping_add(DELEGATE_OFFSET)) };
    let bound = unsafe { read_u32(this_ptr.wrapping_add(BOUND_BASE_OFFSET)) }
        .wrapping_sub(1)
        .wrapping_add(unsafe { read_u32(this_ptr.wrapping_add(BOUND_ADDEND_OFFSET)) });
    let count = lf_checker_rt::callee_thiscall!(CHILD_COUNT, u32, delegate);
    if bound < count {
        bound
    } else {
        lf_checker_rt::callee_thiscall!(CHILD_COUNT, u32, delegate).wrapping_sub(1)
    }
});
