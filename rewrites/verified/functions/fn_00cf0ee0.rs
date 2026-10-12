// original: 0x00CF0EE0 stat_ms_to_days

/// Convert a 64-bit millisecond count to elapsed whole days.
///
/// The cdecl input is an unsigned 64-bit word split into low and high dwords.
/// The original passes it and the constant divisor 1,000 to the intercepted
/// 64-bit unsigned divider, then applies signed 32-bit truncating divisions
/// by 60, 60, and 24 to the divider's low result. The returned value is that
/// final signed day count in EAX.
lf_checker_rt::export!(cdecl, rw_fn_00cf0ee0(milliseconds_low: u32, milliseconds_high: u32) -> u32 {
    const MILLISECONDS_PER_SECOND: u32 = 1_000;
    const SECONDS_PER_MINUTE: i32 = 60;
    const MINUTES_PER_HOUR: i32 = 60;
    const HOURS_PER_DAY: i32 = 24;
    let divided: u64 = lf_checker_rt::callee_stdcall!(
        1, u64, milliseconds_low, milliseconds_high, MILLISECONDS_PER_SECOND, 0u32
    );
    let elapsed_seconds = divided as u32 as i32;
    let elapsed_minutes = elapsed_seconds / SECONDS_PER_MINUTE;
    let elapsed_hours = elapsed_minutes / MINUTES_PER_HOUR;
    (elapsed_hours / HOURS_PER_DAY) as u32
});
