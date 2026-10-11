// original: 0x00DE1B10 scrolling_menu_dispatch_if_no_alert

/// Call the alert predicate. When its low byte is nonzero, clear only that byte and return; otherwise load the delegated object at `+0x1E0` and return the result of its virtual slot `+0x234`.
lf_checker_rt::export!(thiscall, rw_00de1b10(this_ptr: u32) -> u32 {
    const ALERT_PREDICATE: u32 = 1;
    const DELEGATE_OFFSET: u32 = 0x1E0;
    const DELEGATE_SLOT: u32 = 2;
    let alert = lf_checker_rt::callee_thiscall!(ALERT_PREDICATE, u32, this_ptr);
    if alert as u8 != 0 {
        alert & 0xFFFF_FF00
    } else {
        let delegate = unsafe { ((this_ptr.wrapping_add(DELEGATE_OFFSET)) as *const u32).read_unaligned() };
        lf_checker_rt::callee_thiscall!(DELEGATE_SLOT, u32, delegate)
    }
});
