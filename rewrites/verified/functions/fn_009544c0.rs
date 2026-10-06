// original: 0x009544C0 disk_space_check_300m (proposed)

/// Check for at least 300 MiB of free disk space.
///
/// Calls the disk-space check with the fixed requirement
/// (`MIN_LO`, `MIN_HI` = 0x12C00000 bytes = 300 MiB, low word first)
/// and returns its result unchanged. Original is cdecl/0, returns AL.
lf_checker_rt::export!(cdecl, rw_009544C0() -> u32 {
    const CHECK: u32 = 1;
    const MIN_LO: u32 = 0x12C00000;
    const MIN_HI: u32 = 0;
    lf_checker_rt::callee_cdecl!(CHECK, u32, MIN_LO, MIN_HI)
});
