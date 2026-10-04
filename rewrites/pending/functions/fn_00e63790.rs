// original: 0x00e63790 f32_ratio_store_6
/// Store the ratio of two f32 globals into a third: `C = A / B`.
///
/// Same shape as [`rw_00e634c0`] at a sixth address triple.
export!(cdecl, rw_00e63790() -> () {
    unsafe {
        let a = *global::<f32>(0x1036E88);
        let b = *global::<f32>(0x1036E8C);
        *global::<f32>(0x11A2E90) = a / b;
    }
});
