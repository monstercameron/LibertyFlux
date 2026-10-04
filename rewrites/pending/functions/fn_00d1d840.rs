// original: 0x00d1d840 CTaskComplexSlideIntoCover::vf0
// Scalar deleting destructor: runs the class destructor, then frees the
// object through the task pool when the low bit of the argument is set.
// Returns the object.
export!(thiscall, rw_00d1d840(this_ptr: u32, free_flag: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this_ptr);
        if (free_flag & 1) != 0 {
            let mgr = *(global::<u32>(0x0167E2A0));
            callee_thiscall!(2, u32, mgr, this_ptr);
        }
        this_ptr
    }
});
