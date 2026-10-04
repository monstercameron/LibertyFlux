// original: 0x00db19a0 UILayoutFrame::vf26
/// Store a mode word, then blend and forward a clamped amount.
///
/// The mode word is always stored. Mode 1 stops there. Any other mode
/// clamps the amount into [0, 1] (NaN passes through unchanged), blends it
/// with two stored factors using single-precision arithmetic, and forwards
/// the mode plus the blended bits to the frame's blend handler. The return
/// value is only defined on the forwarding path, so callers must ignore it.
export!(thiscall, rw_00db19a0(this_ptr: u32, mode: u32, amount_bits: u32) -> u32 {
    unsafe {
        use core::arch::x86::{
            _mm_add_ss, _mm_cvtss_f32, _mm_mul_ss, _mm_set_ss, _mm_sub_ss,
        };
        const MODE_SLOT: usize = 0x14;
        const FACTOR_A_SLOT: usize = 0x08;
        const FACTOR_B_SLOT: usize = 0x10;
        const BLEND_HANDLER_SLOT: usize = 0x60;
        const CLAMP_HI: u32 = 0x00fe88e8;
        const HALF_FACTOR: u32 = 0x00fe8830;

        *(this_ptr as *mut u32).add(MODE_SLOT / 4) = mode;
        if mode != 1 {
            let hi = f32::from_bits(*global::<u32>(CLAMP_HI));
            let half = f32::from_bits(*global::<u32>(HALF_FACTOR));
            let mut clamped = f32::from_bits(amount_bits);
            // Ordered comparisons only: NaN keeps its bits, matching the
            // original's comiss/ja/jbe pair branch for branch.
            if clamped < 0.0 {
                clamped = 0.0;
            } else if clamped > hi {
                clamped = hi;
            }
            let base = this_ptr as *const u8;
            let a = f32::from_bits(*((base.add(FACTOR_A_SLOT)) as *const u32));
            let b = f32::from_bits(*((base.add(FACTOR_B_SLOT)) as *const u32));
            let scaled = _mm_cvtss_f32(_mm_mul_ss(_mm_set_ss(a), _mm_set_ss(half)));
            let weighted = _mm_cvtss_f32(_mm_mul_ss(_mm_set_ss(a), _mm_set_ss(clamped)));
            let tmp = _mm_cvtss_f32(_mm_sub_ss(_mm_set_ss(b), _mm_set_ss(scaled)));
            let blended = _mm_cvtss_f32(_mm_add_ss(_mm_set_ss(tmp), _mm_set_ss(weighted)));
            // Same load-and-call through the object's handler table.
            let table = *(this_ptr as *const u32) as usize;
            let target = *((table + BLEND_HANDLER_SLOT) as *const u32) as usize;
            let handler: extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(target);
            handler(this_ptr, mode, blended.to_bits());
        }
        0
    }
});
