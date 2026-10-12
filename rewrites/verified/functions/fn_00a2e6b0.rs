// original: 0x00A2E6B0 weather_state_reset
/// Resets the weather selection, stream-state, flag byte, cached weight,
/// and related sentinels to their initial values.
lf_checker_rt::export!(cdecl, rw_00A2E6B0() -> () {
    unsafe {
        const CURRENT_TYPE: u32 = 0x012D_DE84;
        const NEXT_TYPE: u32 = 0x012D_DE80;
        const CURRENT_INDEX: u32 = 0x012D_DE8C;
        const NEXT_INDEX: u32 = 0x012D_DE94;
        const CURRENT_WEIGHT_BITS: u32 = 0x012D_DE90;
        const REQUESTED_INDEX: u32 = 0x012D_DE88;
        const TRANSITION_TIME: u32 = 0x012D_DEDC;
        const WEATHER_FLAGS: u32 = 0x012D_DED2;
        const CACHED_WEIGHT: u32 = 0x012D_DEAC;
        const STREAM_A: u32 = 0x012D_DEEC;
        const STREAM_B: u32 = 0x012D_DEE4;
        const STREAM_C: u32 = 0x012D_DEE8;
        const TRANSITION_SENTINEL: u32 = 0x012D_DEB8;
        const EXTRA_STATE: u32 = 0x012D_DECC;
        lf_checker_rt::global::<u32>(CURRENT_TYPE).write_volatile(1);
        lf_checker_rt::global::<u32>(NEXT_TYPE).write_volatile(1);
        lf_checker_rt::global::<u32>(CURRENT_INDEX).write_volatile(0);
        lf_checker_rt::global::<u32>(NEXT_INDEX).write_volatile(0);
        lf_checker_rt::global::<u32>(CURRENT_WEIGHT_BITS).write_volatile(0);
        lf_checker_rt::global::<u32>(REQUESTED_INDEX).write_volatile(u32::MAX);
        lf_checker_rt::global::<u32>(TRANSITION_TIME).write_volatile(0);
        lf_checker_rt::global::<u8>(WEATHER_FLAGS).write_volatile(0);
        lf_checker_rt::global::<u32>(CACHED_WEIGHT).write_volatile(0);
        lf_checker_rt::global::<u32>(STREAM_A).write_volatile(0);
        lf_checker_rt::global::<u32>(STREAM_B).write_volatile(0);
        lf_checker_rt::global::<u32>(STREAM_C).write_volatile(0);
        lf_checker_rt::global::<u32>(TRANSITION_SENTINEL).write_volatile(0xBF80_0000);
        lf_checker_rt::global::<u32>(EXTRA_STATE).write_volatile(0);
    }
});
