// original: 0x00e64ac0 static_init_audWeaponAudioEntity_012831e4
/// Static initializer: constructs a static audWeaponAudioEntity object and
/// registers its destructor to run at process exit.
/// Takes no arguments; returns the registration result.
lf_rs102_rt::export!(cdecl, rw_00e64ac0() -> u32 {
    let obj = lf_rs102_rt::relocated(0x012831E4);
    lf_rs102_rt::callee_thiscall!(1, u32, obj);
    lf_rs102_rt::callee_cdecl!(2, u32, lf_rs102_rt::relocated(0x00E71930))
});
