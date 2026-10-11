// original: 0x00DED550 UIButton::~UIButton__deleting

/// Run the button teardown and release the object only when bit zero of
/// `flags` is set. The destructor helper and allocator are intercepted calls;
/// one stack argument is consumed and the object pointer is returned.
lf_checker_rt::export!(thiscall, rw_00ded550(this: u32, flags: u32) -> u32 {
    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        if flags & 1 != 0 {
            let _ = lf_checker_rt::callee_cdecl!(2, u32, this);
        }
        this
    }
});
