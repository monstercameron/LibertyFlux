// original: 0x00e63560 f32_ratio_store_2
/// Store the ratio of two f32 globals into a third: `C = A / B`.
///
/// Same shape as [`rw_00e634c0`] at a second address triple.
export!(cdecl, rw_00e63560() -> () {
    unsafe {
        let a = *global::<f32>(0x1036778);
        let b = *global::<f32>(0x103677C);
        *global::<f32>(0x11A0B00) = a / b;
    }
});
