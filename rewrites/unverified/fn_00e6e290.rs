// original: 0x00E6E290 mainloop_timing_release_group_a_slot_1

/// Passes the first group's object pointer to the imported critical-section
/// release routine and returns the API's result.
pub extern "cdecl" fn rw_00e6e290() -> u32 {
    lf_checker_rt::callee_stdcall!(1, u32, lf_checker_rt::relocated(0x018d_c210))
}
