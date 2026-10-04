// original: 0x00a7e1e0 linked_ctor_b_default
/// Default constructor: base-construct, clear the code word and the link,
/// stamp our vtable. Returns the object.
export!(thiscall, rw_00a7e1e0(this: *mut u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        st16(this, 0x18, 0);
        st32(this, 0, relocated(0xEA10A4));
        st32(this, 0x14, 0);
        this as u32
    }
});
