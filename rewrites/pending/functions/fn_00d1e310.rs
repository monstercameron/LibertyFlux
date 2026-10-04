// original: 0x00d1e310 CTaskComplexSlideIntoCover::vf1
// Clones the task: allocates a fresh object from the task pool and runs
// the constructor on it with this object's fields (words, position,
// float and flag byte). Returns the constructor's answer (the clone), or
// null when allocation fails.
export!(thiscall, rw_00d1e310(this_ptr: u32) -> u32 {
    unsafe {
        let mgr = *(global::<u32>(0x0167E2A0));
        let fresh = callee_thiscall!(1, u32, mgr);
        if fresh == 0 {
            return 0;
        }
        let w0 = *((this_ptr.wrapping_add(0x14)) as *const u32);
        let w1 = *((this_ptr.wrapping_add(0x18)) as *const u32);
        let f = *((this_ptr.wrapping_add(0x30)) as *const u32);
        let flag = *((this_ptr.wrapping_add(0x3C)) as *const u8) as u32;
        let w5 = *((this_ptr.wrapping_add(0x40)) as *const u32);
        callee_thiscall!(2, u32, fresh, w0, w1, this_ptr.wrapping_add(0x20), f, flag, w5)
    }
});
