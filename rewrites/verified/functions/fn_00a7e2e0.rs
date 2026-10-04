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
            callee_thiscall!(1, u32, held, slot as u32);
            *slot = 0;
        }
        st32(this, 0, relocated(0xEA105C));
        callee_thiscall!(2, u32, this as u32)
    }
});
