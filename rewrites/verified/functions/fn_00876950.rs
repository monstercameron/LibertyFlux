// original: 0x00876950 crmt_extrapolate_request_initialize

/// Initialize an extrapolate request and its embedded source. Set the base
/// and observer vtables, clear the observer links, call the shared setup hook
/// with the embedded source at +0x04, clear elapsed ticks, and copy five
/// stack words into the request: one f32 bit pattern at +0x18, two dwords at
/// +0x1c/+0x20, and the low bytes of the final two words at +0x24/+0x25.
/// Install the concrete request vtable and return `this`.
lf_checker_rt::export!(thiscall, rw_00876950(this: u32, weight_bits: u32, elapsed_value: u32, frame_value: u32, mode_word: u32, state_word: u32) -> u32 {
    const SUBOBJECT: u32 = 0x04;
    const SOURCE_VTABLE: u32 = 0x00fe7fc8;
    const OBSERVER_VTABLE: u32 = 0x00fe7fb4;
    const REQUEST_VTABLE: u32 = 0x00fe8214;
    const SETUP_HOOK_VA: u32 = 0x00fe7fb8;

    unsafe {
        let source = this + SUBOBJECT;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(SOURCE_VTABLE));
        ((source + 4) as *mut u32).write_unaligned(0);
        (source as *mut u32).write_unaligned(lf_checker_rt::relocated(OBSERVER_VTABLE));
        ((source + 8) as *mut u32).write_unaligned(0);
        ((source + 0x0c) as *mut u32).write_unaligned(0);

        let hook_target = lf_checker_rt::global::<u32>(SETUP_HOOK_VA).read_unaligned();
        let setup: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(hook_target as usize);
        let _ = setup(source);

        ((this + 0x14) as *mut u32).write_unaligned(0);
        ((this + 0x1c) as *mut u32).write_unaligned(elapsed_value);
        ((this + 0x24) as *mut u8).write(mode_word as u8);
        ((this + 0x20) as *mut u32).write_unaligned(frame_value);
        ((this + 0x25) as *mut u8).write(state_word as u8);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(REQUEST_VTABLE));
        ((this + 0x18) as *mut u32).write_unaligned(weight_bits);
        this
    }
});
