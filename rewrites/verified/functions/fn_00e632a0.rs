// original: 0x00e632a0 store_ratio_118d7e4
/// Stores the quotient of two global floats into a third global.
///
/// Computes `*0x118D7E4 = *0x103310C / *0x1033110` in single precision.
export!(cdecl, rw_00e632a0() -> () {
    unsafe {
        let num = *global::<f32>(0x103310C);
        let den = *global::<f32>(0x1033110);
        *global::<f32>(0x118D7E4) = num / den;
    }
});
