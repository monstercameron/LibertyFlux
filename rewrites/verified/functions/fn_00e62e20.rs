// original: 0x00e62e20 refresh_ratio_117358c
/// Refresh the stored ratio for the third scale slot.
export!(cdecl, rw_00e62e20() -> () {
    unsafe {
        let a = *global::<f32>(0x01032320);
        let b = *global::<f32>(0x01032324);
        *global::<f32>(0x0117358c) = a / b;
    }
});
