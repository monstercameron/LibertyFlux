// original: 0x00b50660 CEventGlobalGroup::vf0

/// Deleting destructor of the global event group.
///
/// Runs the destructor body (callee 1, thiscall on `this` with no
/// arguments); when bit 0 of `free_flag` is set, frees the object through
/// the scalar operator delete (callee 2, cdecl). Returns `this` either way.
///
/// Original: 0x00b50660 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b50660(this: u32, free_flag: u32) -> u32 {
    unsafe {
        const DTOR: u32 = 1;
        const OPERATOR_DELETE: u32 = 2;
        const FREE_BIT: u32 = 1;
        lf_checker_rt::callee_thiscall!(DTOR, u32, this);
        if free_flag & FREE_BIT != 0 {
            lf_checker_rt::callee_cdecl!(OPERATOR_DELETE, u32, this);
        }
        this
    }
});
