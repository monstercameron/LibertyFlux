// original: 0x00e63360 store_ratio_118e924
/// Stores the quotient of two global floats into a third global.
///
/// Computes `*0x118E924 = *0x1034468 / *0x103446C` in single precision.
export!(cdecl, rw_00e63360() -> () {
    unsafe {
        let num = *global::<f32>(0x1034468);
        let den = *global::<f32>(0x103446C);
        *global::<f32>(0x118E924) = num / den;
    }
});
