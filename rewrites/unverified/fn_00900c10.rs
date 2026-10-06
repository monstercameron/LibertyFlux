// original: 0x00900c10 input_kick_when_idle (proposed)
/// Kick the shared device twice when this object is idle.
///
/// `this` points to the object. When the state byte at `+0x17` is set
/// nothing happens and entry `eax` is returned. Otherwise the shared device
/// (read from the static word) is passed to the wake callee and then,
/// with argument 1, to the kick callee (both thiscalls); the kick answer
/// is returned. Thiscall with no stack arguments.
export!(thiscall, rw_00900c10(this: u32) -> u32 {
    unsafe {
        /// Shared device word (file VA).
        const DEVICE: u32 = 0x0118F4A8;
        const STATE_OFF: u32 = 0x17;
        const WAKE_ID: u32 = 1;
        const KICK_ID: u32 = 2;
        if (this.wrapping_add(STATE_OFF) as *const u8).read() != 0 {
            return 0;
        }
        let dev = (global::<u32>(DEVICE)).read_unaligned();
        let _: u32 = callee_thiscall!(WAKE_ID, u32, dev);
        callee_thiscall!(KICK_ID, u32, dev, 1u32)
    }
});
