// original: 0x00e64810 static_init_audAmbientAudioEntity_01231800
/// Static initializer: constructs a static audAmbientAudioEntity object and
/// registers its destructor to run at process exit.
/// Takes no arguments; returns the registration result.
lf_rs102_rt::export!(cdecl, rw_00e64810() -> u32 {
    let obj = lf_rs102_rt::relocated(0x01231800);
    lf_rs102_rt::callee_thiscall!(1, u32, obj);
    lf_rs102_rt::callee_cdecl!(2, u32, lf_rs102_rt::relocated(0x00E71850))
});
