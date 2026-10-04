// original: 0x00a1d490 cam_vec_spread_pair (proposed)

/// Fans one source triple out into a below/above pair around an offset.
///
/// `this` points to a record holding a source triple (`+SRC_OFF`, three
/// words copied as bits) and an offset float at `+K_OFF`. Words 0/1/2 of
/// the source are copied to `+LO_OFF` and the offset is subtracted from
/// each (`word - K` in that operand order); the same three words are
/// copied to `+HI_OFF` and the offset is added to each, where the third
/// lane is evaluated as `K + word` (the original's operand order) while
/// the first two are `word + K`. Returns nothing.
///
/// Original: 0x00a1d490 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00a1d490(this: u32) -> u32 {
    unsafe {
        const SRC_OFF: u32 = 0x430;
        const K_OFF: u32 = 0x490;
        const LO_OFF: u32 = 0x60;
        const HI_OFF: u32 = 0x70;
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        let (s0, s1, s2) = (rd32(this + SRC_OFF), rd32(this + SRC_OFF + 4), rd32(this + SRC_OFF + 8));
        wr32(this + LO_OFF, s0);
        wr32(this + LO_OFF + 4, s1);
        wr32(this + LO_OFF + 8, s2);
        let k = rdf(this + K_OFF);
        wr32(this + LO_OFF, sub(rdf(this + LO_OFF), k).to_bits());
        wr32(this + LO_OFF + 4, sub(rdf(this + LO_OFF + 4), k).to_bits());
        wr32(this + LO_OFF + 8, sub(rdf(this + LO_OFF + 8), k).to_bits());
        wr32(this + HI_OFF, s0);
        wr32(this + HI_OFF + 4, s1);
        wr32(this + HI_OFF + 8, s2);
        let k2 = rdf(this + K_OFF);
        wr32(this + HI_OFF, add(rdf(this + HI_OFF), k2).to_bits());
        let t74 = add(rdf(this + HI_OFF + 4), k2);
        let t78 = add(k2, rdf(this + HI_OFF + 8));
        wr32(this + HI_OFF + 4, t74.to_bits());
        wr32(this + HI_OFF + 8, t78.to_bits());
        0
    }
});
