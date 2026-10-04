// original: 0x00e62fe0 refresh_ratio_1176870
/// Refresh the stored ratio for the eleventh scale slot.
export!(cdecl, rw_00e62fe0() -> () {
    unsafe {
        let a = *global::<f32>(0x01032f70);
        let b = *global::<f32>(0x01032f74);
        *global::<f32>(0x01176870) = a / b;
    }
});
