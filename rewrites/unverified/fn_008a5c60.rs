// original: 0x008A5C60 aud_compute_scaled_round (proposed)

/// Look up a sound entry from the audio bank tables, gather three floats
/// through two intercepted helpers, and return a scaled rounded quotient.
///
/// `this` is an audio object: byte `IDX1` at `+0x40` selects one of 256
/// banks (each `BANK_STRIDE` = 0x6F40 bytes) in the table at global
/// `AUD_TABLE` (0x115D988), and byte `IDX2` at `+0x48` indexes inside the
/// bank with element stride from global `AUD_STRIDE` (0x115D964). The entry
/// pointer `P = table[IDX1] + AUD_STRIDE * IDX2` is found at bank offset
/// `BANK_ENTRY` = 0x6F10.
///
/// Early failures return -1 WITHOUT touching `*out`: `IDX2 == 0xFF`,
/// `P == 0`, helper 1 answers -1, helper 2 answers with a zero low byte.
/// Helper 1 (callee id 1, thiscall on `P`) takes a flag byte that this
/// function never reads back; helper 2 (callee id 2, thiscall on `this`)
/// fills three out-words `L18`, `L14`, `L1C`.
///
/// Otherwise let `d = L14 - L18` and `c = L1C` (both single precision, in
/// that operand order). The original's chain of `comiss`/`ucomiss` jumps
/// computes iff `c` is NaN or (`c > 0` and `d` is not below zero); every
/// other combination returns -1 and stores 1 to `*out` (unless null).
/// The compute path stores 0 to `*out`, forms `x = (d + 1) / c`, rounds to
/// the nearest integer with the 2^23 magnitude trick (sign taken from `x`
/// itself), subtracts 1 when the rounded value is not below the signed zero
/// of `x`, truncates toward zero with `cvttss2si` semantics (NaN, infinities
/// and out-of-range magnitudes yield `i32::MIN`, never saturate), and
/// returns that times helper 1's answer with wrapping multiplication.
///
/// Original: 0x008A5C60 (thiscall, `this` in ECX, one stack word `out`,
/// callee pops 4, returns EAX).
lf_checker_rt::export!(thiscall, rw_008A5C60(this: u32, out: u32) -> u32 {
    unsafe {
        const AUD_STRIDE: u32 = 0x115D964;
        const AUD_TABLE: u32 = 0x115D988;
        const IDX1: u32 = 0x40;
        const IDX2: u32 = 0x48;
        const BANK_STRIDE: u32 = 0x6F40;
        const BANK_ENTRY: u32 = 0x6F10;
        const NONE: u8 = 0xFF;
        const ROUND_ONE: f32 = 1.0;
        const ROUND_MAGIC: f32 = 8388608.0; // 2^23
        const SIGN_MASK: u32 = 0x8000_0000;
        const ALL_ONES: u32 = 0xFFFF_FFFF;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        /// `cvttss2si`: truncate toward zero; anything unrepresentable
        /// (NaN, infinities, magnitude >= 2^31) yields `i32::MIN`.
        #[inline(always)]
        fn cvtt(v: f32) -> i32 {
            if v.is_nan() || v >= 2147483648.0 || v < -2147483648.0 {
                i32::MIN
            } else {
                v as i32
            }
        }
        /// Early failure: return -1, leave `*out` alone.
        #[inline(always)]
        fn early_fail() -> u32 {
            0xFFFF_FFFF
        }
        /// Float-stage failure: return -1 and store 1 to `*out`.
        #[inline(always)]
        unsafe fn float_fail(out: u32) -> u32 {
            unsafe {
                if out != 0 {
                    (out as *mut u8).write(1);
                }
                0xFFFF_FFFF
            }
        }

        let idx2 = rd8(this + IDX2);
        if idx2 == NONE {
            return early_fail();
        }
        let stride = lf_checker_rt::global::<u32>(AUD_STRIDE).read();
        let base = lf_checker_rt::global::<u32>(AUD_TABLE).read();
        let bank = (rd8(this + IDX1) as u32).wrapping_mul(BANK_STRIDE);
        let slot = base.wrapping_add(bank).wrapping_add(BANK_ENTRY);
        let entry = (slot as *const u32).read_unaligned();
        let p = entry.wrapping_add(stride.wrapping_mul(idx2 as u32));
        if p == 0 {
            return early_fail();
        }
        let mut flag: u32 = 0;
        let r: u32 = lf_checker_rt::callee_thiscall!(
            1,
            u32,
            p,
            &mut flag as *mut u32 as u32
        );
        if r == 0xFFFF_FFFF {
            return early_fail();
        }
        let mut l18: u32 = 0;
        let mut l14: u32 = 0;
        let mut l1c: u32 = 0;
        let ans: u32 = lf_checker_rt::callee_thiscall!(
            2,
            u32,
            this,
            &mut l18 as *mut u32 as u32,
            &mut l14 as *mut u32 as u32,
            &mut l1c as *mut u32 as u32
        );
        if ans & 0xFF == 0 {
            return early_fail();
        }
        let d = sub(f32::from_bits(l14), f32::from_bits(l18));
        let c = f32::from_bits(l1c);
        // The original's comiss/ucomiss/lahf chain, verified against the
        // flag table: compute iff c is NaN or (c > 0 and d is not < 0).
        let compute = c.is_nan() || (c > 0.0 && !(d < 0.0));
        if !compute {
            return float_fail(out);
        }
        if out != 0 {
            (out as *mut u8).write(0);
        }
        // Round (d + 1) / c to nearest, in the original's operation order.
        let mut x = div(add(d, ROUND_ONE), c);
        let sign = x.to_bits() & SIGN_MASK;
        let mag = f32::from_bits(x.to_bits() ^ sign);
        let mut m: u32 = if mag < ROUND_MAGIC { 0x4B00_0000 } else { 0 };
        m |= sign;
        let off = f32::from_bits(m);
        let mut q = add(x, off);
        q = sub(q, off);
        let back = sub(q, x);
        // cmpnless(back, signed zero): not-less-or-equal, i.e. true unless
        // back <= zero; unordered (NaN) counts as true.
        let adj = if !(back <= f32::from_bits(sign)) {
            ALL_ONES
        } else {
            0
        } & ROUND_ONE.to_bits();
        q = sub(q, f32::from_bits(adj));
        x = q;
        cvtt(x).wrapping_mul(r as i32) as u32
    }
});
