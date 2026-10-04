// original: 0x00d1d7a0 CTaskComplexSeekCover::~CTaskComplexSeekCover
// Destructor: stamps the base vtable, releases the two owned slots when
// set (and nulls them), runs the two member destructors, then tails into
// the base task destructor. Returns the base destructor's answer.
export!(thiscall, rw_00d1d7a0(this_ptr: u32) -> u32 {
    unsafe {
        *(this_ptr as *mut u32) = relocated(0x00EE0A6C);
        let s0 = *((this_ptr.wrapping_add(0x30)) as *const u32);
        if s0 != 0 {
            callee_thiscall!(1, u32, s0, this_ptr.wrapping_add(0x30));
            *((this_ptr.wrapping_add(0x30)) as *mut u32) = 0;
        }
        let s1 = *((this_ptr.wrapping_add(0xB0)) as *const u32);
        if s1 != 0 {
            callee_thiscall!(1, u32, s1, this_ptr.wrapping_add(0xB0));
            *((this_ptr.wrapping_add(0xB0)) as *mut u32) = 0;
        }
        callee_thiscall!(2, u32, this_ptr.wrapping_add(0x6C));
        callee_thiscall!(3, u32, this_ptr.wrapping_add(0x60));
        callee_thiscall!(4, u32, this_ptr)
    }
});
