// original: 0x00e62f80 refresh_ratio_1175c54
/// Refresh the stored ratio for the ninth scale slot.
export!(cdecl, rw_00e62f80() -> () {
    unsafe {
        let a = *global::<f32>(0x01032798);
        let b = *global::<f32>(0x0103279c);
        *global::<f32>(0x01175c54) = a / b;
    }
});
