// original: 0x00d78ae0 angle_blend_two_samples (proposed)

/// Blend the nearer of two heading samples into a stored angle.
///
/// `obj` points to an object holding a per-index triple of handles (dwords at
/// `obj + index*4 + ENTRY_BASE .. +8`) and a center point (two floats at
/// `CENTER_X`/`CENTER_Y` past the struct at `obj + CENTER_PTR`). `out` points
/// to the stored angle (one float).
///
/// Behaviour: callee 1 resolves the handle triple into two sample points
/// (two floats each, written through its last two pointer arguments); each
/// sample minus the center goes through the heading callee (callees 2 and 3,
/// one per sample), giving headings `h0` and `h1`. Both deltas `h - old` are
/// wrapped into [-PI, PI] by repeated +/- TAU steps. If both wrapped deltas
/// are strictly negative the nearer (greater) is kept, if both are strictly
/// positive the nearer (smaller) is kept, otherwise the stored angle is left
/// alone; the kept delta is added to the old angle and stored. Finally the
/// stored angle is wrapped into [0, TAU] the same way. NaN deltas or a NaN
/// stored angle take no branch that stores (every comparison is unordered).
///
/// Original: 0x00d78ae0 (cdecl, three stack words; no return value).
lf_checker_rt::export!(cdecl, rw_00d78ae0(obj: u32, index: u32, out: u32) -> u32 {
    unsafe {
        const ENTRY_BASE: u32 = 0x0dd0;
        const CENTER_PTR: u32 = 0x20;
        const CENTER_X: u32 = 0x30;
        const CENTER_Y: u32 = 0x34;
        const PI: f32 = f32::from_bits(0x4049_0fdb);
        const TAU: f32 = f32::from_bits(0x40c9_0fdb);
        const NEG_PI: f32 = f32::from_bits(0xc049_0fdb);
        const RESOLVE: u32 = 1;
        const HEADING_FIRST: u32 = 2;
        const HEADING_SECOND: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
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
        fn wrap_pi(mut d: f32) -> f32 {
            while d > PI {
                d = sub(d, TAU);
            }
            while NEG_PI > d {
                d = add(d, TAU);
            }
            d
        }

        // Resolve the handle triple into two sample points.
        let entry = obj.wrapping_add(index.wrapping_mul(4));
        let h0 = rd32(entry.wrapping_add(ENTRY_BASE));
        let h1 = rd32(entry.wrapping_add(ENTRY_BASE + 4));
        let h2 = rd32(entry.wrapping_add(ENTRY_BASE + 8));
        let mut first = [0.0f32; 2];
        let mut second = [0.0f32; 2];
        lf_checker_rt::callee_cdecl!(
            RESOLVE, u32, h0, h1, h2,
            first.as_mut_ptr() as u32, second.as_mut_ptr() as u32
        );

        // Headings of (sample - center) for both samples.
        let center = rd32(obj.wrapping_add(CENTER_PTR));
        let cx = rdf(center.wrapping_add(CENTER_X));
        let cy = rdf(center.wrapping_add(CENTER_Y));
        let head0: f32 = lf_checker_rt::callee_cdecl!(
            HEADING_FIRST, f32,
            sub(first[0], cx).to_bits(), sub(first[1], cy).to_bits()
        );
        let head1: f32 = lf_checker_rt::callee_cdecl!(
            HEADING_SECOND, f32,
            sub(second[0], cx).to_bits(), sub(second[1], cy).to_bits()
        );

        // Keep the nearer delta when both agree in sign, else store nothing.
        let old = rdf(out);
        let d0 = wrap_pi(sub(head0, old));
        let d1 = wrap_pi(sub(head1, old));
        if d0 < 0.0 && d1 < 0.0 {
            wrf(out, add(if d0 > d1 { d0 } else { d1 }, old));
        } else if d0 > 0.0 && d1 > 0.0 {
            wrf(out, add(if d1 > d0 { d0 } else { d1 }, old));
        }

        // Wrap the stored angle into [0, TAU].
        let mut v = rdf(out);
        if 0.0 > v {
            while 0.0 > v {
                v = add(v, TAU);
            }
            wrf(out, v);
            v = rdf(out);
        }
        if v > TAU {
            while v > TAU {
                v = sub(v, TAU);
            }
            wrf(out, v);
        }
        0
    }
});
