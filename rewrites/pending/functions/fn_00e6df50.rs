// original: 0x00e6df50 ratio_update_16
/// Refresh the ratio stored at 0x17ACC80.
///
/// Divides the float at 0x10596CC by the float at 0x10596D0 and stores
/// the quotient back, returning it (the original leaves it in
/// the low lane of xmm0 as well).
export!(cdecl, rw_00e6df50() -> f32 {
    unsafe {
        const DIVIDEND: u32 = 0x10596CC;
        const DIVISOR: u32 = 0x10596D0;
        const SLOT: u32 = 0x17ACC80;
        let quotient = *global::<f32>(DIVIDEND) / *global::<f32>(DIVISOR);
        *global::<f32>(SLOT) = quotient;
        quotient
    }
});
