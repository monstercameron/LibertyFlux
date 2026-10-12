// original: 0x00A19FC0 CCamFollowVehicle::is_active_mode
/// Returns one only when the primary mode byte is zero and the secondary
/// mode byte is neither zero nor four.
lf_checker_rt::export!(thiscall, rw_00A19FC0(camera: u32) -> u8 {
    unsafe {
        const PRIMARY_MODE: u32 = 0x140;
        const SECONDARY_MODE: u32 = 0x22C;
        let primary = (camera.wrapping_add(PRIMARY_MODE) as *const u8).read();
        let secondary = (camera.wrapping_add(SECONDARY_MODE) as *const u8).read();
        u8::from(primary == 0 && secondary != 0 && secondary != 4)
    }
});
