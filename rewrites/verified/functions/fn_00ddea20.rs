// original: 0x00DDEA20 UITextField update with default mode
/// UITextField helper (input-ui subsystem). `this` is the field object.
/// Forward to the field updater with mode 0: call the two-argument updater
/// with (`key`, 0) and return its result. Original: thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_00DDEA20(this: u32, key: u32) -> u32 {
    unsafe {
        const UPDATE: u32 = 1;
        lf_checker_rt::callee_thiscall!(UPDATE, u32, this, key, 0)
    }
});
