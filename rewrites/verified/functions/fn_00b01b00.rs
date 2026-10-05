// original: 0x00B01B00 refresh_changed_pairs (proposed)

/// Fetches four floats, optionally re-derives two pairs, and reports them
/// when anything changed.
///
/// `this` is the owning object; the fetch helper is called with `this+0x10`
/// and returns a pointer to at least 16 bytes holding the four base values.
/// When the flag byte at `this+0x488` is set, the pair helper is called with
/// `this+0x46c` and a four-word buffer holding the first pair plus the two
/// reference words, and its answer replaces all four. When the flag byte at
/// `this+0x4a8` is set, the pair helper is called with `this+0x48c` and a
/// buffer holding the second pair, and the new second pair minus the first
/// pair replaces the second pair (float subtraction in the original's order).
/// Finally the four current values are compared against the four reference
/// words with the original's unordered-aware equality (a NaN on either side
/// counts as different); if any pair differs, the sink helper is called with
/// `this+0x10` and the six words (pair one, pair two, 0, 1.0). The sink's
/// callee cleans the six stack words; this function itself takes no stack
/// arguments and returns nothing (thiscall, plain `ret`). It writes nothing
/// outside its own frame; all cross-call state is the four words plus the
/// two reference words the fetch helper's block keeps.
lf_checker_rt::export!(thiscall, rw_00B01B00(this: u32) -> u32 {
    unsafe {
        const FETCH_OBJ: u32 = 0x10;
        const PAIR1_OBJ: u32 = 0x46c;
        const PAIR2_OBJ: u32 = 0x48c;
        const FLAG1_OFF: u32 = 0x488;
        const FLAG2_OFF: u32 = 0x4a8;
        const ONE_BITS: u32 = 0x3f800000;

        #[inline(always)]
        unsafe fn rd32(p: u32) -> u32 {
            unsafe { (p as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(p: u32) -> u8 {
            unsafe { (p as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(p: u32) -> f32 {
            unsafe { f32::from_bits(rd32(p)) }
        }
        #[inline(always)]
        fn sub(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) - core::hint::black_box(y)
        }

        // Fetch block: the original copies six words but only reads four.
        let dp = lf_checker_rt::callee_thiscall!(1, u32, this.wrapping_add(FETCH_OBJ));
        let f0 = rdf(dp);
        let f1 = rdf(dp.wrapping_add(4));
        let f2 = rdf(dp.wrapping_add(8));
        let f3 = rdf(dp.wrapping_add(12));
        let _w4 = rd32(dp.wrapping_add(16));
        let _w5 = rd32(dp.wrapping_add(20));
        let (mut p0, mut p1) = (f0, f1);
        // Second pair lanes: `a` starts as f2 (frame slot +0x10) and `b` as
        // f3 (frame slot +0x08); the helper buffer below follows that order.
        let (mut a, mut b) = (f2, f3);
        let (mut r0, mut r1) = (f0, f1);
        if rd8(this.wrapping_add(FLAG1_OFF)) != 0 {
            let mut buf = [p0.to_bits(), p1.to_bits(), r0.to_bits(), r1.to_bits()];
            lf_checker_rt::callee_thiscall!(
                2,
                u32,
                this.wrapping_add(PAIR1_OBJ),
                buf.as_mut_ptr() as u32
            );
            p0 = f32::from_bits(buf[0]);
            p1 = f32::from_bits(buf[1]);
            r0 = f32::from_bits(buf[2]);
            r1 = f32::from_bits(buf[3]);
        }
        if rd8(this.wrapping_add(FLAG2_OFF)) != 0 {
            // The buffer's second and fourth words are uninitialized stack in
            // the original; the stub overwrites the whole buffer on both
            // sides, so the initial value only matters for the pre-call
            // snapshot, which the contract pins to the stack fill (zero).
            let mut buf = [b.to_bits(), 0, a.to_bits(), 0];
            lf_checker_rt::callee_thiscall!(
                2,
                u32,
                this.wrapping_add(PAIR2_OBJ),
                buf.as_mut_ptr() as u32
            );
            let v0 = f32::from_bits(buf[0]);
            let v1 = f32::from_bits(buf[1]);
            a = sub(v0, p0);
            b = sub(v1, p1);
        }
        if r0 != p0 || r1 != p1 || a != f2 || b != f3 {
            lf_checker_rt::callee_thiscall!(
                3,
                u32,
                this.wrapping_add(FETCH_OBJ),
                p0.to_bits(),
                p1.to_bits(),
                a.to_bits(),
                b.to_bits(),
                0,
                ONE_BITS
            );
        }
        0
    }
});
