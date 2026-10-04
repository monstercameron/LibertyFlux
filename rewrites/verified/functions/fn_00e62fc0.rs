// original: 0x00e62fc0 refresh_ratio_11764c8
/// Refresh the stored ratio for the tenth scale slot.
export!(cdecl, rw_00e62fc0() -> () {
    unsafe {
        let a = *global::<f32>(0x01032f60);
        let b = *global::<f32>(0x01032f64);
        *global::<f32>(0x011764c8) = a / b;
    }
});
