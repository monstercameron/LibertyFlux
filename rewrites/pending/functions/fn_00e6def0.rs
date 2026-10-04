// original: 0x00e6def0 ratio_update_13
/// Refresh the ratio stored at 0x17ACC74.
///
/// Divides the float at 0x1059628 by the float at 0x105962C and stores
/// the quotient back, returning it (the original leaves it in
/// the low lane of xmm0 as well).
export!(cdecl, rw_00e6def0() -> f32 {
    unsafe {
        const DIVIDEND: u32 = 0x1059628;
        const DIVISOR: u32 = 0x105962C;
        const SLOT: u32 = 0x17ACC74;
        let quotient = *global::<f32>(DIVIDEND) / *global::<f32>(DIVISOR);
        *global::<f32>(SLOT) = quotient;
        quotient
    }
});
