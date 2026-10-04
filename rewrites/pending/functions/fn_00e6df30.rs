// original: 0x00e6df30 ratio_update_15
/// Refresh the ratio stored at 0x17ACC7C.
///
/// Divides the float at 0x10596A4 by the float at 0x10596A8 and stores
/// the quotient back, returning it (the original leaves it in
/// the low lane of xmm0 as well).
export!(cdecl, rw_00e6df30() -> f32 {
    unsafe {
        const DIVIDEND: u32 = 0x10596A4;
        const DIVISOR: u32 = 0x10596A8;
        const SLOT: u32 = 0x17ACC7C;
        let quotient = *global::<f32>(DIVIDEND) / *global::<f32>(DIVISOR);
        *global::<f32>(SLOT) = quotient;
        quotient
    }
});
