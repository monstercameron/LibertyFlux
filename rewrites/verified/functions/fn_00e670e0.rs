// original: 0x00E670E0 CCamFollowPed::PlayerHeadLookAt
/// Look up a task handle by name and store it.
///
/// Calls callee 1 (cdecl, two arguments: the name string `NAME`
/// first, then 0) and stores the returned handle to the global
/// `SLOT`.
///
/// Original: 0x00E670E0 (cdecl, no arguments, no return value).
lf_checker_rt::export!(cdecl, rw_00E670E0() -> u32 {
    unsafe {
        const NAME: u32 = 0x00E9B470;
        const SLOT: u32 = 0x012DD2AC;
        let handle: u32 = lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(NAME), 0);
        (lf_checker_rt::global::<u32>(SLOT)).write_unaligned(handle);
        0
    }
});
