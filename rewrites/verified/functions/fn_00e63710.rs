// original: 0x00e63710 f32_ratio_store_4
/// Store the ratio of two f32 globals into a third: `C = A / B`.
///
/// Same shape as [`rw_00e634c0`] at a fourth address triple.
export!(cdecl, rw_00e63710() -> () {
    unsafe {
        let a = *global::<f32>(0x1036B84);
        let b = *global::<f32>(0x1036B88);
        *global::<f32>(0x11A28F8) = a / b;
    }
});
