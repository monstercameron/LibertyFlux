// original: 0x00e65bc0 NO_VOICE
/// Hash the "NO_VOICE" voice name and stash the handle.
///
/// Calls the shared name-hash helper on this slot's static name string and
/// stores the resulting handle in this slot's static cell, returning it.
lf_k2_rt::export!(cdecl, rw_00e65bc0() -> u32 {
    let handle: u32 = lf_k2_rt::callee_cdecl!(1, u32, lf_k2_rt::relocated(0x00E90F78), 0);
    unsafe { *lf_k2_rt::global::<u32>(0x012844B4) = handle; }
    handle
});
