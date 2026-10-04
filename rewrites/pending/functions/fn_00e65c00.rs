// original: 0x00e65c00 PAIN_FEMALE_EXTRAS
/// Hash the "PAIN_FEMALE_EXTRAS" voice name and stash the handle.
///
/// Calls the shared name-hash helper on this slot's static name string and
/// stores the resulting handle in this slot's static cell, returning it.
lf_k2_rt::export!(cdecl, rw_00e65c00() -> u32 {
    let handle: u32 = lf_k2_rt::callee_cdecl!(1, u32, lf_k2_rt::relocated(0x00E90B8C), 0);
    unsafe { *lf_k2_rt::global::<u32>(0x01284434) = handle; }
    handle
});
