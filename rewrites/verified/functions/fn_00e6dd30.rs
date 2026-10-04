// original: 0x00e6dd30 ratio_update_00
/// Refresh the ratio stored at 0x17AB584.
///
/// Divides the float at 0x1057E6C by the float at 0x1057E70 and stores
/// the quotient back, returning it (the original leaves it in
/// the low lane of xmm0 as well).
export!(cdecl, rw_00e6dd30() -> f32 {
    unsafe {
        const DIVIDEND: u32 = 0x1057E6C;
        const DIVISOR: u32 = 0x1057E70;
        const SLOT: u32 = 0x17AB584;
        let quotient = *global::<f32>(DIVIDEND) / *global::<f32>(DIVISOR);
        *global::<f32>(SLOT) = quotient;
        quotient
    }
});
