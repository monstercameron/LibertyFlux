// original: 0x00d56f80 CCamAimWeaponVeh::constructor

/// Constructs CCamAimWeaponVeh by calling the shared camera base constructor, installing the class vtable, and returning the object pointer.
///
/// `this` is a 32-bit pointer to the CCamAimWeaponVeh object; the object uses the shared camera layout.
/// Original calling convention: thiscall with 0 explicit stack argument(s).
lf_checker_rt::export!(thiscall, rw_00d56f80(this: u32) -> u32 {
        #[inline(always)]
        unsafe fn write32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        const CALLEE_BASE_CONSTRUCTOR: u32 = 1;
        const VTABLE_FILE_VA: u32 = 0x00ee69e4;
        let _ = lf_checker_rt::callee_thiscall!(CALLEE_BASE_CONSTRUCTOR, u32, this);
        unsafe { write32(this, lf_checker_rt::relocated(VTABLE_FILE_VA)); }
        this
});
