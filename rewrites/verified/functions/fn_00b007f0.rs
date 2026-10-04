// original: 0x00b007f0 reset_viewport_object
/// Release the object's owned pointer and re-initialise its parameter blocks,
/// adopting the caller's value for the first block when one is given.
export!(thiscall, rw_00b007f0(this: u32, arg: u32) -> u32 {
    unsafe {
        callee_cdecl!(1, u32, *(this as *const u32));
        *(this as *mut u32) = 0;
        *((this + 4) as *mut u32) = 0;
        callee_thiscall!(2, u32, this.wrapping_add(0xc));
        if arg != 0 {
            *((this + 0xc) as *mut u32) = *(arg as *const u32);
        }
        callee_thiscall!(2, u32, this.wrapping_add(0x18));
        *((this + 0x18) as *mut u32) = 0xffffffff;
        0
    }
});
