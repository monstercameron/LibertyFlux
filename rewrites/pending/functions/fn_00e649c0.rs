// original: 0x00e649c0 static_init_audEmitterAudioEntity_012389e0
/// Static initializer: constructs a static audEmitterAudioEntity object and
/// registers its destructor to run at process exit.
/// Takes no arguments; returns the registration result.
lf_rs102_rt::export!(cdecl, rw_00e649c0() -> u32 {
    let obj = lf_rs102_rt::relocated(0x012389E0);
    lf_rs102_rt::callee_thiscall!(1, u32, obj);
    lf_rs102_rt::callee_cdecl!(2, u32, lf_rs102_rt::relocated(0x00E71900))
});
