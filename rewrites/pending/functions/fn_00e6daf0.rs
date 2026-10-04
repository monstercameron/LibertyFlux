// original: 0x00e6daf0 ratio_store_02
/// Ratio of two single-precision globals stored into a third.
///
/// Reads the dividend from one global float and the divisor from the
/// neighbouring word, divides, and stores the quotient into the
/// destination global. Single-precision division is one correctly
/// rounded operation, so this matches the original bit for bit,
/// including zeros, subnormals, infinities and NaN inputs.
export!(cdecl, rw_00e6daf0() -> () {
    unsafe {
        let dividend = *global::<f32>(0x01057AFC);
        let divisor = *global::<f32>(0x01057B00);
        *global::<f32>(0x017AB534) = dividend / divisor;
    }
});
