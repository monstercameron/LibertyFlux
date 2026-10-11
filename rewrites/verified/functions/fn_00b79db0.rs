// original: 0x00B79DB0 CTaskComplexCarSetTempAction::~CTaskComplexCarSetTempAction

/// Destructor body for CTaskComplexCarSetTempAction.
///
/// releases the optional member pointer at `+0x40`; calls the subobject cleanup at `+0x14`; then tail-calls the common task destructor and returns its result.
lf_checker_rt::export!(thiscall, rw_00b79db0(this: u32) -> u32 {
    unsafe {
        const BASE_DESTRUCTOR: u32 = 5;
        const RELEASE_MEMBER: u32 = 1;
        const SUBOBJECT_CLEANUP: u32 = 2;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        const VTABLE_FILE_VA: u32 = 0x00EB37BC;
        let vtable = lf_checker_rt::relocated(VTABLE_FILE_VA);
        (this as *mut u32).write_unaligned(vtable);
        let member_slot = this.wrapping_add(0x40);
        let member = read_word(member_slot);
        if member != 0 {
            lf_checker_rt::callee_thiscall!(RELEASE_MEMBER, u32, member, member_slot);
        }
        lf_checker_rt::callee_thiscall!(SUBOBJECT_CLEANUP, u32, this.wrapping_add(0x14));
        lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this)
    }
});
