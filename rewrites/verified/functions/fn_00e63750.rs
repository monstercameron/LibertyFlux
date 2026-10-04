// original: 0x00e63750 f32_ratio_store_5
/// Store the ratio of two f32 globals into a third: `C = A / B`.
///
/// Same shape as [`rw_00e634c0`] at a fifth address triple.
export!(cdecl, rw_00e63750() -> () {
    unsafe {
        let a = *global::<f32>(0x1036BDC);
        let b = *global::<f32>(0x1036BE0);
        *global::<f32>(0x11A2A68) = a / b;
    }
});
