// original: 0x00E6E2A0 mainloop_timing_release_group_b_slot_1

/// Passes the second group's object pointer to the imported critical-section
/// release routine and returns the API's result.
pub extern "cdecl" fn rw_00e6e2a0() -> u32 {
    lf_checker_rt::callee_stdcall!(1, u32, lf_checker_rt::relocated(0x018d_c250))
}
