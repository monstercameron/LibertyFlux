// original: 0x00B79E90 CTaskComplexLeaveCarAndWander::~CTaskComplexLeaveCarAndWander

/// Destructor body for CTaskComplexLeaveCarAndWander.
///
/// releases the optional member pointer at `+0x14`; then tail-calls the common task destructor and returns its result.
lf_checker_rt::export!(thiscall, rw_00b79e90(this: u32) -> u32 {
    unsafe {
        const BASE_DESTRUCTOR: u32 = 5;
        const RELEASE_MEMBER: u32 = 1;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        const VTABLE_FILE_VA: u32 = 0x00EB322C;
        let vtable = lf_checker_rt::relocated(VTABLE_FILE_VA);
        (this as *mut u32).write_unaligned(vtable);
        let member_slot = this.wrapping_add(0x14);
        let member = read_word(member_slot);
        if member != 0 {
            lf_checker_rt::callee_thiscall!(RELEASE_MEMBER, u32, member, member_slot);
        }
        lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this)
    }
});
