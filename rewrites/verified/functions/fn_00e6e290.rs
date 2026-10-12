// original: 0x00E6E290 mainloop_timing_release_group_a_slot_1

/// Releases the first object in the matching critical-section group.
lf_checker_rt::export!(cdecl, rw_00e6e290() -> u32 {
    release_critical_section(0x018d_c210)
});

#[inline(always)]
fn release_critical_section(resource_va: u32) -> u32 {
    lf_checker_rt::callee_stdcall!(
        DELETE_CRITICAL_SECTION,
        u32,
        lf_checker_rt::relocated(resource_va)
    )
}

const DELETE_CRITICAL_SECTION: u32 = 1;
