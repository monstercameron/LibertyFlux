// original: 0x00CC76B0 ART::ARTFeedbackInterface::vf0

/// Scalar deleting destructor: plant the vtable, free the object on request.
///
/// Writes the interface vtable address at `this`. When bit 0 of `flags` is
/// set the object is handed to the freeing callee; otherwise no call is made.
/// Returns `this` either way. Only the low bit of `flags` is tested.
///
/// Original: 0x00CC76B0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00cc76b0(this: u32, flags: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00ED976C;
        const FREE_CALLEE: u32 = 1;
        const DELETE_ME: u32 = 1;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        if flags & DELETE_ME != 0 {
            lf_checker_rt::callee_cdecl!(FREE_CALLEE, u32, this);
        }
        this
    }
});
