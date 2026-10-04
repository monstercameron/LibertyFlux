// original: 0x00e6dd70 ratio_update_02
/// Refresh the ratio stored at 0x17AB58C.
///
/// Divides the float at 0x1057E98 by the float at 0x1057E9C and stores
/// the quotient back, returning it (the original leaves it in
/// the low lane of xmm0 as well).
export!(cdecl, rw_00e6dd70() -> f32 {
    unsafe {
        const DIVIDEND: u32 = 0x1057E98;
        const DIVISOR: u32 = 0x1057E9C;
        const SLOT: u32 = 0x17AB58C;
        let quotient = *global::<f32>(DIVIDEND) / *global::<f32>(DIVISOR);
        *global::<f32>(SLOT) = quotient;
        quotient
    }
});
