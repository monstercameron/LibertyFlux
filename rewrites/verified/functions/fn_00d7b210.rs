// original: 0x00D7B210 forward_packed_fields_ne_flag (proposed)

/// Forward packed half-words plus a not-equal flag to the next stage.
///
/// Compares the float at `obj + 0xf08` against 0.0 with an unordered-aware
/// comparison and a flag-parity trick that yields 1 exactly when they
/// differ (NaN counts as different, signed zeros as equal), then calls
/// the next stage with the sign-extended word at `+0x2e`, the zero-extended
/// word at `+0x2c`, a flag word and `arg1`. The flag word's low byte is the
/// bit and its second byte is the status-flag image of the comparison
/// (zero, parity and carry per ordered class, bit 1 fixed); bits 16-31 are
/// the caller's incoming register value (the contract fixes it to 0).
/// Returns the callee's answer. Cdecl, two stack words.
use lf_checker_rt::{callee_cdecl, export};

const NEXT: u32 = 1;

export!(cdecl, rw_00d7b210(obj: u32, arg1: u32) -> u32 {
    unsafe {
        const FLOAT_OFF: u32 = 0xf08;
        const LO_OFF: u32 = 0x2c;
        const HI_OFF: u32 = 0x2e;
        let x = f32::from_bits(((obj + FLOAT_OFF) as *const u32).read_unaligned());
        // Status-flag image: (zero, parity, carry) per ordered class.
        let (zf, pf, cf) = if x.is_nan() {
            (true, true, true)
        } else if x > 0.0 {
            (false, false, false)
        } else if x < 0.0 {
            (false, false, true)
        } else {
            (true, false, false)
        };
        let bit = u32::from(zf == pf);
        let ah = (u32::from(zf) << 6) | (u32::from(pf) << 2) | u32::from(cf) | 0x02;
        let flag = (ah << 8) | bit;
        let lo = ((obj + LO_OFF) as *const u16).read_unaligned() as u32;
        let hi = ((obj + HI_OFF) as *const i16).read_unaligned() as i32 as u32;
        callee_cdecl!(NEXT, u32, hi, lo, flag, arg1)
    }
});
