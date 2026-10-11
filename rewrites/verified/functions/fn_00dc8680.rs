// original: 0x00dc8680 task_destroy_optional_child
/// Release the optional child object stored at `+0x18`, clear the slot after
/// the helper returns, and forward to the base task destructor. The helper gets
/// the child in ECX and the task pointer as its stack argument. The final
/// thiscall tail target supplies the return value; there are no stack inputs.
lf_checker_rt::export!(thiscall, rw_00dc8680(this_ptr: u32) -> u32 {
    unsafe {
        const CLASS_VTABLE_VA: u32 = 0x00ef4d14;
        const CHILD_SLOT: u32 = 0x18;
        const RELEASE_CHILD: u32 = 1;
        const BASE_DTOR: u32 = 2;
        let child = (this_ptr.wrapping_add(CHILD_SLOT) as *const u32).read_unaligned();
        (this_ptr as *mut u32).write_unaligned(lf_checker_rt::relocated(CLASS_VTABLE_VA));
        if child != 0 {
            let _released = lf_checker_rt::callee_thiscall!(RELEASE_CHILD, u32, child, this_ptr);
            (this_ptr.wrapping_add(CHILD_SLOT) as *mut u32).write_unaligned(0);
        }
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this_ptr)
    }
});
