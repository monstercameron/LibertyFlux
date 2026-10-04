// original: 0x00e65800 audio_slot_init_08
/// Initialise one audio event slot, then register its tag.
///
/// Runs the shared per-slot setup step on this slot's static object and
/// registers the slot's tag value, returning the registration result.
lf_k2_rt::export!(cdecl, rw_00e65800() -> u32 {
    lf_k2_rt::callee_thiscall!(1, u32, lf_k2_rt::relocated(0x01283D78));
    lf_k2_rt::callee_cdecl!(2, u32, lf_k2_rt::relocated(0x00E71E00))
});
