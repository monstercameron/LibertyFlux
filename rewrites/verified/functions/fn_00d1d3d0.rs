// original: 0x00d1d3d0 CTaskComplexSeekCover::CTaskComplexSeekCover
// Full constructor: runs the base and member constructors, stamps this
// class's vtable, copies the two 3-word vectors and scalar arguments into
// their fields, folds the mode argument into the flag word at +0x90,
// toggles the latch bit at +0x94, zeroes the spare slots and finishes
// through the shared re-initialiser. Returns the object.
export!(thiscall, rw_00d1d3d0(
    this_ptr: u32, w: u32, pos_a: u32, b1: u32, b2: u32, pos_b: u32, f: u32, mode: u32,
) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this_ptr);
        *(this_ptr as *mut u32) = relocated(0x00EE0A6C);
        *((this_ptr.wrapping_add(0x20)) as *mut u32) = *(pos_a as *const u32);
        *((this_ptr.wrapping_add(0x24)) as *mut u32) = *((pos_a.wrapping_add(4)) as *const u32);
        *((this_ptr.wrapping_add(0x28)) as *mut u32) = *((pos_a.wrapping_add(8)) as *const u32);
        *((this_ptr.wrapping_add(0x30)) as *mut u32) = w;
        *((this_ptr.wrapping_add(0x5C)) as *mut u8) = (b1 & 0xFF) as u8;
        callee_thiscall!(2, u32, this_ptr.wrapping_add(0x60));
        callee_thiscall!(3, u32, this_ptr.wrapping_add(0x6C));
        *((this_ptr.wrapping_add(0x74)) as *mut u32) = 0xFFFF_FFFF;
        *((this_ptr.wrapping_add(0x80)) as *mut u32) = *(pos_b as *const u32);
        *((this_ptr.wrapping_add(0x84)) as *mut u32) = *((pos_b.wrapping_add(4)) as *const u32);
        *((this_ptr.wrapping_add(0x88)) as *mut u32) = *((pos_b.wrapping_add(8)) as *const u32);
        let flags = *((this_ptr.wrapping_add(0x90)) as *const u32);
        *((this_ptr.wrapping_add(0x90)) as *mut u32) =
            (((mode & 7) << 3) | (flags & 0xFFFF_FFC1) | 1);
        let latch = *((this_ptr.wrapping_add(0x94)) as *const u8);
        *((this_ptr.wrapping_add(0x94)) as *mut u8) = latch ^ ((latch ^ (b2 as u8)) & 1);
        *((this_ptr.wrapping_add(0x98)) as *mut u32) = f;
        *((this_ptr.wrapping_add(0xA0)) as *mut u32) = 0;
        *((this_ptr.wrapping_add(0xA4)) as *mut u32) = 0;
        *((this_ptr.wrapping_add(0xA8)) as *mut u32) = 0;
        *((this_ptr.wrapping_add(0xB4)) as *mut u32) = 0xFFFF_FFFF;
        *((this_ptr.wrapping_add(0xB8)) as *mut u32) = 0;
        *((this_ptr.wrapping_add(0xBC)) as *mut u32) = 0;
        callee_thiscall!(4, u32, this_ptr);
        this_ptr
    }
});
