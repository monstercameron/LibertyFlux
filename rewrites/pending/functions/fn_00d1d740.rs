// original: 0x00d1d740 CTaskComplexSlideIntoCover::CTaskComplexSlideIntoCover
// Constructor: runs the base task constructor, stamps this class's vtable,
// copies two words, a 3-word position and a float from the arguments, and
// zeroes a spare slot. Returns the object.
export!(thiscall, rw_00d1d740(this_ptr: u32, a: u32, b: u32, pos: u32, f: u32, flag: u32, extra: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this_ptr);
        *((this_ptr.wrapping_add(0x14)) as *mut u32) = a;
        *((this_ptr.wrapping_add(0x18)) as *mut u32) = b;
        *(this_ptr as *mut u32) = relocated(0x00EE0ACC);
        *((this_ptr.wrapping_add(0x20)) as *mut u32) = *(pos as *const u32);
        *((this_ptr.wrapping_add(0x24)) as *mut u32) = *((pos.wrapping_add(4)) as *const u32);
        *((this_ptr.wrapping_add(0x28)) as *mut u32) = *((pos.wrapping_add(8)) as *const u32);
        *((this_ptr.wrapping_add(0x3C)) as *mut u8) = (flag & 0xFF) as u8;
        *((this_ptr.wrapping_add(0x40)) as *mut u32) = extra;
        *((this_ptr.wrapping_add(0x30)) as *mut u32) = f;
        *((this_ptr.wrapping_add(0x38)) as *mut u32) = 0;
        this_ptr
    }
});
