// original: 0x00b316b0 nearer_point_wins (proposed)

/// Order two records by id, breaking ties by distance to a reference point.
///
/// `a` and `b` point to records whose first word is a signed id and whose
/// words at `+0x10`, `+0x14`, `+0x18` are a point. Returns the id of `a` with
/// its low byte set to 1 when `a`'s id is greater, or when the ids are equal,
/// `strict` is non-zero, and `b`'s point is strictly farther (squared
/// distance) from the shared reference point than `a`'s. A pass returns `a`'s
/// id with its low byte set to 1, a fail `a`'s id with its low byte cleared.
/// The additions and the final comparison follow the original's order exactly.
///
/// Original: 0x00b316b0 (cdecl, three stack words; only the low byte of
/// `strict` is read; the reference point is three shared floats).
lf_checker_rt::export!(cdecl, rw_00b316b0(a: u32, b: u32, strict: u32) -> u32 {
    unsafe {
        const ID: u32 = 0x00;
        const PX: u32 = 0x10;
        const PY: u32 = 0x14;
        const PZ: u32 = 0x18;
        const REF_X: u32 = 0x0128e340;
        const REF_Y: u32 = 0x0128e344;
        const REF_Z: u32 = 0x0128e348;
        #[inline(always)]
        fn sub(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) - core::hint::black_box(y)
        }
        #[inline(always)]
        fn mul(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(y)
        }
        #[inline(always)]
        fn add(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) + core::hint::black_box(y)
        }
        #[inline(always)]
        unsafe fn rd(a: u32, off: u32) -> u32 {
            unsafe { (a as *const u32).byte_add(off as usize).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32, off: u32) -> f32 {
            unsafe { f32::from_bits(rd(a, off)) }
        }
        let va = rd(a, ID);
        let vb = rd(b, ID);
        if va as i32 > vb as i32 {
            return va & !0xFF | 1;
        }
        if strict & 0xFF == 0 || va != vb {
            return va & !0xFF;
        }
        let gx = f32::from_bits(lf_checker_rt::global::<u32>(REF_X).read());
        let gy = f32::from_bits(lf_checker_rt::global::<u32>(REF_Y).read());
        let gz = f32::from_bits(lf_checker_rt::global::<u32>(REF_Z).read());
        let ax = sub(rdf(a, PX), gx);
        let by = sub(rdf(b, PY), gy);
        let ay = sub(rdf(a, PY), gy);
        let bx = sub(rdf(b, PX), gx);
        let mut da = add(mul(ax, ax), mul(ay, ay));
        let az = sub(rdf(a, PZ), gz);
        let mut db = add(mul(by, by), mul(bx, bx));
        let bz = sub(rdf(b, PZ), gz);
        da = add(da, mul(az, az));
        db = add(db, mul(bz, bz));
        if db > da { va & !0xFF | 1 } else { va & !0xFF }
    }
});
