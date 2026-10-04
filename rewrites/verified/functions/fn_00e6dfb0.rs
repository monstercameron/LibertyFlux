// original: 0x00e6dfb0 ratio_update_18
/// Refresh the ratio stored at 0x17ACCAC.
///
/// Divides the float at 0x10596FC by the float at 0x1059700 and stores
/// the quotient back, returning it (the original leaves it in
/// the low lane of xmm0 as well).
export!(cdecl, rw_00e6dfb0() -> f32 {
    unsafe {
        const DIVIDEND: u32 = 0x10596FC;
        const DIVISOR: u32 = 0x1059700;
        const SLOT: u32 = 0x17ACCAC;
        let quotient = *global::<f32>(DIVIDEND) / *global::<f32>(DIVISOR);
        *global::<f32>(SLOT) = quotient;
        quotient
    }
});
