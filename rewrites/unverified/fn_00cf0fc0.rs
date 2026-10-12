// original: 0x00CF0FC0 stat_ms_to_minutes

/// Return the minute component within the current hour for a 64-bit
/// millisecond count.
///
/// The cdecl input is split into low and high dwords and divided by 1,000 by
/// the intercepted 64-bit unsigned helper. Its low result is treated as a
/// signed 32-bit count; the function divides by 60 twice and subtracts the
/// whole-hour contribution, returning the signed minute remainder in EAX.
lf_checker_rt::export!(cdecl, rw_fn_00cf0fc0(milliseconds_low: u32, milliseconds_high: u32) -> u32 {
    const MILLISECONDS_PER_SECOND: u32 = 1_000;
    const SECONDS_PER_MINUTE: i32 = 60;
    const MINUTES_PER_HOUR: i32 = 60;
    let divided: u64 = lf_checker_rt::callee_stdcall!(
        1, u64, milliseconds_low, milliseconds_high, MILLISECONDS_PER_SECOND, 0u32
    );
    let elapsed_seconds = divided as u32 as i32;
    let elapsed_minutes = elapsed_seconds / SECONDS_PER_MINUTE;
    let elapsed_hours = elapsed_minutes / MINUTES_PER_HOUR;
    elapsed_minutes.wrapping_sub(elapsed_hours.wrapping_mul(MINUTES_PER_HOUR)) as u32
});
