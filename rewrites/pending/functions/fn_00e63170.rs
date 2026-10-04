// original: 0x00e63170 store_ratio_117e6c4
/// Stores the quotient of two global floats into a third global.
///
/// Computes `*0x117E6C4 = *0x10330F0 / *0x10330F4` in single precision
/// (one `divss`, no other observable effect).
export!(cdecl, rw_00e63170() -> () {
    unsafe {
        let num = *global::<f32>(0x10330F0);
        let den = *global::<f32>(0x10330F4);
        *global::<f32>(0x117E6C4) = num / den;
    }
});
