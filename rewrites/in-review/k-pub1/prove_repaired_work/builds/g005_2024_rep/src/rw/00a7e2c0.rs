// original: 0x00a7e2c0 leaf_ctor
/// Leaf constructor over the root base: stamp our vtable only.
export!(thiscall, rw_00a7e2c0(this: *mut u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        st32(this, 0, relocated(0xEA1A24));
        this as u32
    }
});

unsafe fn st32(obj: *mut u8, off: usize, v: u32) {
    *((obj.add(off)) as *mut u32) = v;
}
