// original: 0x009e8150 ped_heading_to_index

/// Heading index from a float pair and a stored heading.
///
/// `this` points at a record holding a float heading at `+HEAD_OFF` and
/// `p` points at two consecutive floats (x, y). The function negates x,
/// widens both to doubles exactly as the widening conversion does, calls
/// the double arctangent callee with the pair in the two vector
/// registers, then narrows the answer to a float, subtracts the stored
/// heading and adds a quarter turn, in that order, all round to nearest.
/// A negative result is raised by whole turns until it reaches zero;
/// the branch out is also taken for not-a-number, which then flows to
/// the conversion below. The result is scaled by the reciprocal of half
/// a turn and truncated toward zero to an integer, where not-a-number
/// and out-of-range values give the integer indefinite; an index above
/// 3 is wrapped into its turn remainder. The integer is the return.
///
/// The callee's first vector register keeps a constant sign-mask
/// leftover in its upper half on the original side, which the transport
/// zeroes on the rewrite side; only the low doubles compare there.
///
/// Original: 0x009e8150 (thiscall, one stack word: the pair pointer).
lf_checker_rt::export!(thiscall, rw_009e8150(this: u32, p: u32) -> u32 {
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
            let lz = (f as u32).leading_zeros() - 9;
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
    /// Truncating float-to-integer conversion as the instruction does it:
    /// not-a-number and values outside the signed range give i32::MIN.
    fn cvtt_i32(x: f32) -> i32 {
        if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
            i32::MIN
        } else {
            // Inside the range the cast truncates exactly like the instruction.
            x as i32
        }
    }
    unsafe {
        const SIGN_MASK_AT: u32 = 0x00FE8FA0;
        const QUARTER_TURN_AT: u32 = 0x00FE8894;
        const FULL_TURN_AT: u32 = 0x00FE8AEC;
        const RECIP_HALF_TURN_AT: u32 = 0x00E98888;
        const HEAD_OFF: u32 = 0xAA0;
        let sign_mask = lf_checker_rt::global::<u32>(SIGN_MASK_AT).read_unaligned();
        let quarter = lf_checker_rt::global::<f32>(QUARTER_TURN_AT).read_unaligned();
        let full = lf_checker_rt::global::<f32>(FULL_TURN_AT).read_unaligned();
        let recip = lf_checker_rt::global::<f32>(RECIP_HALF_TURN_AT).read_unaligned();
        let f0 = (p as *const u32).read_unaligned();
        let f1 = (p as *const u32).add(1).read_unaligned();
        let (a0, a1) = lo_hi(cvtss2sd_bits(f0 ^ sign_mask));
        let (b0, b1) = lo_hi(cvtss2sd_bits(f1));
        let ans: u64 = lf_checker_rt::callee_cdecl!(1, u64, a0, a1, b0, b1);
        let head = ((this + HEAD_OFF) as *const f32).read_unaligned();
        let narrowed = f64::from_bits(ans) as f32;
        let mut r = core::hint::black_box(narrowed) - core::hint::black_box(head);
        r = core::hint::black_box(r) + core::hint::black_box(quarter);
        if r < 0.0 {
            let step = core::hint::black_box(full);
            while r < 0.0 {
                r = core::hint::black_box(r) + step;
            }
        }
        r = core::hint::black_box(r) * core::hint::black_box(recip);
        let e = cvtt_i32(r);
        if e > 3 {
            let eu = e as u32;
            let edx = eu.wrapping_sub(4) & 0xFFFF_FFFC;
            let ecx = 0xFFFF_FFFCu32.wrapping_sub(edx);
            eu.wrapping_add(ecx)
        } else {
            e as u32
        }
    }
});
