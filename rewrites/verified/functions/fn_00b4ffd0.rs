// original: 0x00b4ffd0 path_length_below_limit (proposed)

/// Test whether a route's remaining length is below 4 units.
///
/// `this` carries a context pointer at `+0x20`, a point count at `+0x140`,
/// a cursor at `+0x144` and a flag at `+0x14C`. The test fails (0) unless
/// at least 2500 ticks of game time (`CLOCK` minus `STAMP`) have passed, the
/// count exceeds 1 and the flag is clear. Otherwise the length from the
/// context point to point `cursor + 21` plus every 16-byte step from there
/// to point `count - 1` is summed in single precision (each leg as
/// sqrt((dy*dy + dx*dx) + dz*dz), the first leg identically), and 1 is
/// returned when the 4.0 limit (`LIMIT`) is above the total.
///
/// Original: 0x00b4ffd0 (thiscall, no stack words; boolean in AL).
lf_checker_rt::export!(thiscall, rw_00b4ffd0(this: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x140;
        const CURSOR: u32 = 0x144;
        const FLAG: u32 = 0x14c;
        const CTX: u32 = 0x20;
        const STRIDE: u32 = 0x10;
        const FIRST_EXTRA: u32 = 0x15;
        const STEPS_BASE: u32 = 0x154;
        const CLOCK: u32 = 0x011735b4;
        const STAMP: u32 = 0x0166838c;
        const LIMIT: u32 = 0x00fe8ab8;
        const COOLDOWN: u32 = 2500;
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
        let now = lf_checker_rt::global::<u32>(CLOCK).read_unaligned();
        let then = lf_checker_rt::global::<u32>(STAMP).read_unaligned();
        if now.wrapping_sub(then) < COOLDOWN {
            return 0;
        }
        let count = ((this + COUNT) as *const u32).read_unaligned();
        if count <= 1 {
            return 0;
        }
        if ((this + FLAG) as *const u32).read_unaligned() != 0 {
            return 0;
        }
        let ctx = ((this + CTX) as *const u32).read_unaligned();
        let cursor = ((this + CURSOR) as *const u32).read_unaligned();
        let first = this + (cursor.wrapping_add(FIRST_EXTRA)).wrapping_mul(STRIDE);
        let dx = sub(rdf(first), rdf(ctx + 0x30));
        let dy = sub(rdf(first + 4), rdf(ctx + 0x34));
        let dz = sub(rdf(first + 8), rdf(ctx + 0x38));
        let mut total = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz)).sqrt();
        if cursor < count.wrapping_sub(1) {
            let mut p = this + STEPS_BASE + cursor.wrapping_mul(STRIDE);
            let mut left = count.wrapping_sub(1).wrapping_sub(cursor);
            while left != 0 {
                // Leg from the point ending at p to the one starting at
                // p + 0x10: (p+0xC)-(p-4), (p+0x10)-p, (p+0x14)-(p+4).
                let ax = sub(rdf(p + 0x0c), rdf(p.wrapping_sub(4)));
                let ay = sub(rdf(p + 0x10), rdf(p));
                let az = sub(rdf(p + 0x14), rdf(p + 4));
                let leg = add(add(mul(ax, ax), mul(ay, ay)), mul(az, az)).sqrt();
                total = add(total, leg);
                p = p.wrapping_add(STRIDE);
                left = left.wrapping_sub(1);
            }
        }
        let limit = (lf_checker_rt::global::<u32>(LIMIT) as *const u32).read_unaligned();
        u32::from(f32::from_bits(limit) > total)
    }
});
