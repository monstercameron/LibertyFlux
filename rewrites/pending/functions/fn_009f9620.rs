// original: 0x009f9620 counters_reset5
/// Counter block reset, five dwords (cdecl/0 -> void). See `rw_s18f4`.
export!(cdecl, rw_s18f11() -> u32 {
    unsafe {
        *global::<u32>(0x12B6270) = 0;
        *global::<u32>(0x12B6274) = 0;
        *global::<u32>(0x12B6278) = 0;
        *global::<u32>(0x12B628C) = 0;
        *global::<u32>(0x12B6284) = 0;
        0
    }
});
