// original: 0x00e63830 f32_ratio_store_8
/// Store the ratio of two f32 globals into a third: `C = A / B`.
///
/// Same shape as [`rw_00e634c0`] at an eighth address triple.
export!(cdecl, rw_00e63830() -> () {
    unsafe {
        let a = *global::<f32>(0x1036EFC);
        let b = *global::<f32>(0x1036F00);
        *global::<f32>(0x11A4F1C) = a / b;
    }
});
