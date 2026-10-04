// original: 0x00e6dc90 ratio_store_15
/// Ratio of two single-precision globals stored into a third.
///
/// Reads the dividend from one global float and the divisor from the
/// neighbouring word, divides, and stores the quotient into the
/// destination global. Single-precision division is one correctly
/// rounded operation, so this matches the original bit for bit,
/// including zeros, subnormals, infinities and NaN inputs.
export!(cdecl, rw_00e6dc90() -> () {
    unsafe {
        let dividend = *global::<f32>(0x01057D98);
        let divisor = *global::<f32>(0x01057D9C);
        *global::<f32>(0x017AB570) = dividend / divisor;
    }
});
