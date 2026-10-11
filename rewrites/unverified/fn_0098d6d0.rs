// original: 0x0098D6D0 audVehicleAudioEntity::horn_mode_is_one

/// Follow the object pointer at `this + 0x9E0`. Return one when its low two
/// flag bits at offset `+7` equal one; return zero when the pointer is null or
/// the masked value differs. The byte mask is unsigned and the result is in
/// AL.
///
/// This is a `thiscall` leaf with no stack arguments.
lf_checker_rt::export!(thiscall, rw_aud_vehicle_horn_mode_is_one(this: u32) -> u32 {
    unsafe {
        const VEHICLE_OBJECT: u32 = 0x9E0;
        const OBJECT_FLAGS: u32 = 7;
        let object = (this.wrapping_add(VEHICLE_OBJECT) as *const u32).read_unaligned();
        if object == 0 {
            return 0;
        }
        let mode = (object.wrapping_add(OBJECT_FLAGS) as *const u8).read() & 3;
        u32::from(mode == 1)
    }
});
