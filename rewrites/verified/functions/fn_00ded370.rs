// original: 0x00DED370 UILayoutManager::~UILayoutManager__deleting

/// Run the layout manager teardown and release the object only when bit zero
/// of `flags` is set. The teardown and release helpers are intercepted calls;
/// one stack argument is consumed and the object pointer is returned.
lf_checker_rt::export!(thiscall, rw_00ded370(this: u32, flags: u32) -> u32 {
    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        if flags & 1 != 0 {
            let _ = lf_checker_rt::callee_cdecl!(2, u32, this);
        }
        this
    }
});
