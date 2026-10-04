// original: 0x00e62ed0 refresh_ratio_1173728
/// Refresh the stored ratio for the sixth scale slot.
export!(cdecl, rw_00e62ed0() -> () {
    unsafe {
        let a = *global::<f32>(0x0103276c);
        let b = *global::<f32>(0x01032770);
        *global::<f32>(0x01173728) = a / b;
    }
});
