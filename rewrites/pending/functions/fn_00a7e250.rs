// original: 0x00a7e250 ext_ctor
/// Five-argument constructor: base-construct with the fourth and first
/// arguments (pushed first-fourth, so the callee sees them swapped), store
/// the remaining three, stamp our vtable. Returns the object.
export!(thiscall, rw_00a7e250(
    this: *mut u8,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32, a3, a0);
        st32(this, 0x24, a1);
        st32(this, 0x28, a2);
        st32(this, 0x2C, a4);
        st32(this, 0, relocated(0xEA119C));
        this as u32
    }
});
