// original: 0x00b20530 scale_vec_add_scaled_normal (proposed)

/// Scales a vector and adds a second vector times the input's length.
///
/// Thiscall of one stack word: a pointer to three floats (callee pops 4
/// bytes). With s from +0x30 and coefficients from +0x20..+0x28 in the
/// object: each component is multiplied by s (x as `x*s`, y and z as
/// `s*y` and `s*z`), then `sqrt(x*x + y*y + z*z)` times the matching
/// coefficient is added (`len*c0`, `c1*len`, `c2*len`). The float
/// operation order is the original's. Returns the vector pointer.
lf_checker_rt::export!(thiscall, rw_00b20530(this: u32, vec: u32) -> u32 {
    unsafe {
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        let x = (vec as *const f32).read_unaligned();
        let y = ((vec + 4) as *const f32).read_unaligned();
        let z = ((vec + 8) as *const f32).read_unaligned();
        let s = ((this + 0x30) as *const f32).read_unaligned();
        let len2 = add(add(mul(x, x), mul(y, y)), mul(z, z));
        let len = len2.sqrt();
        let c0 = ((this + 0x20) as *const f32).read_unaligned();
        let c1 = ((this + 0x24) as *const f32).read_unaligned();
        let c2 = ((this + 0x28) as *const f32).read_unaligned();
        (vec as *mut f32).write_unaligned(add(mul(x, s), mul(len, c0)));
        ((vec + 4) as *mut f32).write_unaligned(add(mul(s, y), mul(c1, len)));
        ((vec + 8) as *mut f32).write_unaligned(add(mul(s, z), mul(c2, len)));
        vec
    }
});
