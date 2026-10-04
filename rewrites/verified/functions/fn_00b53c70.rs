// original: 0x00b53c70 pool_array_ctor
/// Constructor: stamps the vtable, constructs the head and tail
/// sub-objects, constructs 50 array elements back to front, restamps the
/// vtable, and returns the last element-construction answer.
export!(thiscall, rw_00b53c70(this: *mut u8) -> u32 {
    unsafe {
        *(this as *mut u32) = relocated(0xEAF3D0);
        callee_thiscall!(1, u32, this as u32);
        callee_thiscall!(2, u32, (this as u32).wrapping_add(0x3F4));
        let mut e = (this as u32).wrapping_add(0x3EC);
        let mut i = 0x31u32;
        let mut last = 0u32;
        loop {
            e = e.wrapping_sub(0x14);
            last = callee_thiscall!(3, u32, e);
            if i == 0 {
                break;
            }
            i -= 1;
        }
        *(this as *mut u32) = relocated(0xEAF2EC);
        last
    }
});
