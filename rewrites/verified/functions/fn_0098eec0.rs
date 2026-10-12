// original: 0x0098EEC0 audVehicleAudioEntity::set_loud_radio

/// Store the low byte of the one stack argument in the loud-radio flag at
/// `this + 0xB39`. This `thiscall` setter removes four argument bytes from the
/// stack; its return register is not part of the contract.
lf_checker_rt::export!(thiscall, rw_aud_vehicle_set_loud_radio(this: u32, enabled: u32) -> () {
    unsafe {
        const LOUD_RADIO_ENABLED: u32 = 0xB39;
        (this.wrapping_add(LOUD_RADIO_ENABLED) as *mut u8).write(enabled as u8);
    }
});
