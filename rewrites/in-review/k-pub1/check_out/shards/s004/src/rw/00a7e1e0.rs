// original: 0x00a7e1e0 linked_ctor_b_default
/// Default constructor: base-construct, clear the code word and the link,
/// stamp our vtable. Returns the object.
export!(thiscall, rw_00a7e1e0(this: *mut u8) -> u32 {
    unsafe {
        __lf_callee_thiscall!(1, u32, this as u32);
        st16(this, 0x18, 0);
        st32(this, 0, relocated(0xEA10A4));
        st32(this, 0x14, 0);
        this as u32
    }
});

unsafe fn st16(obj: *mut u8, off: usize, v: u16) {
    *((obj.add(off)) as *mut u16) = v;
}

unsafe fn st32(obj: *mut u8, off: usize, v: u32) {
    *((obj.add(off)) as *mut u32) = v;
}
