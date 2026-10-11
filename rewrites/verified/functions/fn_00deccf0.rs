// original: 0x00DECCF0 UIStackLayout::~UIStackLayout__deleting

/// Run the layout object's teardown and, when bit zero of `flags` is set,
/// release the object through the intercepted allocator helper. The function
/// returns the original object pointer and consumes one stack argument using
/// the thiscall convention.
lf_checker_rt::export!(thiscall, rw_00deccf0(this: u32, flags: u32) -> u32 {
    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        if flags & 1 != 0 {
            let _ = lf_checker_rt::callee_cdecl!(2, u32, this);
        }
        this
    }
});
