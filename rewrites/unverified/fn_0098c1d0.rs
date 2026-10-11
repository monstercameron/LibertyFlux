// original: 0x0098C1D0 audVehicleAudioEntity::road_noise_state_get

/// Return the road-noise state word when the enable bit at `this + 0x164` is
/// set, and zero otherwise. The test is on bit 0 of one byte; the returned
/// state is a full 32-bit word at `this + 0xD0`.
///
/// This is a `thiscall` leaf with no stack arguments.
lf_checker_rt::export!(thiscall, rw_aud_vehicle_road_noise_state_get(this: u32) -> u32 {
    unsafe {
        const NOISE_FLAGS: u32 = 0x164;
        const ROAD_NOISE_STATE: u32 = 0xD0;
        let flags = (this.wrapping_add(NOISE_FLAGS) as *const u8).read();
        if flags & 1 == 0 {
            0
        } else {
            (this.wrapping_add(ROAD_NOISE_STATE) as *const u32).read_unaligned()
        }
    }
});
