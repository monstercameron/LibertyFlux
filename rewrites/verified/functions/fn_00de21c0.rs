// original: 0x00DE21C0 UIScrollingMenu::vf110

/// When the low flag byte at `+0x215` is set, request a measurement record, pass its low word as the selector, and query the delegated status through slot `+0x21C`. Selector one reports the prior index and dispatches the first notification; selector two queries status again, reads a 16-bit list count from slot `+0x1D0`, and dispatches the second notification when the signed index is below that count. Both notification paths finish with the shared virtual update call. The flag-clear early return preserves incoming EAX and is outside this proof's input set.
lf_checker_rt::export!(thiscall, rw_00de21c0(this_ptr: u32) -> u32 {
    const FLAG_OFFSET: u32 = 0x215;
    const DELEGATE_OFFSET: u32 = 0x1E0;
    const LOOKUP_CONTEXT_VA: u32 = 0x019D2E08;
    const NOTIFICATION_CONTEXT_VA: u32 = 0x01176888;
    const MEASUREMENT_LOOKUP: u32 = 1;
    const DELEGATE_STATUS: u32 = 2;
    const NOTIFICATION: u32 = 3;
    const LIST_QUERY: u32 = 4;
    const APPLY_INDEX: u32 = 5;
    const FINAL_UPDATE: u32 = 6;
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    unsafe fn read_u16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
    if unsafe { *((this_ptr.wrapping_add(FLAG_OFFSET)) as *const u8) } == 0 { return 0; }
    let delegate = unsafe { read_u32(this_ptr.wrapping_add(DELEGATE_OFFSET)) };
    let lookup_context = lf_checker_rt::relocated(LOOKUP_CONTEXT_VA);
    let record = lf_checker_rt::callee_thiscall!(MEASUREMENT_LOOKUP, u32, lookup_context, 0xF00, 1);
    let selector = unsafe { read_u32(record) };
    let _measurement_high = unsafe { read_u32(record.wrapping_add(4)) };
    let _measurement_tag = unsafe { read_u32(record.wrapping_add(8)) };
    let prior_index = lf_checker_rt::callee_thiscall!(DELEGATE_STATUS, u32, delegate);
    if selector == 1 {
        if prior_index == 0 { return selector; }
        let notify_context = lf_checker_rt::relocated(NOTIFICATION_CONTEXT_VA);
        let first_notice = lf_checker_rt::relocated(0x00EFD578);
        let _notice_result = lf_checker_rt::callee_thiscall!(NOTIFICATION, u32, notify_context, first_notice);
        let _apply_result = lf_checker_rt::callee_thiscall!(APPLY_INDEX, u32, this_ptr, prior_index.wrapping_sub(1), 1);
        return lf_checker_rt::callee_thiscall!(FINAL_UPDATE, u32, lookup_context, 1);
    }
    if selector == 2 {
        let next_index = lf_checker_rt::callee_thiscall!(DELEGATE_STATUS, u32, delegate).wrapping_add(1);
        let list = lf_checker_rt::callee_thiscall!(LIST_QUERY, u32, delegate);
        let item_count = unsafe { read_u16(list.wrapping_add(4)) } as u32;
        if (next_index as i32) >= (item_count as i32) { return item_count; }
        let notify_context = lf_checker_rt::relocated(NOTIFICATION_CONTEXT_VA);
        let second_notice = lf_checker_rt::relocated(0x00EFD598);
        let _notice_result = lf_checker_rt::callee_thiscall!(NOTIFICATION, u32, notify_context, second_notice);
        let _apply_result = lf_checker_rt::callee_thiscall!(APPLY_INDEX, u32, this_ptr, next_index, 1);
        return lf_checker_rt::callee_thiscall!(FINAL_UPDATE, u32, lookup_context, 1);
    }
    selector
});
