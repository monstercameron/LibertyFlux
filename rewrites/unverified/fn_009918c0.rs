// original: 0x009918C0 audVehicleAudioEntity::notify_headlamp_switch

/// Store the low byte of the one stack argument at `this + 0xB2E`, then set
/// the notification flag at `this + 0xB30` to one. This `thiscall` setter
/// removes four argument bytes from the stack; its return register is not part
/// of the contract.
lf_checker_rt::export!(thiscall, rw_aud_vehicle_notify_headlamp_switch(this: u32, state: u32) -> () {
    unsafe {
        const HEADLAMP_STATE: u32 = 0xB2E;
        const HEADLAMP_NOTIFY: u32 = 0xB30;
        (this.wrapping_add(HEADLAMP_STATE) as *mut u8).write(state as u8);
        (this.wrapping_add(HEADLAMP_NOTIFY) as *mut u8).write(1);
    }
});
