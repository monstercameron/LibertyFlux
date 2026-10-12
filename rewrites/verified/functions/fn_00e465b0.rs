// original: 0x00E465B0 UIScreen::~UIScreen__deleting

/// Run the class-specific cleanup for this object, then call the deallocator when bit zero of `flags` is set, and return the object pointer. The cleanup is a thiscall with no stack arguments; the deallocator is a cdecl with the object pointer as its sole argument.
lf_checker_rt::export!(thiscall, rw_00e465b0(this: u32, flags: u32) -> u32 {
    const DELETE_FLAG: u32 = 0x01;
    let _cleanup_result = lf_checker_rt::callee_thiscall!(1, u32, this);
    if flags & DELETE_FLAG != 0 {
        let _deallocate_result = lf_checker_rt::callee_cdecl!(2, u32, this);
    }
    this
});
