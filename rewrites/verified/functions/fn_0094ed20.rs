// original: 0x0094ED20 accumulate_validated_pairs (proposed)

/// Scale and accumulate six validated pairs from a truncated float product.
///
/// Multiplies the global floats `FA` and `FB` (single-precision, the
/// original's operand order pinned) and truncates the product to a signed
/// 64-bit integer exactly as the original's x87 `fistp` with the
/// round-toward-zero control word does: in-range values truncate toward
/// zero, while NaN, infinities and out-of-range magnitudes yield the
/// indefinite value (low word 0). Only the low 32 bits are kept.
/// Then, for each of the six (key, accumulator) pairs at `this + 4 + i * 8`:
/// the key resolves through callee 1 (thiscall, the context word `CTX` in
/// ECX); a null answer marks the key -1 and zeroes the accumulator, while a
/// live answer is validated through callee 2 (thiscall, `this` in ECX) and,
/// on a non-zero LOW byte, the truncated product is added (wrapping) to the
/// accumulator, else the accumulator is zeroed. No return channel.
///
/// Original: 0x0094ED20 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_0094ED20(this: u32) -> u32 {
    unsafe {
        const FA: u32 = 0x11735BC;
        const FB: u32 = 0xFE8C58;
        const CTX: u32 = 0x12E22A4;
        const PAIRS: u32 = 6;
        const RESOLVE: u32 = 1;
        const VALIDATE: u32 = 2;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        let a = f32::from_bits((lf_checker_rt::global::<u32>(FA) as *const u32).read());
        let b = f32::from_bits((lf_checker_rt::global::<u32>(FB) as *const u32).read());
        let prod = mul(a, b);
        let low = if prod.is_nan()
            || prod >= 9223372036854775808.0
            || prod < -9223372036854775808.0
        {
            0u32
        } else {
            (prod as i64) as u32
        };
        let ctx = (lf_checker_rt::global::<u32>(CTX) as *const u32).read();
        let mut i = 0u32;
        while i < PAIRS {
            let pair = this.wrapping_add(4).wrapping_add(i.wrapping_mul(8));
            let key = (pair.wrapping_sub(4) as *const u32).read_unaligned();
            let r = lf_checker_rt::callee_thiscall!(RESOLVE, u32, ctx, key);
            if r == 0 {
                (pair.wrapping_sub(4) as *mut u32).write_unaligned(0xFFFF_FFFF);
                (pair as *mut u32).write_unaligned(0);
            } else {
                let v = lf_checker_rt::callee_thiscall!(VALIDATE, u32, this, key);
                if (v & 0xFF) != 0 {
                    let acc = (pair as *const u32).read_unaligned();
                    (pair as *mut u32).write_unaligned(acc.wrapping_add(low));
                } else {
                    (pair as *mut u32).write_unaligned(0);
                }
            }
            i += 1;
        }
        0
    }
});
