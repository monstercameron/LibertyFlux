// original: 0x00e64920 static_init_audio_entity_012388a0
/// Static initializer: constructs a static audio entity object and
/// registers its destructor to run at process exit.
/// Takes no arguments; returns the registration result.
lf_checker_rt::export!(cdecl, rw_00e64920() -> u32 {
    let obj = lf_checker_rt::relocated(0x012388A0);
    lf_checker_rt::callee_thiscall!(1, u32, obj);
    lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(0x00E718B0))
});
