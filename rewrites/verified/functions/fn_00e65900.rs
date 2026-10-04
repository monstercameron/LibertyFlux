// original: 0x00e65900 audio_slot_init_11
/// Initialise one audio event slot, then register its tag.
///
/// Runs the shared per-slot setup step on this slot's static object and
/// registers the slot's tag value, returning the registration result.
lf_k2_rt::export!(cdecl, rw_00e65900() -> u32 {
    lf_k2_rt::callee_thiscall!(1, u32, lf_k2_rt::relocated(0x012844BC));
    lf_k2_rt::callee_cdecl!(2, u32, lf_k2_rt::relocated(0x00E71E30))
});
