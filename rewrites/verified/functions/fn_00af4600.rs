// original: 0x00AF4600 drawable_refresh_self (proposed)

/// Refresh the object through its helper, then return the object.
///
/// Calls the refresh callee with this object and returns the object
/// pointer unchanged. The callee result is ignored.
///
/// Original: 0x00AF4600 (thiscall, no stack words, one direct callee).
lf_checker_rt::export!(thiscall, rw_00af4600(this: u32) -> u32 {
    unsafe {
        const REFRESH_CALLEE: u32 = 1;
        lf_checker_rt::callee_thiscall!(REFRESH_CALLEE, u32, this);
        this
    }
});
