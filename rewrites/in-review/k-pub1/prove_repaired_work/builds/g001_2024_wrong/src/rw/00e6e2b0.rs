

/// Deliberately releases the neighboring resource to exercise the call proof.
#[unsafe(no_mangle)]
pub extern "cdecl" fn rw_00e6e2b0() -> u32 {
    release_critical_section(0x018d_c234)
}

/// Releases the third object in the matching critical-section group.
lf_checker_rt::export!(cdecl, lane_base_00e6e2b0() -> u32 {
    release_critical_section(0x018d_c230)
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
