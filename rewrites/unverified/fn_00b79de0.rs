// original: 0x00B79DE0 CTaskComplexDrivePointRoute::~CTaskComplexDrivePointRoute

/// Destructor body for CTaskComplexDrivePointRoute.
///
/// passes the optional member at `+0x44` to cleanup using the global handle at its documented slot; calls the subobject cleanup at `+0x14`; then tail-calls the common task destructor and returns its result.
lf_checker_rt::export!(thiscall, rw_00b79de0(this: u32) -> u32 {
    unsafe {
        const RELEASE_MEMBER: u32 = 1;
        const BASE_DESTRUCTOR: u32 = 5;
        const SUBOBJECT_CLEANUP: u32 = 2;
        const MANAGED_RELEASE: u32 = 3;
        const MANAGER_SLOT: u32 = 0x0179D114;
        const VTABLE_FILE_VA: u32 = 0x00EB36B4;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let vtable = lf_checker_rt::relocated(VTABLE_FILE_VA);
        (this as *mut u32).write_unaligned(vtable);
        let member = read_word(this.wrapping_add(0x44));
        if member != 0 {
            let manager = lf_checker_rt::global::<u32>(0x0179D114).read();
            lf_checker_rt::callee_thiscall!(MANAGED_RELEASE, u32, manager, member);
        }
        lf_checker_rt::callee_thiscall!(SUBOBJECT_CLEANUP, u32, this.wrapping_add(0x14));
        lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this)
    }
});
