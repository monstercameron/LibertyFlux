// original: 0x00e62e90 refresh_ratio_11736a0
/// Refresh the stored ratio for the fifth scale slot.
export!(cdecl, rw_00e62e90() -> () {
    unsafe {
        let a = *global::<f32>(0x01032358);
        let b = *global::<f32>(0x0103235c);
        *global::<f32>(0x011736a0) = a / b;
    }
});
