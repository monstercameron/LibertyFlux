// original: 0x00cb7f20 range_check_radius_plus_5
/// Whether a point is inside a radius grown by five units (leaf).
///
/// Takes the squared distance between the point at `[this + 0x20 .. +0x28]`
/// (thiscall) and the anchor at `[[a0 + 0x20] + 0x30 .. +0x38]`, and the
/// squared grown radius `([this + 0x50] + 5.0)^2`, where `5.0` is the
/// constant from file address `0x00FE8AD8`. Returns 1 when the grown
/// radius strictly exceeds the distance, else 0 (an `(an instruction of the original)` clears
/// the whole register first, so the result is exactly 0 or 1). The float
/// operation order is the original's. No calls, no writes.
lf_checker_rt::export!(thiscall, rw_00cb7f20(this: u32, a0: u32) -> u32 {
    unsafe {
        /// Point offsets in this object; radius offset.
        const PX: u32 = 0x20;
        const PY: u32 = 0x24;
        const PZ: u32 = 0x28;
        const RADIUS: u32 = 0x50;
        /// Anchor-pointer offset in the argument object, then coordinates.
        const ANCHOR_OFF: u32 = 0x20;
        const AX: u32 = 0x30;
        const AY: u32 = 0x34;
        const AZ: u32 = 0x38;
        /// Grown-radius padding (constant from file address 0x00FE8AD8).
        const PADDING: f32 = 5.0;
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        let anchor = ((a0 + ANCHOR_OFF) as *const u32).read_unaligned();
        let dx = sub(rdf(this + PX), rdf(anchor + AX));
        let dy = sub(rdf(this + PY), rdf(anchor + AY));
        let dz = sub(rdf(this + PZ), rdf(anchor + AZ));
        let grown = add(rdf(this + RADIUS), PADDING);
        let syy = mul(dy, dy);
        let sxx = mul(dx, dx);
        let szz = mul(dz, dz);
        let dist2 = add(add(syy, sxx), szz);
        let rad2 = mul(grown, grown);
        u32::from(rad2 > dist2)
    }
});
