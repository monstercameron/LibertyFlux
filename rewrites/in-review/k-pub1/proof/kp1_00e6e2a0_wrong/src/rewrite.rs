/// Deliberately releases the neighboring resource to exercise the call proof.
#[unsafe(no_mangle)]
pub extern "cdecl" fn rw_00e6e2a0() -> u32 {
    release_critical_section(0x018d_c254)
}

#[inline(always)]
fn release_critical_section(resource_va: u32) -> u32 {
    lf_checker_rt::callee_stdcall!(
        DELETE_CRITICAL_SECTION,
        u32,
        lf_checker_rt::relocated(resource_va)
    )
}

const DELETE_CRITICAL_SECTION: u32 = 1;
