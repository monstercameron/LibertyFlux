// original: 0x00DE14B0 scrolling_menu_apply_child_state

/// Fetch the child list from the delegate, and unless the control byte is nonzero, visit the unsigned 16-bit child count in order. Each child receives a no-argument query followed by an update with argument zero. Store the control byte at `+0x215`. EAX keeps the upper 24 bits from the getter when no child is visited, or from the one-past-end child-array pointer after a nonempty traversal; its low byte is replaced with the control byte. `thiscall`, one stack word.
lf_checker_rt::export!(thiscall, rw_00de14b0(this_ptr: u32, control_word: u32) -> u32 {
    const DELEGATE_OFFSET: u32 = 0x1E0;
    const FLAG_OFFSET: u32 = 0x215;
    const LIST_GETTER: u32 = 1;
    const CHILD_QUERY: u32 = 2;
    const CHILD_UPDATE: u32 = 3;
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }
    let delegate = unsafe { read_u32(this_ptr.wrapping_add(DELEGATE_OFFSET)) };
    let mut result = lf_checker_rt::callee_thiscall!(LIST_GETTER, u32, delegate);
    let control = control_word as u8;
    if control == 0 {
        let list = result;
        let children = unsafe { read_u32(list) };
        let count = unsafe { read_u16(list.wrapping_add(4)) } as u32;
        if count != 0 {
            let one_past_end = children.wrapping_add(count.wrapping_mul(4));
            for index in 0..count {
                let child = unsafe { read_u32(children.wrapping_add(index.wrapping_mul(4))) };
                let _query_result = lf_checker_rt::callee_thiscall!(CHILD_QUERY, u32, child);
                let _update_result = lf_checker_rt::callee_thiscall!(CHILD_UPDATE, u32, child, 0);
            }
            result = one_past_end;
        }
    }
    unsafe { ((this_ptr.wrapping_add(FLAG_OFFSET)) as *mut u8).write(control) };
    (result & 0xFFFF_FF00) | u32::from(control)
});
