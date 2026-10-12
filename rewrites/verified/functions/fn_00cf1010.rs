// original: 0x00CF1010 stat_ms_to_seconds

/// Return the second component within the current minute for a 64-bit
/// millisecond count.
///
/// The cdecl input is split into low and high dwords and divided by 1,000 by
/// the intercepted 64-bit unsigned helper. Its low result is treated as a
/// signed 32-bit count; dividing by 60 gives whole minutes, which are removed
/// from the seconds count. The signed remainder is returned in EAX.
lf_checker_rt::export!(cdecl, rw_fn_00cf1010(milliseconds_low: u32, milliseconds_high: u32) -> u32 {
    const MILLISECONDS_PER_SECOND: u32 = 1_000;
    const SECONDS_PER_MINUTE: i32 = 60;
    let divided: u64 = lf_checker_rt::callee_stdcall!(
        1, u64, milliseconds_low, milliseconds_high, MILLISECONDS_PER_SECOND, 0u32
    );
    let elapsed_seconds = divided as u32 as i32;
    let elapsed_minutes = elapsed_seconds / SECONDS_PER_MINUTE;
    elapsed_seconds.wrapping_sub(elapsed_minutes.wrapping_mul(SECONDS_PER_MINUTE)) as u32
});
