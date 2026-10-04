// original: 0x00e64a40 static_init_audCutsceneAudioEntity_01282fa8
/// Static initializer: constructs a static audCutsceneAudioEntity object and
/// registers its destructor to run at process exit.
/// Takes no arguments; returns the registration result.
lf_rs102_rt::export!(cdecl, rw_00e64a40() -> u32 {
    let obj = lf_rs102_rt::relocated(0x01282FA8);
    lf_rs102_rt::callee_thiscall!(1, u32, obj);
    lf_rs102_rt::callee_cdecl!(2, u32, lf_rs102_rt::relocated(0x00E71910))
});
