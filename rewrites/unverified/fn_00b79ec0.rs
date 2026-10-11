// original: 0x00B79EC0 CTaskComplexPlayerDrive::~CTaskComplexPlayerDrive

/// Destructor body for CTaskComplexPlayerDrive.
///
/// releases the optional member pointer at `+0x28` and clears it after the call; calls the subobject cleanup at `+0x34`; calls the subobject cleanup at `+0x14`; then tail-calls the common task destructor and returns its result.
lf_checker_rt::export!(thiscall, rw_00b79ec0(this: u32) -> u32 {
    unsafe {
        const RELEASE_MEMBER: u32 = 1;
        const BASE_DESTRUCTOR: u32 = 5;
        const SUBOBJECT_CLEANUP: u32 = 2;
        const VTABLE_FILE_VA: u32 = 0x00EB34BC;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let vtable = lf_checker_rt::relocated(VTABLE_FILE_VA);
        (this as *mut u32).write_unaligned(vtable);
        let member_slot = this.wrapping_add(0x28);
        let member = read_word(member_slot);
        if member != 0 {
            lf_checker_rt::callee_thiscall!(RELEASE_MEMBER, u32, member, member_slot);
            (member_slot as *mut u32).write_unaligned(0);
        }
        lf_checker_rt::callee_thiscall!(SUBOBJECT_CLEANUP, u32, this.wrapping_add(0x34));
        lf_checker_rt::callee_thiscall!(SUBOBJECT_CLEANUP, u32, this.wrapping_add(0x14));
        lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this)
    }
});
