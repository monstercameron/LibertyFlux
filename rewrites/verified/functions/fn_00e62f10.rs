// original: 0x00e62f10 refresh_ratio_1175764
/// Refresh the stored ratio for the eighth scale slot.
export!(cdecl, rw_00e62f10() -> () {
    unsafe {
        let a = *global::<f32>(0x01032788);
        let b = *global::<f32>(0x0103278c);
        *global::<f32>(0x01175764) = a / b;
    }
});
