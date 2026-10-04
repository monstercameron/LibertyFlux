// original: 0x00e6ddf0 ratio_update_06
/// Refresh the ratio stored at 0x17AB59C.
///
/// Divides the float at 0x1057F30 by the float at 0x1057F34 and stores
/// the quotient back, returning it (the original leaves it in
/// the low lane of xmm0 as well).
export!(cdecl, rw_00e6ddf0() -> f32 {
    unsafe {
        const DIVIDEND: u32 = 0x1057F30;
        const DIVISOR: u32 = 0x1057F34;
        const SLOT: u32 = 0x17AB59C;
        let quotient = *global::<f32>(DIVIDEND) / *global::<f32>(DIVISOR);
        *global::<f32>(SLOT) = quotient;
        quotient
    }
});
