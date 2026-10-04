// original: 0x00e62e70 refresh_ratio_1173690
/// Refresh the stored ratio for the fourth scale slot.
export!(cdecl, rw_00e62e70() -> () {
    unsafe {
        let a = *global::<f32>(0x01032338);
        let b = *global::<f32>(0x0103233c);
        *global::<f32>(0x01173690) = a / b;
    }
});
