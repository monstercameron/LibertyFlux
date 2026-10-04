// original: 0x00a7e170 payload_ctor_default
/// Default constructor: base-construct, stamp our vtable, clear the payload
/// word. Returns the object.
export!(thiscall, rw_00a7e170(this: *mut u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        st32(this, 0, relocated(0xEA10EC));
        st32(this, 0x18, 0);
        this as u32
    }
});
