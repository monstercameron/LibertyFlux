// original: 0x00e65c20 PAIN_MALE_EXTRAS
/// Hash the "PAIN_MALE_EXTRAS" voice name and stash the handle.
///
/// Calls the shared name-hash helper on this slot's static name string and
/// stores the resulting handle in this slot's static cell, returning it.
lf_checker_rt::export!(cdecl, rw_00e65c20() -> u32 {
    let handle: u32 = lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(0x00E90BA8), 0);
    unsafe { *lf_checker_rt::global::<u32>(0x01284458) = handle; }
    handle
});
