// original: 0x00e62ef0 refresh_ratio_117379c
/// Refresh the stored ratio for the seventh scale slot.
export!(cdecl, rw_00e62ef0() -> () {
    unsafe {
        let a = *global::<f32>(0x0103277c);
        let b = *global::<f32>(0x01032780);
        *global::<f32>(0x0117379c) = a / b;
    }
});
