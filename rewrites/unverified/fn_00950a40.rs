// original: 0x00950a40 float_chain_converter
/// Converts an integer input through a float scale chain into a ratio count.
///
/// When the second argument is zero the chain starts from a default constant;
/// otherwise a lookup call yields an integer that is widened to an unsigned
/// double through the sign-select table and narrowed back to a float. The
/// value is pre-scaled and passed to the curve probe, whose float answer feeds
/// a truncating conversion (not-a-number and out-of-range values yield the
/// integer indefinite, exactly as the instruction does). The truncated value is
/// post-scaled, widened, and passed with the base double to the power helper;
/// one divided by the helper's narrowed answer, times the first argument as a
/// float, truncates to the returned count. The probe's answer is also stored
/// into the second argument slot by the original; that store is not compared
/// (the stack check is off, see the result record).
use core::hint::black_box;
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, relocated};

/// The processor's truncating float-to-int conversion, bit for bit: NaN and
/// values outside the signed 32-bit range yield the integer indefinite.
#[inline(always)]
fn cvtt_i32(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}

export!(cdecl, rw_00950a40(arg1: u32, arg2: u32) -> u32 {
    unsafe {
        /// Default chain input when the second argument is zero.
        const ZERO_DEFAULT: u32 = 0x00E7_58F8;
        /// Sign-select table that completes the unsigned widening.
        const U32_TABLE: u32 = 0x00FE_8F50;
        /// Scale applied before the curve probe.
        const PRE_SCALE: u32 = 0x00E8_AE9C;
        /// Curve object passed as the probe's this pointer.
        const CURVE_OBJ: u32 = 0x0116_8B3C;
        /// Base double passed as the power helper's first argument.
        const BASE_DBL: u32 = 0x00FE_8A28;
        /// Scale applied after the truncating conversion.
        const POST_SCALE: u32 = 0x00E7_8574;
        /// Unit numerator of the final ratio.
        const UNIT: u32 = 0x00FE_88E8;
        /// Lookup callee id.
        const LOOKUP: u32 = 1;
        /// Curve probe callee id.
        const PROBE: u32 = 2;
        /// Power helper callee id.
        const POWER: u32 = 3;

        let base = if arg2 == 0 {
            *(relocated(ZERO_DEFAULT) as *const f32)
        } else {
            let r: u32 = callee_cdecl!(LOOKUP, u32, arg2);
            let sel = r >> 31;
            let adj = *(relocated(U32_TABLE).wrapping_add(sel * 8) as *const f64);
            let wide = black_box((r as i32) as f64) + black_box(adj);
            black_box(wide) as f32
        };
        let k1 = *(relocated(PRE_SCALE) as *const f32);
        let probe_in = black_box(base) * black_box(k1);
        let f: f32 = callee_thiscall!(PROBE, f32, relocated(CURVE_OBJ), probe_in.to_bits());
        let t = cvtt_i32(black_box(f));
        let k2 = *(relocated(POST_SCALE) as *const f32);
        let v = black_box(t as f32) * black_box(k2);
        let w: f64 = black_box(v) as f64;
        let base_bits = *(relocated(BASE_DBL) as *const u64);
        let w_bits = w.to_bits();
        let answer: u64 = callee_cdecl!(
            POWER,
            u64,
            (base_bits & 0xFFFF_FFFF) as u32,
            (base_bits >> 32) as u32,
            (w_bits & 0xFFFF_FFFF) as u32,
            (w_bits >> 32) as u32
        );
        let one = *(relocated(UNIT) as *const f32);
        let denom: f32 = black_box(f64::from_bits(answer)) as f32;
        let ratio = black_box(one) / black_box(denom);
        let m = black_box(arg1 as i32) as f32;
        cvtt_i32(black_box(ratio) * black_box(m)) as u32
    }
});
