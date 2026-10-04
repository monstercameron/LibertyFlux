// original: 0x00e6de50 ratio_update_08
/// Refresh the ratio stored at 0x17ACC60.
///
/// Divides the float at 0x1059560 by the float at 0x1059564 and stores
/// the quotient back, returning it (the original leaves it in
/// the low lane of xmm0 as well).
export!(cdecl, rw_00e6de50() -> f32 {
    unsafe {
        const DIVIDEND: u32 = 0x1059560;
        const DIVISOR: u32 = 0x1059564;
        const SLOT: u32 = 0x17ACC60;
        let quotient = *global::<f32>(DIVIDEND) / *global::<f32>(DIVISOR);
        *global::<f32>(SLOT) = quotient;
        quotient
    }
});
