// original: 0x00b3ff60 range_band_probe (proposed)

/// Test a point against two distance bands around the module's centre.
///
/// Reads three floats at `p`, subtracts the global centre triple and forms
/// the squared distance in the original's order, `(dy*dy + dx*dx) + dz*dz`.
/// Returns 0 at once when that exceeds `fb*fb`. Otherwise two band flags
/// are formed: `f1` when the distance lies between `fc*fc` and `fd*fd`,
/// `f2` when it lies between `fa*fa` and `fb*fb` (all comparisons ordered,
/// so a NaN anywhere selects neither band). If neither band holds, the
/// result is 0 without calling out.
///
/// Otherwise the point and `fe` are offered to a probe callee (thiscall on
/// `a1 + 0x10`, five stack words: x, y, z, `fe`, 0) whose nonzero answer
/// counts as a hit, and the result is `(f2 AND hit) OR (f1 AND NOT hit)`:
/// the outer band selects a hit, the inner band selects a miss.
///
/// Original: 0x00B3FF60 (stdcall, seven stack words, boolean in AL).
lf_checker_rt::export!(stdcall, rw_00b3ff60(p: u32, a1: u32, fa: u32, fb: u32, fc: u32, fd: u32, fe: u32) -> u32 {
    unsafe {
        const CENTER: u32 = 0x016C85A0;
        const PROBE: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn gf(off: u32) -> f32 {
            unsafe { f32::from_bits(lf_checker_rt::global::<u32>(CENTER + off).read_unaligned()) }
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

        let x = rdf(p);
        let y = rdf(p.wrapping_add(4));
        let z = rdf(p.wrapping_add(8));
        let dx = sub(x, gf(0));
        let dy = sub(y, gf(4));
        let dz = sub(z, gf(8));
        let d2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
        let b = f32::from_bits(fb);
        let b2 = mul(b, b);
        if d2 > b2 {
            return 0;
        }
        let c = f32::from_bits(fc);
        let d = f32::from_bits(fd);
        let c2 = mul(c, c);
        let dsq = mul(d, d);
        let f1 = d2 >= c2 && dsq >= d2;
        let a = f32::from_bits(fa);
        let a2 = mul(a, a);
        let f2 = d2 >= a2 && b2 >= d2;
        if !f1 && !f2 {
            return 0;
        }
        let hit: u32 = lf_checker_rt::callee_thiscall!(
            PROBE, u32, a1.wrapping_add(0x10),
            x.to_bits(), y.to_bits(), z.to_bits(), fe, 0
        );
        if (f2 && hit != 0) || (f1 && hit == 0) {
            1
        } else {
            0
        }
    }
});
