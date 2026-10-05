// original: 0x00AF46C0 gtaDrawable::~gtaDrawable__deleting (symbols)

/// Deleting destructor: destroy the drawable, free it when asked.
///
/// Calls the destroying callee with this object; when the low bit of
/// `flags` is set the object is then released through the free callee.
/// Returns the object pointer either way.
///
/// Original: 0x00AF46C0 (thiscall, one stack word, two direct callees).
lf_checker_rt::export!(thiscall, rw_00af46c0(this: u32, flags: u32) -> u32 {
    unsafe {
        const DESTROY_CALLEE: u32 = 1;
        const FREE_CALLEE: u32 = 2;
        const FREE_FLAG: u32 = 1;
        lf_checker_rt::callee_thiscall!(DESTROY_CALLEE, u32, this);
        if flags & FREE_FLAG != 0 {
            lf_checker_rt::callee_cdecl!(FREE_CALLEE, u32, this);
        }
        this
    }
});
