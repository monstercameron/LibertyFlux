// original: 0x006941E0 optional_pointer_release_006941E0 (proposed)

/// Release an optional object with one associated value.
///
/// The value is the first cdecl stack argument and the optional object is the
/// second. A null object returns zero without a call. Otherwise the object is
/// passed as ECX and the value is forwarded to the release helper; its result
/// becomes this function's return value.
lf_checker_rt::export!(cdecl, rw_006941E0(value: u32, object: u32) -> u32 {
    if object == 0 {
        return 0;
    }
    lf_checker_rt::callee_thiscall!(1, u32, object, value)
});
