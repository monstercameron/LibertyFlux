// original: 0x00a7e030 timer_ctor
/// Three-argument constructor: base-construct, store both timer words and
/// the mode byte, stamp our vtable. Returns the object.
export!(thiscall, rw_00a7e030(this: *mut u8, a: u32, b: u32, c: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        st32(this, 0x18, a);
        st32(this, 0x1C, b);
        st8(this, 0x20, c as u8);
        st32(this, 0, relocated(0xEA1904));
        this as u32
    }
});
