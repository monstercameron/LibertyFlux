// original: 0x00e6de10 ratio_update_07
/// Refresh the ratio stored at 0x17AB5A0.
///
/// Divides the float at 0x1057F5C by the float at 0x1057F60 and stores
/// the quotient back, returning it (the original leaves it in
/// the low lane of xmm0 as well).
export!(cdecl, rw_00e6de10() -> f32 {
    unsafe {
        const DIVIDEND: u32 = 0x1057F5C;
        const DIVISOR: u32 = 0x1057F60;
        const SLOT: u32 = 0x17AB5A0;
        let quotient = *global::<f32>(DIVIDEND) / *global::<f32>(DIVISOR);
        *global::<f32>(SLOT) = quotient;
        quotient
    }
});
