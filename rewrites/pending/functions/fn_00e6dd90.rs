// original: 0x00e6dd90 ratio_update_03
/// Refresh the ratio stored at 0x17AB590.
///
/// Divides the float at 0x1057EB8 by the float at 0x1057EBC and stores
/// the quotient back, returning it (the original leaves it in
/// the low lane of xmm0 as well).
export!(cdecl, rw_00e6dd90() -> f32 {
    unsafe {
        const DIVIDEND: u32 = 0x1057EB8;
        const DIVISOR: u32 = 0x1057EBC;
        const SLOT: u32 = 0x17AB590;
        let quotient = *global::<f32>(DIVIDEND) / *global::<f32>(DIVISOR);
        *global::<f32>(SLOT) = quotient;
        quotient
    }
});
