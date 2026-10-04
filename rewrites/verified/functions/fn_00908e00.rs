// original: 0x00908e00 radar_range_float_update
/// Update the radar-range float global.
///
/// When a fixed range count is configured and the override flag byte is
/// clear, stores the count as a float. Otherwise stores the argument float,
/// scaled by the configured ratio when the mode allows it and the ratio is
/// neither 0 nor 1 (either comparison treats NaN as "scale"). Returns the
/// configured count; on the comparing paths AH carries the EFLAGS left by the
/// last float comparison (the original funnels both through lahf).
export!(cdecl, rw_00908E00(fbits: u32, flag: u32) -> u32 {
    unsafe {
        let n = *global::<i32>(0x11E6234);
        // AH of the return: the count's own second byte unless a comparison
        // overwrote it via lahf (ucomiss zeroes SF/AF/OF, so these bytes are
        // exact: equal 0x42, unordered 0x47, greater 0x02, less 0x03).
        let mut ah = (n as u32) & 0xFF00;
        if n != 0 && (flag & 0xFF) == 0 {
            *global::<f32>(0x118F4B0) = n as f32;
        } else {
            let f = f32::from_bits(fbits);
            *global::<f32>(0x118F4B0) = f;
            if *global::<i32>(0x11D6FD4) >= 2 && *global::<u8>(0x11609F6) == 0 {
                let r = *global::<f32>(0x11E6238);
                if r == 0.0 {
                    ah = 0x4200;
                } else if r == 1.0 {
                    ah = 0x4200;
                } else {
                    ah = if r.is_nan() {
                        0x4700
                    } else if r > 1.0 {
                        0x0200
                    } else {
                        0x0300
                    };
                    *global::<f32>(0x118F4B0) = r * f;
                }
            }
        }
        ((n as u32) & 0xFFFF00FF) | ah
    }
});
