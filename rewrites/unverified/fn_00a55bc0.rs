// original: 0x00a55bc0 vehicle_dist_check (proposed)

/// Whether the center is farther than the radius from the other point: 1/0.
///
/// Takes the squared distance between the center (+0x19a0, three floats) and
/// the other's point (+0x30) against the squared sum of the half-size
/// (+0x1a00) and the other's radius (+0x8). The sum of squares accumulates
/// y, then x, then z. Strictly greater means outside (NaN compares false).
/// Thiscall, one stack word, 1 or 0 in eax.
lf_checker_rt::export!(thiscall, rw_00a55bc0(this: u32, other: u32) -> u32 {
    unsafe {
        const C0: u32 = 0x19a0;
        const C1: u32 = 0x19a4;
        const C2: u32 = 0x19a8;
        const HALF: u32 = 0x1a00;
        const P0: u32 = 0x30;
        const P1: u32 = 0x34;
        const P2: u32 = 0x38;
        const RAD: u32 = 0x08;
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        let dx = sub(rdf(this.wrapping_add(C0)), rdf(other.wrapping_add(P0)));
        let dy = sub(rdf(this.wrapping_add(C1)), rdf(other.wrapping_add(P1)));
        let dz = sub(rdf(this.wrapping_add(C2)), rdf(other.wrapping_add(P2)));
        let r = add(rdf(this.wrapping_add(HALF)), rdf(other.wrapping_add(RAD)));
        let d2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
        let r2 = mul(r, r);
        if d2 > r2 {
            1
        } else {
            0
        }
    }
});
