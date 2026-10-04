// original: 0x00d284d0 targeting_range_factor (proposed)

/// Compute a target's range factor from a query point, with clamping.
///
/// `this` carries the limit float at `+0x22c` and a mode float at `+0x230`;
/// `origin` selects a base point (the block at `arg3 + 0x14`, offset `+0x30`
/// past its link at `+0x20`, or `+0x10` past the block itself when the link is
/// null) and `point` is the query. Stores into `*out` the distance from the
/// query to the base, clamped into `[0, limit]` (NaN passes through: the
/// original's second compare jumps below-or-equal on unordered). Then forms
/// `q = 1.0 - clamped / limit`, adds a further 1.0 when the mode float equals
/// exactly 1.0, and returns `0.2 * q` in ST0 — unless the low three bits of
/// the byte at `arg3 + 0x28` equal 1, in which case it returns `q` itself
/// (the original reuses its fourth argument slot as scratch and returns it
/// unscaled). The third argument is never read. All floating-point operation
/// order matches the original.
///
/// Original: 0x00D284D0 (thiscall, four stack arguments).
lf_checker_rt::export!(thiscall, rw_00d284d0(this: u32, out: u32, point: u32, _spare: u32, origin: u32) -> f32 {
    unsafe {
        const LIMIT_OFF: u32 = 0x22c;
        const MODE_OFF: u32 = 0x230;
        const BLOCK_OFF: u32 = 0x14;
        const LINK_OFF: u32 = 0x20;
        const LINKED_PT_OFF: u32 = 0x30;
        const DIRECT_PT_OFF: u32 = 0x10;
        const KIND_OFF: u32 = 0x28;
        const KIND_MASK: u8 = 7;
        const KIND_PASSTHROUGH: u8 = 1;
        const ONE_BITS: u32 = 0x3f80_0000; // 1.0f
        const GAIN_BITS: u32 = 0x3e4c_cccd; // 0.2f
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
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        let block = unsafe { ((origin + BLOCK_OFF) as *const u32).read_unaligned() };
        let link = unsafe { ((block + LINK_OFF) as *const u32).read_unaligned() };
        let base = if link != 0 { link + LINKED_PT_OFF } else { block + DIRECT_PT_OFF };
        let dx = sub(rdf(point), rdf(base));
        let dy = sub(rdf(point + 4), rdf(base + 4));
        let dz = sub(rdf(point + 8), rdf(base + 8));
        let dist2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
        let dist = dist2.sqrt();
        let limit = rdf(this + LIMIT_OFF);
        // Clamp into [0, limit]; NaN passes through (see doc comment).
        let clamped = if 0.0 > dist {
            0.0
        } else if !(dist > limit) {
            dist
        } else {
            limit
        };
        unsafe { (out as *mut u32).write_unaligned(clamped.to_bits()) };
        let q = sub(
            f32::from_bits(ONE_BITS),
            core::hint::black_box(clamped) / core::hint::black_box(limit),
        );
        let mut q = q;
        if rdf(this + MODE_OFF) == f32::from_bits(ONE_BITS) {
            q = add(q, f32::from_bits(ONE_BITS));
        }
        let kind = unsafe { ((origin + KIND_OFF) as *const u8).read() } & KIND_MASK;
        if kind == KIND_PASSTHROUGH {
            q
        } else {
            mul(f32::from_bits(GAIN_BITS), q)
        }
    }
});
