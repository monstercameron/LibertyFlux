// original: 0x00e63000 refresh_ratio_1176d40
/// Refresh the stored ratio for the twelfth scale slot.
export!(cdecl, rw_00e63000() -> () {
    unsafe {
        let a = *global::<f32>(0x01032fe4);
        let b = *global::<f32>(0x01033050);
        *global::<f32>(0x01176d40) = a / b;
    }
});
