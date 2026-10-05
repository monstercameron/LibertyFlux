// original: 0x00b4f310 route_point_in_range (proposed)

/// Test whether a target point is within range of a route's far point.
///
/// `this` carries a point count at `+0x00` and a timestamp at `+0x08`. When
/// fewer than 1000 ticks of game time (`CLOCK`) have passed the test fails
/// (0); with one point or none it passes (1). Otherwise the squared
/// distance from `target` (three floats) to point `count` (16-byte points
/// at `+0x10 * i`, formed as (dy*dy + dx*dx) + dz*dz in single precision)
/// is compared against `radius`: returns 1 when the radius is above it (a
/// NaN on either side fails the test).
///
/// Original: 0x00b4f310 (thiscall, two stack words; boolean in AL).
lf_checker_rt::export!(thiscall, rw_00b4f310(this: u32, target: u32, radius_bits: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x00;
        const STAMP: u32 = 0x08;
        const STRIDE: u32 = 0x10;
        const CLOCK: u32 = 0x011735b4;
        const COOLDOWN: u32 = 1000;
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
        let then = ((this + STAMP) as *const u32).read_unaligned();
        if now.wrapping_sub(then) < COOLDOWN {
            return 0;
        }
        let count = ((this + COUNT) as *const u32).read_unaligned();
        if count <= 1 {
            return 1;
        }
        let pt = this + count.wrapping_mul(STRIDE);
        let dx = sub(rdf(pt), rdf(target));
        let dy = sub(rdf(pt + 4), rdf(target + 4));
        let dz = sub(rdf(pt + 8), rdf(target + 8));
        let d2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
        u32::from(f32::from_bits(radius_bits) > d2)
    }
});
