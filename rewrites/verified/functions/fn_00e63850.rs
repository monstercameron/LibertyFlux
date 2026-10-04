// original: 0x00e63850 f32_ratio_store_9
/// Store the ratio of two f32 globals into a third: `C = A / B`.
///
/// Same shape as [`rw_00e634c0`] at a ninth address triple.
export!(cdecl, rw_00e63850() -> () {
    unsafe {
        let a = *global::<f32>(0x1036F04);
        let b = *global::<f32>(0x1036F08);
        *global::<f32>(0x11A4FA4) = a / b;
    }
});
