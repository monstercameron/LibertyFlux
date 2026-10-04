// original: 0x009f6370 scaled_quotient_floor
use lf_k2_rt::{export, callee_addr};

/// Tunable identifiers read by `rw_s18f0`.
const F0_NUM_ID: u32 = 0x123;

/// Tunable identifiers read by `rw_s18f0`.
const F0_DEN_ID: u32 = 0x29;

/// Scale factor applied to the numerator (the original's table constant).
const F0_SCALE: f32 = 100.0;

/// Magic rounding constant, 2^23 (the original's table constant).
const F0_MAGIC: f32 = 8_388_608.0;

/// Scaled quotient with round-down (cdecl/0 -> ST0).
///
/// Reads two tunable floats, returns 0 when either is zero, otherwise rounds
/// `num * 100 / den` down to an integer using the original's add/sub magic
/// sequence with a sign-dependent correction. Bit-exact, including NaN.
export!(cdecl, rw_s18f0() -> f64 {
    unsafe {
        // The stub answers in ST0 and also leaves the bits in eax; take the
        // integer channel so the rewrite never depends on f32 return ABI.
        let get: extern "cdecl" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let num = f32::from_bits(get(F0_NUM_ID));
        let den = f32::from_bits(get(F0_DEN_ID));
        // The original branches on ucomiss-against-zero equality: true only
        // for +0/-0, never for NaN. `== 0.0` matches that exactly.
        if den == 0.0 || num == 0.0 {
            return 0.0;
        }
        let x = num * F0_SCALE / den;
        // Sign of x, then the magic round: (x + m) - m with m = +/-2^23
        // rounds to nearest for |x| < 2^23 and is exact otherwise.
        let sign = x.to_bits() & 0x8000_0000;
        let ax = f32::from_bits(x.to_bits() ^ sign);
        // cmpltss: ordered less-than, false on NaN — like `<` below.
        let mask: u32 = if ax < F0_MAGIC { 0xFFFF_FFFF } else { 0 };
        let m = f32::from_bits((F0_MAGIC.to_bits() & mask) | sign);
        let rounded = (x + m) - m;
        // cmpnles(delta, sign): ordered greater-than (predicate 6), false on
        // NaN or equality — exactly what `>` below computes.
        let delta = rounded - x;
        let s = f32::from_bits(sign);
        let corr: u32 = if delta > s { 0xFFFF_FFFF } else { 0 };
        (rounded - f32::from_bits(1.0f32.to_bits() & corr)) as f64
    }
});
