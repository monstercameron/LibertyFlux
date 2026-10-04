// original: 0x00e636f0 f32_ratio_store_3
/// Store the ratio of two f32 globals into a third: `C = A / B`.
///
/// Same shape as [`rw_00e634c0`] at a third address triple.
export!(cdecl, rw_00e636f0() -> () {
    unsafe {
        let a = *global::<f32>(0x1036AD4);
        let b = *global::<f32>(0x1036AD8);
        *global::<f32>(0x11A1BE4) = a / b;
    }
});
