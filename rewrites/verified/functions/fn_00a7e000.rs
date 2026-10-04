// original: 0x00a7e000 angle_ctor_default
/// Default constructor: base-construct, stamp our vtable, clear the timer
/// words, the angle and the flag word. Returns the object.
export!(thiscall, rw_00a7e000(this: *mut u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        st32(this, 0, relocated(0xEA18BC));
        st32(this, 0x18, 0);
        st32(this, 0x1C, 0);
        st32(this, 0x20, 0);
        st16(this, 0x24, 0);
        this as u32
    }
});
