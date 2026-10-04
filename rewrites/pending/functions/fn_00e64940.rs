// original: 0x00e64940 static_init_rage__audEntity_01238898
/// Static initializer: constructs a static rage::audEntity object and
/// registers its destructor to run at process exit.
/// Installs the final table pointer before registering the destructor.
/// Takes no arguments; returns the registration result.
lf_rs102_rt::export!(cdecl, rw_00e64940() -> u32 {
    let obj = lf_rs102_rt::relocated(0x01238898);
    lf_rs102_rt::callee_thiscall!(1, u32, obj);
    unsafe { *(obj as *mut u32) = lf_rs102_rt::relocated(0x00E8DECC); }
    lf_rs102_rt::callee_cdecl!(2, u32, lf_rs102_rt::relocated(0x00E718C0))
});
