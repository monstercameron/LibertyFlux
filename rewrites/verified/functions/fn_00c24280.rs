// original: 0x00c24280 cam_interp_spawn (proposed)
/// Construct a camera interpolator in place: run the base constructor,
/// install this class's virtual table, then run the field initialiser.
/// Returns the object pointer.
///
/// Original: 0x00c24280 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00c24280(obj: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00ec57c0;
        lf_checker_rt::callee_thiscall!(1, u32, obj);
        (obj as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(2, u32, obj);
        obj
    }
});
