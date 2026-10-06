// original: 0x00D7E040 projection_sign_test (proposed)

/// Compare two dot products of a delta vector and return the sign.
///
/// `a` and `b` point to objects whose word at `+0x20` points at float
/// blocks. With `d[i] = q[0x30+i] - p[0x30+i]` for `i` in 0..3 (x/y/z),
/// forms `s = p[0x14]*dy + p[0x10]*dx + p[0x18]*dz` and
/// `t = q[0x14]*dy + q[0x10]*dx + q[0x18]*dz`, then returns 1 when `-t`
/// is strictly above `s` (ordered compare, NaN gives 0), else 0. Cdecl,
/// two stack words. Float operation order is the original's.
use lf_checker_rt::export;

export!(cdecl, rw_00d7e040(a: u32, b: u32) -> u32 {
    unsafe {
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        const BLK_OFF: u32 = 0x20;
        const SIGN: u32 = 0x8000_0000;
        let p = ((a + BLK_OFF) as *const u32).read_unaligned();
        let q = ((b + BLK_OFF) as *const u32).read_unaligned();
        let dx = sub(rdf(q + 0x30), rdf(p + 0x30));
        let dy = sub(rdf(q + 0x34), rdf(p + 0x34));
        let dz = sub(rdf(q + 0x38), rdf(p + 0x38));
        let s = add(add(mul(rdf(p + 0x14), dy), mul(rdf(p + 0x10), dx)), mul(rdf(p + 0x18), dz));
        let t = add(add(mul(rdf(q + 0x14), dy), mul(rdf(q + 0x10), dx)), mul(rdf(q + 0x18), dz));
        let tn = f32::from_bits(t.to_bits() ^ SIGN);
        u32::from(tn > s)
    }
});
