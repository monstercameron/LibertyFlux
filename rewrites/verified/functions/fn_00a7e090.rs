// original: 0x00a7e090 timerb_ctor
/// Two-argument constructor: base-construct, store both timer words, stamp
/// our vtable. Returns the object.
export!(thiscall, rw_00a7e090(this: *mut u8, a: u32, b: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        st32(this, 0x18, a);
        st32(this, 0x1C, b);
        st32(this, 0, relocated(0xEA194C));
        this as u32
    }
});
