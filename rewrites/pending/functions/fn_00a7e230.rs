// original: 0x00a7e230 mid_base_ctor
/// Mid-level base constructor: run the root constructor, stamp our vtable,
/// clear the trailing link word. Returns the object.
export!(thiscall, rw_00a7e230(this: *mut u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        st32(this, 0, relocated(0xEA105C));
        st32(this, 0x14, 0);
        this as u32
    }
});
