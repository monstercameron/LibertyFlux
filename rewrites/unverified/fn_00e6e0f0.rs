// original: 0x00E6E0F0 delete_global_critical_section_00e6e0f0

/// Pass the relocated critical-section object at 0x018b9b5c to the
/// KERNEL32 DeleteCriticalSection import. The imported call is scripted and its
/// single stdcall argument is compared. The API is void and the wrapper defines
/// no return value.

lf_checker_rt::export!(cdecl, rw_delete_global_critical_section_00e6e0f0() -> u32 {
    unsafe {
        const CRITICAL_SECTION_VA: u32 = 0x018B9B5C;
        let critical_section = lf_checker_rt::relocated(CRITICAL_SECTION_VA);
        lf_checker_rt::callee_stdcall!(1, u32, critical_section)
    }
});
