// original: 0x00967e20 timing_range_mark (proposed)

/// Convert four callee-supplied doubles to integers, derive their quotient
/// and remainder ranges over 120, and mark a byte table for the covered
/// span.
///
/// Each of the four stack arguments is passed in turn (with `this`) to one
/// of four intercepted callees (ids 1-4, one call site each, thiscall, one
/// stack argument, double result on the x87 stack). Each result is converted
/// with chop toward zero exactly like `fistp` (out-of-range or NaN yields
/// `0x8000000000000000`, of which the low word is kept), giving `v1..v4`.
///
/// The four values are split into quotients `q = v / 120` and remainders
/// `r = v % 120` (unsigned), and the minima and maxima of each are formed,
/// with the maxima clamped to 0x77 for the table walk. Every integer `b`
/// from `min(v)` to `max(v)` whose own quotient and remainder both fall
/// inside those ranges gets bit 0x80 set in the table byte at
/// `TABLE_BASE + b * 2`. The function returns a leftover of the last
/// quotient computation (`MAGIC * ((max(v) / 120) * 120)`, low word); the
/// `min > max` early exit cannot happen for a minimum and maximum and is
/// kept only for shape.
///
/// Original: thiscall, four stack words, callee cleans them.
lf_checker_rt::export!(thiscall, rw_00967e20(this: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const TABLE_BASE: u32 = 0x0121_8559;
        const DIVISOR: u32 = 120;
        const CLAMP: u32 = 0x77;
        const MAGIC: u32 = 0x8888_8889;
        const MARK: u8 = 0x80;

        /// Chop a double to 64 bits exactly like `fistp qword` under a
        /// truncating control word; returns the low 32 bits kept by the
        /// original.
        fn fistp_low(x: f64) -> u32 {
            const TWO63: f64 = 9223372036854775808.0;
            if x.is_nan() || x >= TWO63 || x <= -TWO63 {
                0x8000_0000u32
            } else {
                (x as i64) as u32
            }
        }
        #[inline(always)]
        unsafe fn mark(base: u32, b: u32) {
            unsafe {
                let p = base.wrapping_add(b.wrapping_mul(2));
                let cur = (p as *const u8).read();
                (p as *mut u8).write(cur | MARK);
            }
        }

        let d1: f64 = lf_checker_rt::callee_thiscall!(1, f64, this, a1);
        let d2: f64 = lf_checker_rt::callee_thiscall!(2, f64, this, a2);
        let d3: f64 = lf_checker_rt::callee_thiscall!(3, f64, this, a3);
        let d4: f64 = lf_checker_rt::callee_thiscall!(4, f64, this, a4);
        let (v1, v2, v3, v4) = (fistp_low(d1), fistp_low(d2), fistp_low(d3), fistp_low(d4));
        let (q1, r1) = (v1 / DIVISOR, v1 % DIVISOR);
        let (q2, r2) = (v2 / DIVISOR, v2 % DIVISOR);
        let (q3, r3) = (v3 / DIVISOR, v3 % DIVISOR);
        let (q4, r4) = (v4 / DIVISOR, v4 % DIVISOR);
        let min_v = v1.min(v2).min(v3).min(v4);
        let max_v = v1.max(v2).max(v3).max(v4);
        let min_r = r1.min(r2).min(r3).min(r4);
        let max_r = r1.max(r2).max(r3).max(r4).min(CLAMP);
        let min_q = q1.min(q2).min(q3).min(q4);
        let max_q = q1.max(q2).max(q3).max(q4).min(CLAMP);
        if min_v > max_v {
            return min_q;
        }
        let table = lf_checker_rt::relocated(TABLE_BASE);
        let mut b = min_v;
        loop {
            let (qb, rb) = (b / DIVISOR, b % DIVISOR);
            if rb >= min_r && rb <= max_r && qb >= min_q && qb <= max_q {
                mark(table, b);
            }
            if b == max_v {
                break;
            }
            b = b.wrapping_add(1);
        }
        MAGIC.wrapping_mul((max_v / DIVISOR).wrapping_mul(DIVISOR))
    }
});
