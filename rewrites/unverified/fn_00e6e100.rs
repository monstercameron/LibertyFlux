// original: 0x00E6E100 mainloop_timing_release_critical_section_primary

/// Release the primary timing critical section. The cdecl wrapper has no
/// stack arguments, calls the imported stdcall cleanup routine once with the
/// relocated object address, and leaves that call's EAX value as its result.
lf_checker_rt::export!(cdecl, rw_00e6e100() -> u32 {
    const DELETE_CRITICAL_SECTION: u32 = 1;
    const RESOURCE_VA: u32 = 0x018b_9b7c;

    let resource = lf_checker_rt::relocated(RESOURCE_VA);
    lf_checker_rt::callee_stdcall!(DELETE_CRITICAL_SECTION, u32, resource)
});
