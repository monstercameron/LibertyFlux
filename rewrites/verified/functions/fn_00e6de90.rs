// original: 0x00e6de90 ratio_update_10
/// Refresh the ratio stored at 0x17ACC68.
///
/// Divides the float at 0x10595A0 by the float at 0x10595A4 and stores
/// the quotient back, returning it (the original leaves it in
/// the low lane of xmm0 as well).
export!(cdecl, rw_00e6de90() -> f32 {
    unsafe {
        const DIVIDEND: u32 = 0x10595A0;
        const DIVISOR: u32 = 0x10595A4;
        const SLOT: u32 = 0x17ACC68;
        let quotient = *global::<f32>(DIVIDEND) / *global::<f32>(DIVISOR);
        *global::<f32>(SLOT) = quotient;
        quotient
    }
});
