// original: 0x00b50560 advance_route_index (proposed)

/// Advance a route cursor past its next point when close enough.
///
/// `this` holds a point count at `+0x00`, the cursor at `+0x04` and 16-byte
/// points at `+0x10 * i`. When the count exceeds 1, the squared distance from
/// `target` (three floats) to point `cursor + 1` is formed in single
/// precision as (dy*dy + dx*dx) + dz*dz; if `limit` is above it (a NaN
/// limit or distance never advances) and the cursor is not already at the
/// last point, the cursor is incremented. No meaningful return value.
///
/// Original: 0x00b50560 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00b50560(this: u32, target: u32, limit_bits: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x00;
        const CURSOR: u32 = 0x04;
        const STRIDE: u32 = 0x10;
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
        let count = ((this + COUNT) as *const u32).read_unaligned();
        if count <= 1 {
            return 0;
        }
        let cursor = ((this + CURSOR) as *const u32).read_unaligned();
        let pt = this + (cursor.wrapping_add(1)).wrapping_mul(STRIDE);
        let dx = sub(rdf(pt), rdf(target));
        let dy = sub(rdf(pt + 4), rdf(target + 4));
        let dz = sub(rdf(pt + 8), rdf(target + 8));
        let d2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
        if f32::from_bits(limit_bits) > d2 && cursor < count.wrapping_sub(1) {
            ((this + CURSOR) as *mut u32).write_unaligned(cursor.wrapping_add(1));
        }
        0
    }
});
