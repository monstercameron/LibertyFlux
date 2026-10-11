// original: 0x00DE1A90 scrolling_menu_any_child_alert_active

/// Query the first child alert predicate and query the second only when the first result's low byte is zero. A successful result has its low byte set to one with its upper 24 bits preserved; if neither succeeds, return the full second result.
lf_checker_rt::export!(thiscall, rw_00de1a90(this_ptr: u32) -> u32 {
    const FIRST_CHILD_ALERT: u32 = 1;
    const SECOND_CHILD_ALERT: u32 = 2;
    let mut result = lf_checker_rt::callee_thiscall!(FIRST_CHILD_ALERT, u32, this_ptr);
    if result as u8 == 0 {
        result = lf_checker_rt::callee_thiscall!(SECOND_CHILD_ALERT, u32, this_ptr);
    }
    if result as u8 != 0 { result = (result & 0xFFFF_FF00) | 1; }
    result
});
