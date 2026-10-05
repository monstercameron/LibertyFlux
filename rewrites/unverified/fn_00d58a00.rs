// original: 0x00d58a00 CCamViewFind::vf0
/// Scalar deleting destructor for CCamViewFind: runs the class destructor, then
/// frees the object through the game allocator when the deleting flag is set.
///
/// Calls the destructor (intercepted callee 1, thiscall, no stack arguments)
/// with the object pointer, then tests bit 0 of the stack flag: when set it
/// reads the allocator from global `ALLOCATOR` and hands the object to the
/// free routine (intercepted callee 2, thiscall, one argument). Returns the
/// object pointer in both cases.
///
/// Original: thiscall, one stack argument (deleting flag), returns `this`.
lf_checker_rt::export!(thiscall, rw_00d58a00 (this: u32, flags: u32) -> u32 {
    unsafe {
        const DTOR_CALLEE: u32 = 1;
        const FREE_CALLEE: u32 = 2;
        const ALLOCATOR: u32 = 0x012FB1A0;
        const DELETE_FLAG: u32 = 1;
        let _: u32 = lf_checker_rt::callee_thiscall!(DTOR_CALLEE, u32, this);
        if flags & DELETE_FLAG != 0 {
            let alloc = lf_checker_rt::global::<u32>(ALLOCATOR).read_unaligned();
            let _: u32 = lf_checker_rt::callee_thiscall!(FREE_CALLEE, u32, alloc, this);
        }
        this
    }
});
