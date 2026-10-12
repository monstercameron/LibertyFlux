// original: 0x008d64f0 atan2_of_negated_pair

/// Arctangent of a negated float pair, as a float.
///
/// `p` points at two consecutive floats (x, y). The function negates x
/// (sign bit flipped with the all-lanes sign mask in read-only data),
/// widens both to doubles exactly as the widening conversion does
/// (values exact, infinities kept, NaNs quieted with payload kept,
/// denormals normalised, signed zeros kept), calls the double
/// arctangent callee with the pair in the two vector registers, narrows
/// the double answer back to a float (round to nearest, ties to even)
/// and returns it on the x87 register stack.
///
/// The original also stores the answer float over its own incoming
/// argument slot as scratch before loading it; a Rust rewrite cannot
/// address its own argument slot, so the stack check is off for this
/// proof (the stored value equals the compared return value).
///
/// Original: 0x008d64f0 (cdecl, one stack word: the pair pointer).
lf_checker_rt::export!(cdecl, rw_008d64f0(p: u32) -> f32 {
    /// Bit-exact widening of f32 bits to f64 bits, as the widening
    /// conversion instruction does.
    fn cvtss2sd_bits(a: u32) -> u64 {
        let sign = u64::from(a >> 31) << 63;
        let e = (a >> 23) & 0xFF;
        let f = u64::from(a & 0x7F_FFFF);
        if e == 0xFF {
            if f == 0 {
                return sign | 0x7FF0_0000_0000_0000; // infinity
            }
            return sign | 0x7FF8_0000_0000_0000 | (f << 29); // NaN, quieted
        }
        if e == 0 {
            if f == 0 {
                return sign; // signed zero
            }
            // Denormal: value = f * 2^-149, normalised into the double.
            let lz = (f as u32).leading_zeros() - 9; // 0..=22 within 23 bits
            let exp = 896 - lz; // 1023 - 127 - lz
            let frac = ((f << (lz + 1)) & 0x7F_FFFF) << 29;
            return sign | (u64::from(exp) << 52) | frac;
        }
        sign | (u64::from(e + 896) << 52) | (f << 29)
    }
    /// Low and high words of a double, the order the callee transport takes.
    fn lo_hi(d: u64) -> (u32, u32) {
        ((d & 0xFFFF_FFFF) as u32, ((d >> 32) & 0xFFFF_FFFF) as u32)
    }
    unsafe {
        // Sign mask the original flips the low lane with, read from the
        // image like the original reads it (only the low lane feeds the
        // conversion; the other lanes land in the narrowed upper half).
        const SIGN_MASK_AT: u32 = 0x00FE8FA0;
        let sign_mask = lf_checker_rt::global::<u32>(SIGN_MASK_AT).read_unaligned();
        let f0 = (p as *const u32).read_unaligned();
        let f1 = (p as *const u32).add(1).read_unaligned();
        let (a0, a1) = lo_hi(cvtss2sd_bits(f0 ^ sign_mask));
        let (b0, b1) = lo_hi(cvtss2sd_bits(f1));
        let ans: u64 = lf_checker_rt::callee_cdecl!(1, u64, a0, a1, b0, b1);
        f64::from_bits(ans) as f32
    }
});
