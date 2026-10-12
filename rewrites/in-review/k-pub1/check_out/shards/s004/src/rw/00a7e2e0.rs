// original: 0x00a7e2e0 dtor_a
/// Destructor: stamp our vtable, release the held peer through the unlink
/// helper (clearing the slot) when non-null, stamp the base vtable, then
/// tail-call the base destructor and return its answer.
export!(thiscall, rw_00a7e2e0(this: *mut u8) -> u32 {
    unsafe {
        let slot = (this.add(0x1C)) as *mut u32;
        let held = *slot;
        st32(this, 0, relocated(0xEA13FC));
        if held != 0 {
            __lf_callee_thiscall!(1, u32, held, slot as u32);
            *slot = 0;
        }
        st32(this, 0, relocated(0xEA105C));
        __lf_callee_thiscall!(2, u32, this as u32)
    }
});

unsafe fn st32(obj: *mut u8, off: usize, v: u32) {
    *((obj.add(off)) as *mut u32) = v;
}
