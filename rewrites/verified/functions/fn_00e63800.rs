// original: 0x00e63800 f32_ratio_store_7
/// Store the ratio of two f32 globals into a third: `C = A / B`.
///
/// Same shape as [`rw_00e634c0`] at a seventh address triple.
export!(cdecl, rw_00e63800() -> () {
    unsafe {
        let a = *global::<f32>(0x1036EA8);
        let b = *global::<f32>(0x1036EAC);
        *global::<f32>(0x11A2EB4) = a / b;
    }
});
