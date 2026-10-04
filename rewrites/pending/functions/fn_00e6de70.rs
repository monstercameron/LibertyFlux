// original: 0x00e6de70 ratio_update_09
/// Refresh the ratio stored at 0x17ACC64.
///
/// Divides the float at 0x1059590 by the float at 0x1059594 and stores
/// the quotient back, returning it (the original leaves it in
/// the low lane of xmm0 as well).
export!(cdecl, rw_00e6de70() -> f32 {
    unsafe {
        const DIVIDEND: u32 = 0x1059590;
        const DIVISOR: u32 = 0x1059594;
        const SLOT: u32 = 0x17ACC64;
        let quotient = *global::<f32>(DIVIDEND) / *global::<f32>(DIVISOR);
        *global::<f32>(SLOT) = quotient;
        quotient
    }
});
