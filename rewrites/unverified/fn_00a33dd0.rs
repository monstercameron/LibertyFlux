// original: 0x00A33DD0 CMissionCleanup::update

/// Ask the update helper for its one-byte ready result. Write status 2 to the
/// global state word only when the helper is nonzero and the cleanup-active
/// global word is nonzero; otherwise write status 3. The helper has no
/// arguments, and its result is read from AL. The two absolute data references
/// are relocated through the checker runtime.
lf_checker_rt::export!(cdecl, rw_00a33dd0() -> () {
    const UPDATE_READY: u32 = 1;
    const CLEANUP_ACTIVE_VA: u32 = 0x017a_d180;
    const UPDATE_STATUS_VA: u32 = 0x0103_cb08;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let ready = lf_checker_rt::callee_cdecl!(UPDATE_READY, u8);
        let cleanup_active = read_u32(lf_checker_rt::relocated(CLEANUP_ACTIVE_VA));
        let status = if ready != 0 && cleanup_active != 0 { 2 } else { 3 };
        write_u32(lf_checker_rt::relocated(UPDATE_STATUS_VA), status);
    }
});
