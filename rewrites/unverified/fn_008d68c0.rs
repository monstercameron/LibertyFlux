// original: 0x008d68c0 heading_from_offset

/// Heading of a 2D offset vector through the scripted two-double helper.
///
/// Reads the source pointer at `this + 0x20`. A null pointer returns +0.0.
/// Otherwise loads `x` at source + 0x10, `y` at source + 0x14 and `z` at
/// source + 0x18, forms the length `r = sqrt(x*x + y*y)`, and negates it
/// when the limit at source + 0x28 is strictly below zero (an unordered
/// comparison leaves it positive). Then calls the scripted callee with the
/// doubles `z` and signed `r` (low lanes; the upper lanes of the second
/// register carry caller leftovers the callee never reads) and returns the
/// callee's double answer narrowed to float. 32-bit thiscall, no stack
/// arguments, float result.
lf_checker_rt::export!(thiscall, rw_008d68c0(this: u32) -> f32 {
    unsafe {
        const SRC_PTR: u32 = 0x20;
        const X_OFF: u32 = 0x10;
        const Y_OFF: u32 = 0x14;
        const Z_OFF: u32 = 0x18;
        const LIMIT_OFF: u32 = 0x28;
        const NEG_ONE_VA: u32 = 0x00FE8D94;
        const CALLEE: u32 = 1;

        let src = ((this + SRC_PTR) as *const u32).read_unaligned();
        if src == 0 {
            return 0.0;
        }
        let x = ((src + X_OFF) as *const f32).read_unaligned();
        let y = ((src + Y_OFF) as *const f32).read_unaligned();
        let z = ((src + Z_OFF) as *const f32).read_unaligned();
        let limit = ((src + LIMIT_OFF) as *const f32).read_unaligned();
        let xx = x * x;
        let yy = y * y;
        let sum = core::hint::black_box(xx) + core::hint::black_box(yy);
        let mut r = sum.sqrt();
        if limit < 0.0 {
            let neg_one = lf_checker_rt::global::<f32>(NEG_ONE_VA).read_unaligned();
            r = core::hint::black_box(r) * core::hint::black_box(neg_one);
        }
        let zd = z as f64;
        let rd = r as f64;
        let zb = zd.to_bits();
        let rb = rd.to_bits();
        let ans = lf_checker_rt::callee_cdecl!(
            CALLEE,
            u64,
            (zb & 0xFFFF_FFFF) as u32,
            (zb >> 32) as u32,
            (rb & 0xFFFF_FFFF) as u32,
            (rb >> 32) as u32
        );
        core::hint::black_box(f64::from_bits(ans)) as f32
    }
});
