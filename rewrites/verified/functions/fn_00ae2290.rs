// original: 0x00ae2290 CGrcResetToDefault::~CGrcResetToDefault
/// Deleting destructor: restore the base vtable, free on request.
///
/// Writes back the base vtable; when the low flag bit is set the object is
/// handed to the deallocator. Returns the object. (This address is shared
/// with a folded template vtable slot, hence the twin name in the batch.)
export!(thiscall, rw_00ae2290(this: *mut u32, flags: u32) -> u32 {
    unsafe {
        const VT_BASE: u32 = 0x00E7E048;
        *this = relocated(VT_BASE);
        if flags & 1 != 0 {
            callee_cdecl!(1, u32, this as u32);
        }
        this as u32
    }
});
