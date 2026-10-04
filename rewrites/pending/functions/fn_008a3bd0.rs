// original: 0x008a3bd0 rage::audSwitchSound::audSwitchSound
/// Construct an `audSwitchSound`: base init, vtable, field defaults.
///
/// Original 0x008A3BD0 (`thiscall/0`): runs the base constructor, installs
/// the class vtable, writes the default words/bytes, returns `this`.
export!(thiscall, rw_008a3bd0(this: u32) -> u32 {    callee_thiscall!(1, u32, this);
    unsafe {
        (this as *mut u32).write_unaligned(relocated(0xe7ae34));
        ((this + 0xb0) as *mut u32).write_unaligned(0xFFFFFFFF);
        ((this + 0xb4) as *mut u32).write_unaligned(0xFFFFFFFF);
        ((this + 0xb8) as *mut u32).write_unaligned(0x3F800000);
        ((this + 0xbc) as *mut u32).write_unaligned(0x3F800000);
        ((this + 0xc0) as *mut u32).write_unaligned(0);
        ((this + 0xc4) as *mut u16).write_unaligned(0);
        ((this + 0xf1) as *mut u16).write_unaligned(0);
        ((this + 0xf0) as *mut u8).write_unaligned(0xFF);
    }
    this
});
