// original: 0x00a7e330 dtor_b
/// Destructor: stamp our vtable, release both held peers through the unlink
/// helper (clearing each slot) when non-null, then tail-call the base
/// destructor and return its answer.
export!(thiscall, rw_00a7e330(this: *mut u8) -> u32 {
    unsafe {
        st32(this, 0, relocated(0xEA11F4));
        let first = (this.add(0x1C)) as *mut u32;
        if *first != 0 {
            callee_thiscall!(1, u32, *first, first as u32);
            *first = 0;
        }
        let second = (this.add(0x20)) as *mut u32;
        if *second != 0 {
            callee_thiscall!(1, u32, *second, second as u32);
            *second = 0;
        }
        callee_thiscall!(2, u32, this as u32)
    }
});
