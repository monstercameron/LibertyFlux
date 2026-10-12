// original: 0x0098EE80 audVehicleAudioEntity::set_gps_voice

/// Store the low byte of the one stack argument, clamped to 4, at
/// `this + 0xB03`. Values are zero-extended from the argument byte before the
/// signed comparison, so every byte above 4 also produces 4.
///
/// This is a `thiscall` setter that removes four argument bytes from the
/// stack; its return register is not part of the contract.
lf_checker_rt::export!(thiscall, rw_aud_vehicle_set_gps_voice(this: u32, voice: u32) -> () {
    unsafe {
        const GPS_VOICE_INDEX: u32 = 0xB03;
        const MAX_VOICE_INDEX: u32 = 4;
        let voice = voice & 0xFF;
        let clamped = if voice < MAX_VOICE_INDEX { voice } else { MAX_VOICE_INDEX };
        (this.wrapping_add(GPS_VOICE_INDEX) as *mut u8).write(clamped as u8);
    }
});
