// original: 0x00e62e00 refresh_ratio_1173354
/// Refresh the stored ratio for the second scale slot.
export!(cdecl, rw_00e62e00() -> () {
    unsafe {
        let a = *global::<f32>(0x01031b18);
        let b = *global::<f32>(0x01031b1c);
        *global::<f32>(0x01173354) = a / b;
    }
});
