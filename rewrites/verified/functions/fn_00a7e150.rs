// original: 0x00a7e150 payload_ctor
/// One-argument constructor: base-construct, store the payload, stamp our
/// vtable. Returns the object.
export!(thiscall, rw_00a7e150(this: *mut u8, a: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        st32(this, 0x18, a);
        st32(this, 0, relocated(0xEA10EC));
        this as u32
    }
});
