// original: 0x00a7e130 linked_ctor_default
/// Default constructor: base-construct, clear the code word and the payload,
/// stamp our vtable. Returns the object.
export!(thiscall, rw_00a7e130(this: *mut u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        st16(this, 0x20, 0);
        st32(this, 0, relocated(0xEA1144));
        st32(this, 0x1C, 0);
        this as u32
    }
});
