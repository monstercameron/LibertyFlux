// original: 0x00b53fb0 CAtdNodeAnimPlayerPooledObject::vf0
/// Scalar deleting destructor: stamps the node vtable, destroys the
/// embedded player member, stamps the base vtable, and returns `this`
/// to its pool when bit 0 of the flag is set. Returns `this`.
export!(thiscall, rw_00b53fb0(this: *mut u8, flag: u32) -> u32 {
    unsafe {
        *(this as *mut u32) = relocated(0xEAF3A0);
        callee_thiscall!(1, u32, (this as u32).wrapping_add(4));
        *(this as *mut u32) = relocated(0xEAF398);
        if flag & 1 != 0 {
            let pool = *global::<u32>(0x1669D5C);
            callee_thiscall!(2, u32, pool, this as u32);
        }
        this as u32
    }
});
