// original: 0x00c03fd0 stream_quat_get_b (proposed)

/// Decode a packed direction (two signed bytes) into a unit float vector.
///
/// `this` points to the object, `out` to three caller-owned floats. The bytes
/// at `+0x20` and `+0x21` are sign-extended, converted to float and
/// scaled by `K_SCALE`, giving `x` and `y`. `z` is `sqrt(|1 - x*x - y*y|)`,
/// except that an exactly-zero remainder (byte pairs such as (127, 0), where
/// the scaled value rounds to exactly 1) yields `z = 1`; when bit `0x80`
/// of the byte at `+0x17` is set, `z` is negated. The vector is then
/// normalised by `1 / sqrt(y*y + x*x + z*z)` (a zero length, unreachable from
/// byte inputs, would zero the output instead). Float operation order is the
/// original's. The original leaves flag soup in `eax` (two `lahf`), so no
/// return channel is compared.
///
/// Original: 0x00c03fd0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c03fd0(this: u32, out: u32) -> u32 {
    unsafe {
        const OFF_X: u32 = 0x20;
        const OFF_Y: u32 = 0x21;
        const OFF_FLAGS: u32 = 0x17;
        const SIGN_BIT: u8 = 0x80;
        const K_SCALE: f32 = f32::from_bits(0x3c010204);
        const K_ONE: f32 = 1.0;
        const K_NEG_ONE: f32 = -1.0;
        const FABS_MASK: u32 = 0x7fff_ffff;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        let x = mul((rd8(this.wrapping_add(OFF_X)) as i8 as i32) as f32, K_SCALE);
        wrf(out, x);
        let y = mul((rd8(this.wrapping_add(OFF_Y)) as i8 as i32) as f32, K_SCALE);
        wrf(out.wrapping_add(4), y);
        let mut t = K_ONE;
        t = sub(t, mul(x, x));
        t = sub(t, mul(y, y));
        t = f32::from_bits(t.to_bits() & FABS_MASK);
        let mut z = if t == 0.0 { K_ONE } else { t.sqrt() };
        if rd8(this.wrapping_add(OFF_FLAGS)) & SIGN_BIT != 0 {
            z = mul(z, K_NEG_ONE);
        }
        wrf(out.wrapping_add(8), z);
        let n2 = add(add(mul(y, y), mul(x, x)), mul(z, z));
        let inv = if n2 == 0.0 { 0.0 } else { div(K_ONE, n2.sqrt()) };
        wrf(out, mul(x, inv));
        wrf(out.wrapping_add(4), mul(y, inv));
        wrf(out.wrapping_add(8), mul(z, inv));
        0
    }
});
