// original: 0x00A18FA0 CCamFollowVehicle::read_look_scale
/// If the feature byte is clear, leaves both caller outputs untouched. When
/// enabled, compares the camera speed with an image float using COMISS order:
/// unordered or speed >= threshold stores zero; speed below threshold stores
/// the alternate image float. The second output is always cleared on this path.
lf_checker_rt::export!(thiscall, rw_00A18FA0(camera: u32, scale_out: u32, status_out: u32) -> u32 {
    unsafe {
        const FEATURE_ENABLED: u32 = 0x012B_D1C8;
        const SPEED: u32 = 0x190;
        const PRIMARY_THRESHOLD: u32 = 0x00FE_87B4;
        const ACTIVE_SCALE: u32 = 0x00FE_8BC4;

        if lf_checker_rt::global::<u8>(FEATURE_ENABLED).read_volatile() == 0 {
            return 0;
        }
        let threshold = lf_checker_rt::global::<f32>(PRIMARY_THRESHOLD).read_volatile();
        let speed = (camera.wrapping_add(SPEED) as *const f32).read_unaligned();
        let scale = if threshold > speed {
            lf_checker_rt::global::<f32>(ACTIVE_SCALE).read_volatile()
        } else {
            0.0
        };
        (scale_out as *mut f32).write_unaligned(scale);
        (status_out as *mut u32).write_unaligned(0);
        0
    }
});
