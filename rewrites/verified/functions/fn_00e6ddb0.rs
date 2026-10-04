// original: 0x00e6ddb0 ratio_update_04
/// Refresh the ratio stored at 0x17AB594.
///
/// Divides the float at 0x1057EE8 by the float at 0x1057EEC and stores
/// the quotient back, returning it (the original leaves it in
/// the low lane of xmm0 as well).
export!(cdecl, rw_00e6ddb0() -> f32 {
    unsafe {
        const DIVIDEND: u32 = 0x1057EE8;
        const DIVISOR: u32 = 0x1057EEC;
        const SLOT: u32 = 0x17AB594;
        let quotient = *global::<f32>(DIVIDEND) / *global::<f32>(DIVISOR);
        *global::<f32>(SLOT) = quotient;
        quotient
    }
});
