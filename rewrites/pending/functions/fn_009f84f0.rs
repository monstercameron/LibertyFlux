// original: 0x009f84f0 counters_reset4
/// Counter block reset, four dwords (cdecl/0 -> void).
///
/// The original leaves an indeterminate value in eax (whatever the caller had
/// there), so the contract compares no return channel.
export!(cdecl, rw_s18f4() -> u32 {
    unsafe {
        *global::<u32>(0x12B6270) = 0;
        *global::<u32>(0x12B6274) = 0;
        *global::<u32>(0x12B6278) = 0;
        *global::<u32>(0x12B628C) = 0;
        0
    }
});
