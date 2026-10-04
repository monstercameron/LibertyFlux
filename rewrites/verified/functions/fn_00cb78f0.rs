// original: 0x00cb78f0 proximity_flags_update
/// Accumulate planar-proximity bits and test the vertical gap (leaf).
///
/// Compares the point at `a1` against the anchor reached through `a0`
/// (`[a0 + 0x20]`, thiscall with three stack arguments). When the point's
/// x is strictly greater than the anchor's x bit 1 is set in the flag word
/// at `a2`, when it is strictly smaller bit 2 is set (equal or unordered
/// sets nothing); the y pair sets bits 4 and 8 the same way. Then, when
/// the tolerance at `[this + 0xA4]` strictly exceeds the absolute z gap,
/// the function returns the `a2` address with its low byte forced to 1 if
/// the flag word equals exactly `0xF`, else with its low byte cleared;
/// when the tolerance does not exceed the gap it returns the cleared form.
/// The absolute value clears the sign bit, preserving NaN payloads. The
/// float operation order is the original's. No calls.
lf_checker_rt::export!(thiscall, rw_00cb78f0(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        /// Anchor pointer inside the first argument's object.
        const ANCHOR_OFF: u32 = 0x20;
        /// Anchor coordinate offsets.
        const AX: u32 = 0x30;
        const AY: u32 = 0x34;
        const AZ: u32 = 0x38;
        /// Tolerance offset in this object.
        const TOL_OFF: u32 = 0xA4;
        /// Bits for x-greater, x-smaller, y-greater, y-smaller.
        const X_GT: u32 = 1;
        const X_LT: u32 = 2;
        const Y_GT: u32 = 4;
        const Y_LT: u32 = 8;
        /// Flag value meaning both axes bracketed on both sides.
        const ALL_BITS: u32 = 0xF;
        const SIGN: u32 = 0x8000_0000;
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd(a)) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        let anchor = rd(a0 + ANCHOR_OFF);
        let mut flags = rd(a2);
        let px = rdf(a1);
        let qx = rdf(anchor + AX);
        if px > qx {
            flags |= X_GT;
        } else if qx > px {
            flags |= X_LT;
        }
        let py = rdf(a1 + 4);
        let qy = rdf(anchor + AY);
        if py > qy {
            flags |= Y_GT;
        } else if qy > py {
            flags |= Y_LT;
        }
        (a2 as *mut u32).write_unaligned(flags);
        let dz = sub(rdf(anchor + AZ), rdf(a1 + 8));
        let gap = f32::from_bits(dz.to_bits() & !SIGN);
        let tol = rdf(this + TOL_OFF);
        let done = rd(a2);
        if tol > gap && done == ALL_BITS {
            (a2 & 0xFFFFFF00) | 1
        } else {
            a2 & 0xFFFFFF00
        }
    }
});
