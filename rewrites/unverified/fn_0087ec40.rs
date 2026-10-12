// original: 0x0087EC40 get_or_empty_object

/// Return the stored object or a fixed empty-object sentinel.
///
/// `this` points to a small handle. Its active pointer is stored at `+0x08`;
/// only when that word is nonzero does the function read the first word as the
/// stored object pointer. A nonzero stored pointer is returned directly; zero
/// in either guarded path returns the relocated sentinel whose preferred-base
/// address is `0x00F1C65A`.
///
/// Original: thiscall with no stack arguments. A null `this` pointer faults
/// before either branch and is outside the completed input set.
lf_checker_rt::export!(thiscall, rw_0087ec40(this: u32) -> u32 {
    const OBJECT_OFFSET: u32 = 0;
    const ACTIVE_OFFSET: u32 = 0x08;
    const EMPTY_OBJECT_FILE_VA: u32 = 0x00F1_C65A;

    unsafe {
        let active = (this.wrapping_add(ACTIVE_OFFSET) as *const u32).read_unaligned();
        if active == 0 {
            return lf_checker_rt::relocated(EMPTY_OBJECT_FILE_VA);
        }
        let object = (this.wrapping_add(OBJECT_OFFSET) as *const u32).read_unaligned();
        if object == 0 {
            lf_checker_rt::relocated(EMPTY_OBJECT_FILE_VA)
        } else {
            object
        }
    }
});
