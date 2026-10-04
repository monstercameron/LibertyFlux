// original: 0x006641a0 init_pair_record
/// Small record initializer: zero the header, init both sub-objects.
///
/// thiscall/0, returns `this`. Zeroes the header words and runs the two
/// embedded sub-object constructors.
export!(thiscall, rw_006641a0(this_ptr: u32) -> u32 {
    unsafe {
        ((this_ptr) as *mut u32).write(0);
        ((this_ptr + 4) as *mut u32).write(0);
        ((this_ptr + 0x10) as *mut u32).write(0);
        ((this_ptr + 0x14) as *mut u32).write(0);
        callee_thiscall!(1, u32, this_ptr.wrapping_add(0x18));
        callee_thiscall!(2, u32, this_ptr.wrapping_add(8));
        this_ptr
    }
});
