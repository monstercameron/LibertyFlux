// original: 0x005AE5E0 sync_config_cache (proposed)

/// Refresh the cached config value when the live value changed.
///
/// Compares the cache word against the live config word; when they match,
/// nothing is queried. Otherwise the sensor callee is asked (kind 0x1d) for a
/// float through a stack slot, the float is truncated toward zero exactly
/// like `cvttss2si` (NaN, infinities and out-of-range magnitudes all yield
/// `i32::MIN`, never saturation), and the notify callee receives the
/// truncated value with (0, 0xff) and the notify object in ECX. Either way
/// the cache ends holding the live value, except that a live value of -1
/// caches as 0xfffffffe. Returns the cached value.
lf_checker_rt::export!(cdecl, rw_005AE5E0() -> u32 {
    unsafe {
        const LIVE: u32 = 0x01034_4A8;
        const CACHE: u32 = 0x0114E_3BC;
        const SENSOR_KIND: u32 = 0x1D;
        const NOTIFY_OBJ: u32 = 0x01161_5A8;
        const SENSOR: u32 = 1;
        const NOTIFY: u32 = 2;

        /// Truncate `bits` (an f32) toward zero, matching `cvttss2si`: any
        /// NaN, infinity or magnitude at or beyond 2^31 yields `i32::MIN`.
        fn cvtt(mut bits: u32) -> i32 {
            bits = core::hint::black_box(bits);
            let f = f32::from_bits(bits);
            if f.is_nan() || f >= 2147483648.0 || f < -2147483648.0 {
                return i32::MIN;
            }
            core::hint::black_box(f) as i32
        }

        let live = lf_checker_rt::global::<u32>(LIVE).read();
        if lf_checker_rt::global::<u32>(CACHE).read() != live {
            let mut slot = 0u32;
            lf_checker_rt::callee_cdecl!(SENSOR, u32, &mut slot as *mut u32 as u32, SENSOR_KIND);
            let v = cvtt(slot);
            lf_checker_rt::callee_thiscall!(
                NOTIFY,
                u32,
                lf_checker_rt::relocated(NOTIFY_OBJ),
                v as u32,
                0u32,
                0xFFu32
            );
        }
        let cur = lf_checker_rt::global::<u32>(LIVE).read();
        let cached = if cur == 0xFFFF_FFFF { 0xFFFF_FFFE } else { cur };
        lf_checker_rt::global::<u32>(CACHE).write(cached);
        cached
    }
});
