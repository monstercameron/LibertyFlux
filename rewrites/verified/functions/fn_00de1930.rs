// original: 0x00DE1930 scrolling_menu_any_alert_active

/// Query the primary alert predicate and query the fallback only when the first result's low byte is zero. If either low byte is nonzero, set that result's low byte to one while preserving its upper 24 bits; if both are zero, return the full fallback result.
lf_checker_rt::export!(thiscall, rw_00de1930(this_ptr: u32) -> u32 {
    const PRIMARY_ALERT: u32 = 1;
    const FALLBACK_ALERT: u32 = 2;
    let mut result = lf_checker_rt::callee_thiscall!(PRIMARY_ALERT, u32, this_ptr);
    if result as u8 == 0 {
        result = lf_checker_rt::callee_thiscall!(FALLBACK_ALERT, u32, this_ptr);
    }
    if result as u8 != 0 { result = (result & 0xFFFF_FF00) | 1; }
    result
});
