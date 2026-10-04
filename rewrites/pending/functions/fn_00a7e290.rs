// original: 0x00a7e290 ext_ctor_default
/// Default constructor: base-construct, stamp our vtable, mark the two id
/// words unset (-1), clear the trailing word. Returns the object.
export!(thiscall, rw_00a7e290(this: *mut u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        st32(this, 0, relocated(0xEA119C));
        st32(this, 0x24, 0xFFFF_FFFF);
        st32(this, 0x28, 0xFFFF_FFFF);
        st32(this, 0x2C, 0);
        this as u32
    }
});
