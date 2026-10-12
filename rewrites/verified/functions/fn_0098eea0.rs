// original: 0x0098EEA0 NativeImpl_HELI_AUDIO_SHOULD_SKIP_STARTUP

/// If the startup-skip byte at `this + 0xB04` is zero, store the low byte of
/// the one stack argument at `this + 0xB17`. A nonzero guard leaves the field
/// untouched. This `thiscall` setter removes four argument bytes from the
/// stack; its return register is not part of the contract.
lf_checker_rt::export!(thiscall, rw_nativeimpl_heli_audio_should_skip_startup(this: u32, value: u32) -> () {
    unsafe {
        const STARTUP_SKIP_GUARD: u32 = 0xB04;
        const STARTUP_SKIP_VALUE: u32 = 0xB17;
        let guard = (this.wrapping_add(STARTUP_SKIP_GUARD) as *const u8).read();
        if guard == 0 {
            (this.wrapping_add(STARTUP_SKIP_VALUE) as *mut u8).write(value as u8);
        }
    }
});
