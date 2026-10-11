// original: 0x0098EE70 NativeImpl_ENABLE_GPS_IN_VEHICLE

/// Store the low byte of the one stack argument in the GPS voice flag at
/// `this + 0xB02`. The original is a `thiscall` setter that removes four
/// argument bytes from the stack; its return register is not part of the
/// contract.
lf_checker_rt::export!(thiscall, rw_nativeimpl_enable_gps_in_vehicle(this: u32, enabled: u32) -> () {
    unsafe {
        const GPS_VOICE_ENABLED: u32 = 0xB02;
        (this.wrapping_add(GPS_VOICE_ENABLED) as *mut u8).write(enabled as u8);
    }
});
