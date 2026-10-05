// original: 0x008B10E0 rage::audNullEffect::vf0 (merged symbol)

/// Scalar-deleting destructor: install the vtable, run the base destructor,
/// conditionally free `this`, and return `this`.
///
/// Stores the relocated vtable address for file VA `0xe7cbc4` at `this`,
/// calls the base destructor (`0x8a9300`, thiscall, no arguments), and when
/// the low bit of the `flags` argument is set passes `this` to the free
/// routine (`0x401250`, cdecl) before returning `this`. Original is thiscall
/// with one stack word (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_008B10E0(this: u32, flags: u32) -> u32 {
    const BASE_DTOR: u32 = 1;
    const FREE: u32 = 2;
    const VTABLE_FILE_VA: u32 = 0x00e7_cbc4;
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_FILE_VA));
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this);
        if flags & 1 != 0 {
            lf_checker_rt::callee_cdecl!(FREE, u32, this);
        }
    }
    this
});
