// original: 0x00e6ddd0 ratio_update_05
/// Refresh the ratio stored at 0x17AB598.
///
/// Divides the float at 0x1057F0C by the float at 0x1057F10 and stores
/// the quotient back, returning it (the original leaves it in
/// the low lane of xmm0 as well).
export!(cdecl, rw_00e6ddd0() -> f32 {
    unsafe {
        const DIVIDEND: u32 = 0x1057F0C;
        const DIVISOR: u32 = 0x1057F10;
        const SLOT: u32 = 0x17AB598;
        let quotient = *global::<f32>(DIVIDEND) / *global::<f32>(DIVISOR);
        *global::<f32>(SLOT) = quotient;
        quotient
    }
});
