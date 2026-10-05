// original: 0x0088D560 rage::audVoiceDSound::vf5

/// Shut down this voice's output device, slot 0x48 of the device table.
///
/// `this` points to the voice object; the device object sits at `+0x90`.
/// The device's function table is read from its first word and the entry at
/// `+0x48` is invoked with the device object as its single stack argument
/// (the callee pops it). The callee's answer is left in `eax` and returned.
///
/// Original: 0x0088D560 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_0088D560(this: u32) -> u32 {
    unsafe {
        const VOICE_DEVICE: u32 = 0x90;
        const DEVICE_SLOT: u32 = 0x48;
        let obj = ((this + VOICE_DEVICE) as *const u32).read_unaligned();
        let table = (obj as *const u32).read_unaligned();
        let target: extern "stdcall" fn(u32) -> u32 = core::mem::transmute(
            ((table + DEVICE_SLOT) as *const u32).read_unaligned() as usize
        );
        target(obj)
    }
});
