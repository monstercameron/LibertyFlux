// original: 0x00e6deb0 ratio_update_11
/// Refresh the ratio stored at 0x17ACC6C.
///
/// Divides the float at 0x10595E4 by the float at 0x10595E8 and stores
/// the quotient back, returning it (the original leaves it in
/// the low lane of xmm0 as well).
export!(cdecl, rw_00e6deb0() -> f32 {
    unsafe {
        const DIVIDEND: u32 = 0x10595E4;
        const DIVISOR: u32 = 0x10595E8;
        const SLOT: u32 = 0x17ACC6C;
        let quotient = *global::<f32>(DIVIDEND) / *global::<f32>(DIVISOR);
        *global::<f32>(SLOT) = quotient;
        quotient
    }
});
