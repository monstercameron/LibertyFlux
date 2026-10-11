// original: 0x00C79390 streaming_counters_reset
/// Clear the three shared streaming counters.
///
/// This zero-argument cdecl helper writes zero to three separate 32-bit
/// global words. It performs no calls and has no defined return channel; the
/// machine body leaves the incoming EAX value untouched.
///
#[unsafe(no_mangle)]
pub extern "cdecl" fn rw_00c79390() {
        unsafe { lf_checker_rt::global::<u32>(0x016ECCF0).write_volatile(0); }
        unsafe { lf_checker_rt::global::<u32>(0x016ECCFC).write_volatile(0); }
        unsafe { lf_checker_rt::global::<u32>(0x016ECCF8).write_volatile(0); }
}
