// original: 0x00ab6ce0 stream_set_ready (proposed)

/// Publish the streaming ready flag and clear the pending count.
///
/// Writes 1 to the flag byte and 0 to the pending word in the loader's
/// global block. No arguments, no return value.
///
/// Original: 0x00ab6ce0 (cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_00ab6ce0() -> u32 {
    unsafe {
        const FLAG: u32 = 0x0150_E0D1;
        const PENDING: u32 = 0x0150_E0D4;
        (lf_checker_rt::global::<u8>(FLAG)).write(1);
        (lf_checker_rt::global::<u32>(PENDING)).write_unaligned(0);
        0
    }
});
