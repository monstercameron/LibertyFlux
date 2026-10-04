// original: 0x00a7e200 base_ctor
/// Root base constructor: set flag bits 0 and 3 at +0x8 (clearing bits 1-2),
/// stamp the vtable, and write the default header words. Returns the object.
export!(thiscall, rw_00a7e200(this: *mut u8) -> u32 {
    unsafe {
        let flags = ld32(this, 8);
        st32(this, 8, (flags & 0xFFFF_FFF9) | 9);
        st32(this, 0, relocated(0xEA1014));
        st32(this, 4, 0xC8);
        st32(this, 0xC, 0);
        st32(this, 0x10, 0);
        this as u32
    }
});
