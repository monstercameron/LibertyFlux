// original: 0x00e6df10 ratio_update_14
/// Refresh the ratio stored at 0x17ACC78.
///
/// Divides the float at 0x105964C by the float at 0x1059650 and stores
/// the quotient back, returning it (the original leaves it in
/// the low lane of xmm0 as well).
export!(cdecl, rw_00e6df10() -> f32 {
    unsafe {
        const DIVIDEND: u32 = 0x105964C;
        const DIVISOR: u32 = 0x1059650;
        const SLOT: u32 = 0x17ACC78;
        let quotient = *global::<f32>(DIVIDEND) / *global::<f32>(DIVISOR);
        *global::<f32>(SLOT) = quotient;
        quotient
    }
});
