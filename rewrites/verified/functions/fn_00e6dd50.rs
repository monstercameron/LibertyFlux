// original: 0x00e6dd50 ratio_update_01
/// Refresh the ratio stored at 0x17AB588.
///
/// Divides the float at 0x1057E90 by the float at 0x1057E94 and stores
/// the quotient back, returning it (the original leaves it in
/// the low lane of xmm0 as well).
export!(cdecl, rw_00e6dd50() -> f32 {
    unsafe {
        const DIVIDEND: u32 = 0x1057E90;
        const DIVISOR: u32 = 0x1057E94;
        const SLOT: u32 = 0x17AB588;
        let quotient = *global::<f32>(DIVIDEND) / *global::<f32>(DIVISOR);
        *global::<f32>(SLOT) = quotient;
        quotient
    }
});
