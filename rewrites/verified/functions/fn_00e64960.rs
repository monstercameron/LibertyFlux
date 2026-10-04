// original: 0x00e64960 static_init_audio_entity_012388c8
/// Static initializer: constructs a static audio entity object and
/// registers its destructor to run at process exit.
/// Takes no arguments; returns the registration result.
lf_rs102_rt::export!(cdecl, rw_00e64960() -> u32 {
    let obj = lf_rs102_rt::relocated(0x012388C8);
    lf_rs102_rt::callee_thiscall!(1, u32, obj);
    lf_rs102_rt::callee_cdecl!(2, u32, lf_rs102_rt::relocated(0x00E718E0))
});
