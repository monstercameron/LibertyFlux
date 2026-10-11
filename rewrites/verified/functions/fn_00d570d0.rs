// original: 0x00d570d0 CCamCinematicCamMan::constructor

/// Constructs CCamCinematicCamMan by initializing the base camera, its parameter block and its state in that order, then returning the object pointer.
///
/// `this` is a 32-bit pointer to the CCamCinematicCamMan object; the object uses the shared camera layout.
/// Original calling convention: thiscall with 0 explicit stack argument(s).
lf_checker_rt::export!(thiscall, rw_00d570d0(this: u32) -> u32 {
        #[inline(always)]
        unsafe fn write32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        const CALLEE_BASE_CONSTRUCTOR: u32 = 1;
        const CALLEE_PARAMETER_BLOCK_INITIALIZER: u32 = 2;
        const CALLEE_STATE_INITIALIZER: u32 = 3;
        const VTABLE_FILE_VA: u32 = 0x00ee6f3c;
        const PARAM_BLOCK_OFFSET: u32 = 0x140;
        let _ = lf_checker_rt::callee_thiscall!(CALLEE_BASE_CONSTRUCTOR, u32, this);
        unsafe { write32(this, lf_checker_rt::relocated(VTABLE_FILE_VA)); }
        let _ = lf_checker_rt::callee_thiscall!(CALLEE_PARAMETER_BLOCK_INITIALIZER, u32, this.wrapping_add(PARAM_BLOCK_OFFSET));
        let _ = lf_checker_rt::callee_thiscall!(CALLEE_STATE_INITIALIZER, u32, this);
        this
});
