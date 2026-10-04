// original: 0x00e63320 store_ratio_118e7dc
/// Stores the quotient of two global floats into a third global.
///
/// Computes `*0x118E7DC = *0x1033128 / *0x103312C` in single precision.
export!(cdecl, rw_00e63320() -> () {
    unsafe {
        let num = *global::<f32>(0x1033128);
        let den = *global::<f32>(0x103312C);
        *global::<f32>(0x118E7DC) = num / den;
    }
});
