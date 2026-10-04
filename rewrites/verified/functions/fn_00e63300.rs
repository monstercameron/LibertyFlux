// original: 0x00e63300 store_ratio_118debc
/// Stores the quotient of two global floats into a third global.
///
/// Computes `*0x118DEBC = *0x1033114 / *0x1033118` in single precision.
export!(cdecl, rw_00e63300() -> () {
    unsafe {
        let num = *global::<f32>(0x1033114);
        let den = *global::<f32>(0x1033118);
        *global::<f32>(0x118DEBC) = num / den;
    }
});
