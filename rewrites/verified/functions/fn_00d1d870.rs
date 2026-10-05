// original: 0x00d1d870 cover_heading_test (proposed)
/// Test whether a ped's heading geometry passes a rotated half-plane check.
///
/// `this` is the task, `ped` the ped. A null tag pointer at `ped+0xd68`
/// fails at once. Otherwise a direction vector is built two ways: when the
/// tag's low 3 bits equal 1, callee 1 (thiscall on the tag, an out-pointer
/// plus 0) writes three floats, the position at `ped+0x20` plus `0x30` is
/// subtracted, and the difference is scaled by 1/length (a zero length maps
/// to a zero scale through a parity trick on the comparison flags, so NaN
/// lengths take the divide path); any other tag value reads the vector from
/// the three floats at callee 2's answer instead.
///
/// Callees 3 and 4 (cdecl, no stack args; each takes pi/2 as an f64 in XMM0,
/// which the checker cannot observe, and answers an f64 in XMM0) supply two
/// scalars narrowed to f32; the vector's x/y are rotated by them
/// (x*s2 - y*s1, y*s2 + x*s1). Callees 5 and 6 (cdecl, the float at
/// `this+0x30` in XMM0, f32 answers) supply two more scalars that fold the
/// rotated vector into one float through multiply-by-zero and add chains in
/// the original's exact operand order. A positive result passes (1),
/// otherwise, NaN included, it fails (0). Only the low result byte is
/// behaviour. Original: 0x00d1d870 (thiscall, one stack arg).
lf_checker_rt::export!(thiscall, rw_00d1d870(this: u32, ped: u32) -> u32 {
    unsafe {
        const PED_TAG: u32 = 0xd68;
        const PED_POS: u32 = 0x20;
        const ALT_SCALAR: u32 = 0x30;
        const NORM_ONE: u32 = 0x00fe88e8;
        const SAMPLE: u32 = 1;
        const READOUT: u32 = 2;
        const FIRST_F64: u32 = 3;
        const SECOND_F64: u32 = 4;
        const FIRST_F32: u32 = 5;
        const SECOND_F32: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        let tag = rd32(ped + PED_TAG);
        if tag == 0 {
            return 0;
        }
        let mut out = [0u32; 3];
        let (vx, vy, vz) = if (rd8(tag) & 7) == 1 {
            lf_checker_rt::callee_thiscall!(SAMPLE, u32, tag, out.as_mut_ptr() as u32, 0);
            let base = rd32(ped + PED_POS);
            let x = sub(f32::from_bits(out[0]), rdf(base + 0x30));
            let y = sub(f32::from_bits(out[1]), rdf(base + 0x34));
            let z = sub(f32::from_bits(out[2]), rdf(base + 0x38));
            let len2 = add(add(mul(x, x), mul(y, y)), mul(z, z));
            let scale = if len2 == 0.0 {
                0.0
            } else {
                let one = f32::from_bits(rd32(lf_checker_rt::relocated(NORM_ONE)));
                div(one, core::hint::black_box(len2).sqrt())
            };
            (mul(x, scale), mul(y, scale), mul(z, scale))
        } else {
            let mut scratch = [0u32; 3];
            let p: u32 = lf_checker_rt::callee_thiscall!(READOUT, u32, tag, scratch.as_mut_ptr() as u32, 0);
            (rdf(p), rdf(p + 4), rdf(p + 8))
        };
        let r3: u64 = lf_checker_rt::callee_cdecl!(FIRST_F64, u64,);
        let s1 = core::hint::black_box(f64::from_bits(r3)) as f32;
        let r4: u64 = lf_checker_rt::callee_cdecl!(SECOND_F64, u64,);
        let s2 = core::hint::black_box(f64::from_bits(r4)) as f32;
        let t1 = mul(vy, s1);
        let t2 = mul(vx, s1);
        let t3 = mul(vx, s2);
        let t4 = mul(vy, s2);
        let nx = sub(t3, t1);
        let ny = add(t4, t2);
        let seed = rd32(this + ALT_SCALAR);
        let r5 = f32::from_bits(lf_checker_rt::callee_cdecl!(FIRST_F32, u32, seed));
        let r6 = f32::from_bits(lf_checker_rt::callee_cdecl!(SECOND_F32, u32, seed));
        let t5 = mul(r6, 0.0);
        let t6 = sub(t5, r5);
        let t7 = mul(r5, 0.0);
        let t8 = mul(t6, nx);
        let t9 = add(t7, r6);
        let t10 = mul(vz, 0.0);
        let t11 = mul(t9, ny);
        let t12 = add(t11, t8);
        let res = add(t12, t10);
        if res > 0.0 {
            1
        } else {
            0
        }
    }
});
