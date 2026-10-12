// original: 0x00CF0F30 stat_ms_to_hours

/// Return the hour component within the current day for a 64-bit millisecond
/// count.
///
/// The cdecl input is split into low and high dwords and divided by 1,000 by
/// the intercepted 64-bit unsigned helper. The helper's low result is then
/// treated as a signed 32-bit count: successive truncating divisions by 60,
/// 60, and 24 produce whole days, which are removed from whole hours. EAX
/// receives the signed hour remainder.
lf_checker_rt::export!(cdecl, rw_fn_00cf0f30(milliseconds_low: u32, milliseconds_high: u32) -> u32 {
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
    let elapsed_days = elapsed_hours / HOURS_PER_DAY;
    elapsed_hours.wrapping_sub(elapsed_days.wrapping_mul(HOURS_PER_DAY)) as u32
});
