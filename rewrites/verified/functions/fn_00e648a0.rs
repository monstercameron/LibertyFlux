// original: 0x00e648a0 static_init_audio_entity_pair_01231788
/// Static initializer: constructs two static audio entity objects and
/// registers its destructor to run at process exit.
/// The two objects are 0x28 bytes apart and share one destructor entry.
/// Takes no arguments; returns the registration result.
lf_rs102_rt::export!(cdecl, rw_00e648a0() -> u32 {
    let mut obj = lf_rs102_rt::relocated(0x01231788);
    for _ in 0..2 {
        lf_rs102_rt::callee_thiscall!(1, u32, obj);
        obj = obj.wrapping_add(0x28);
    }
    lf_rs102_rt::callee_cdecl!(2, u32, lf_rs102_rt::relocated(0x00E71890))
});
