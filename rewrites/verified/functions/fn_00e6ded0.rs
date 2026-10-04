// original: 0x00e6ded0 ratio_update_12
/// Refresh the ratio stored at 0x17ACC70.
///
/// Divides the float at 0x1059604 by the float at 0x1059608 and stores
/// the quotient back, returning it (the original leaves it in
/// the low lane of xmm0 as well).
export!(cdecl, rw_00e6ded0() -> f32 {
    unsafe {
        const DIVIDEND: u32 = 0x1059604;
        const DIVISOR: u32 = 0x1059608;
        const SLOT: u32 = 0x17ACC70;
        let quotient = *global::<f32>(DIVIDEND) / *global::<f32>(DIVISOR);
        *global::<f32>(SLOT) = quotient;
        quotient
    }
});
