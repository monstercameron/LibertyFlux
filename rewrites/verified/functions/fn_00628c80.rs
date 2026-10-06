// original: 0x00628c80 transform_box_minmax (proposed)

/// Transform an axis-aligned box by a matrix, tracking the minimum and
/// maximum of all eight corners.
///
/// `mat` holds the matrix (three rows at `+0x00`/`+0x10`/`+0x20`-ish layout
/// with translations at `+0x30`/`+0x34`/`+0x38`; the `+0x20` column is
/// xored with the shared mask first, which negates it when the mask is the
/// sign bit). `box_a` and `box_b` each hold three input floats; the eight
/// corners mix them per axis (all of `box_a` first, then the seven with at
/// least one `box_b` lane). Every corner is transformed with the original's
/// exact operation order (the first corner adds the y-term before the x-term,
/// later corners the other way round) and folded into running minima and
/// maxima with `comiss`+`ja` semantics: a new value replaces the bound
/// unless it is strictly above (for minima) or strictly below (for maxima)
/// it, and an unordered (NaN) comparison always stores.
///
/// Results: `box_a` gets (min_x, min_y, min_z, fill) and `box_b` gets
/// (max_x, max_y, max_z, fill), where the second word reuses the register
/// holding the final y-minimum and `fill` is the contract's defined fill
/// for a stack slot the original never writes (uninitialized garbage in the
/// game; 0 under the checker). Finishes with the security-cookie check call,
/// whose argument (cookie xored with the stack pointer) differs between the
/// sides and is not compared, and no meaningful return value (a stale loop
/// pointer).
///
/// The original takes its third argument on the stack but returns with a
/// plain `ret` (caller cleanup), which no Rust calling convention expresses:
/// the rewrite is a three-argument fastcall that pops its stack word, so the
/// stack-pointer check is off and only the epilogue differs (precedent:
/// lane r-b200's accepted caller-cleanup rewrite).
///
/// Original: 0x00628c80 (fastcall: matrix, box_a, box_b on the stack).
lf_checker_rt::export!(fastcall, rw_00628c80(mat: u32, box_a: u32, box_b: u32) -> u32 {
    unsafe {
        const COOKIE_CHECK: u32 = 1;
        const COOKIE_GLOBAL: u32 = 0x01057fb4;
        const MASK_GLOBAL: u32 = 0x00fe8fa0;
        // Corner lane sources: 0 = box_a lane, 1 = box_b lane, per (x, y, z).
        const CORNERS: [(u8, u8, u8); 7] =
            [(1, 0, 0), (0, 1, 0), (0, 0, 1), (1, 1, 0), (0, 1, 1), (1, 0, 1), (1, 1, 1)];

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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let mask = rd32(lf_checker_rt::relocated(MASK_GLOBAL));
        let nm20 = f32::from_bits(rd32(mat.wrapping_add(0x20)) ^ mask);
        let nm24 = f32::from_bits(rd32(mat.wrapping_add(0x24)) ^ mask);
        let nm28 = f32::from_bits(rd32(mat.wrapping_add(0x28)) ^ mask);
        let a0 = rdf(box_a);
        let a1 = rdf(box_a.wrapping_add(4));
        let a2 = rdf(box_a.wrapping_add(8));
        let b0 = rdf(box_b);
        let b1 = rdf(box_b.wrapping_add(4));
        let b2 = rdf(box_b.wrapping_add(8));

        // First corner (all box_a lanes): y-term added before x-term.
        let mut minx = add(
            add(add(mul(nm20, a1), mul(rdf(mat), a0)), mul(rdf(mat.wrapping_add(0x10)), a2)),
            rdf(mat.wrapping_add(0x30)),
        );
        let mut miny = add(
            add(
                add(mul(nm24, a1), mul(rdf(mat.wrapping_add(4)), a0)),
                mul(rdf(mat.wrapping_add(0x14)), a2),
            ),
            rdf(mat.wrapping_add(0x34)),
        );
        let mut minz = add(
            add(
                add(mul(nm28, a1), mul(rdf(mat.wrapping_add(8)), a0)),
                mul(rdf(mat.wrapping_add(0x18)), a2),
            ),
            rdf(mat.wrapping_add(0x38)),
        );
        let (mut maxx, mut maxy, mut maxz) = (minx, miny, minz);
        for (sx, sy, sz) in CORNERS {
            let x = if sx == 0 { a0 } else { b0 };
            let y = if sy == 0 { a1 } else { b1 };
            let z = if sz == 0 { a2 } else { b2 };
            // Later corners: x-term added before y-term.
            let tx = add(
                add(add(mul(rdf(mat), x), mul(nm20, y)), mul(rdf(mat.wrapping_add(0x10)), z)),
                rdf(mat.wrapping_add(0x30)),
            );
            let ty = add(
                add(
                    add(mul(rdf(mat.wrapping_add(4)), x), mul(nm24, y)),
                    mul(rdf(mat.wrapping_add(0x14)), z),
                ),
                rdf(mat.wrapping_add(0x34)),
            );
            let tz = add(
                add(
                    add(mul(rdf(mat.wrapping_add(8)), x), mul(nm28, y)),
                    mul(rdf(mat.wrapping_add(0x18)), z),
                ),
                rdf(mat.wrapping_add(0x38)),
            );
            if !(tx > minx) {
                minx = tx;
            }
            if !(ty > miny) {
                miny = ty;
            }
            if !(tz > minz) {
                minz = tz;
            }
            if !(maxx > tx) {
                maxx = tx;
            }
            if !(maxy > ty) {
                maxy = ty;
            }
            if !(maxz > tz) {
                maxz = tz;
            }
        }
        wrf(box_a, minx);
        wrf(box_a.wrapping_add(4), miny);
        wrf(box_a.wrapping_add(8), minz);
        wrf(box_a.wrapping_add(0x0c), 0.0);
        wrf(box_b, maxx);
        wrf(box_b.wrapping_add(4), maxy);
        wrf(box_b.wrapping_add(8), maxz);
        wrf(box_b.wrapping_add(0x0c), 0.0);
        // Security-cookie check: the original passes cookie^esp in ecx, which
        // cannot match across sides, so only the call itself is compared.
        let cookie = (lf_checker_rt::global::<u32>(COOKIE_GLOBAL)).read();
        lf_checker_rt::callee_fastcall!(COOKIE_CHECK, u32, cookie, 0);
        0
    }
});
