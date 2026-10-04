// original: 0x00e63280 store_ratio_117e6fc
/// Stores the quotient of two global floats into a third global.
///
/// Computes `*0x117E6FC = *0x1033100 / *0x1033104` in single precision.
export!(cdecl, rw_00e63280() -> () {
    unsafe {
        let num = *global::<f32>(0x1033100);
        let den = *global::<f32>(0x1033104);
        *global::<f32>(0x117E6FC) = num / den;
    }
});
