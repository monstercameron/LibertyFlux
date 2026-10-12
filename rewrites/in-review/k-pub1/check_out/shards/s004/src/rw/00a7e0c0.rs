// original: 0x00a7e0c0 timerb_ctor_default
/// Default constructor: base-construct, stamp our vtable, mark both timer
/// words unset (-1). Returns the object.
export!(thiscall, rw_00a7e0c0(this: *mut u8) -> u32 {
    unsafe {
        __lf_callee_thiscall!(1, u32, this as u32);
        st32(this, 0, relocated(0xEA194C));
        st32(this, 0x18, 0xFFFF_FFFF);
        st32(this, 0x1C, 0xFFFF_FFFF);
        this as u32
    }
});

unsafe fn st32(obj: *mut u8, off: usize, v: u32) {
    *((obj.add(off)) as *mut u32) = v;
}
