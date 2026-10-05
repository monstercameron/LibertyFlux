// original: 0x00b1ddc0 frustum_point_outside (proposed)

/// Reports whether a point is outside any of the object's planes.
///
/// Thiscall with three stack words: a pointer to the point (three
/// floats), a bias float and a flag byte (callee pops 12 bytes). Tests
/// the first 6 planes, or the first 4 when the flag is nonzero; plane i
/// is four floats at +0x10*i with the normal's y at +0, x at +4, z at +8
/// and the distance at +12. For each plane it forms
/// `dist - (py*nx + px*ny + pz*nz) + bias` in that operand order; when
/// the value is ordered-less-than zero the point is outside and the
/// function returns 1 at once, else 0 after all planes. A NaN value
/// never counts as outside. Only AL is meaningful on return.
lf_checker_rt::export!(thiscall, rw_00b1ddc0(this: u32, point: u32, bias: f32, flag: u32) -> u32 {
    unsafe {
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        const PLANE_STRIDE: u32 = 0x10;
        let px = (point as *const f32).read_unaligned();
        let py = ((point + 4) as *const f32).read_unaligned();
        let pz = ((point + 8) as *const f32).read_unaligned();
        let count = if (flag & 0xff) != 0 { 4u32 } else { 6u32 };
        let mut i = 0u32;
        while i < count {
            let base = this + i * PLANE_STRIDE;
            let ny = (base as *const f32).read_unaligned();
            let nx = ((base + 4) as *const f32).read_unaligned();
            let nz = ((base + 8) as *const f32).read_unaligned();
            let dist = ((base + 12) as *const f32).read_unaligned();
            let dot = add(add(mul(py, nx), mul(px, ny)), mul(pz, nz));
            let v = add(sub(dist, dot), bias);
            if 0.0f32 > v {
                return 1;
            }
            i += 1;
        }
        0
    }
});
