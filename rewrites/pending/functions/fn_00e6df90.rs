// original: 0x00e6df90 ratio_update_17
/// Refresh the ratio stored at 0x17ACCA8.
///
/// Divides the float at 0x10596F4 by the float at 0x10596F8 and stores
/// the quotient back, returning it (the original leaves it in
/// the low lane of xmm0 as well).
export!(cdecl, rw_00e6df90() -> f32 {
    unsafe {
        const DIVIDEND: u32 = 0x10596F4;
        const DIVISOR: u32 = 0x10596F8;
        const SLOT: u32 = 0x17ACCA8;
        let quotient = *global::<f32>(DIVIDEND) / *global::<f32>(DIVISOR);
        *global::<f32>(SLOT) = quotient;
        quotient
    }
});
