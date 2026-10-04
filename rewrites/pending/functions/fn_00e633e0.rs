// original: 0x00e633e0 store_ratio_1190e68
/// Stores the quotient of two global floats into a third global.
///
/// Computes `*0x1190E68 = *0x1034488 / *0x103448C` in single precision.
export!(cdecl, rw_00e633e0() -> () {
    unsafe {
        let num = *global::<f32>(0x1034488);
        let den = *global::<f32>(0x103448C);
        *global::<f32>(0x1190E68) = num / den;
    }
});
