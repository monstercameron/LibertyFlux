// original: 0x00D2B6C0 navmesh_sweep_probe (proposed)

/// Sweep a probe value upward in quarter steps, asking a tester about each.
///
/// `shape` points at four floats (x, y, z, w). `level` is a float argument
/// passed to two float helpers (their answers seed the probe record).
/// `target` is passed through to the tester untouched.
///
/// Behaviour: the helpers are called with `level` first; their answers,
/// the negated second answer, the words of `shape`, a 1.0 word and several
/// zero words are laid out as a flat probe record. When `z + 0.5` strictly
/// exceeds `z - 0.5` the loop runs: before each tester call the record word
/// shadowing z is overwritten with the current probe value, and the tester
/// is called with `target`, four pointers into the record (at record
/// offsets 0x30, 0x18, 0x20 and 0x1c), a zero word, two constant floats
/// (about 1.8 and 0.3), and three more zero words. A nonzero tester answer
/// ends the sweep and is the result; otherwise the probe value grows by
/// 0.25 and the sweep continues while `z + 0.5` still strictly exceeds it.
/// A NaN or huge z skips the loop and the result is 0.
///
/// The record bytes the original never writes read as the contract's stack
/// fill (zero); the rewrite zeroes the whole record first. The two float
/// constants and the 0.5/0.25 steps are values the original reads from its
/// read-only data. The upper bytes of the result are whatever the last
/// helper or tester call left in EAX, reproduced exactly.
///
/// Original: 0x00D2B6C0 (stdcall, three stack words).
lf_checker_rt::export!(stdcall, rw_00d2b6c0(shape: u32, level: u32, target: u32) -> u32 {
    unsafe {
        const HALF: f32 = 0.5;
        const QUARTER: f32 = 0.25;
        const K_A: f32 = f32::from_bits(0x3FE6_6666);
        const K_B: f32 = f32::from_bits(0x3E99_999A);
        const ONE_BITS: u32 = 0x3F80_0000;
        const SIGN: u32 = 0x8000_0000;
        const HELPER_A: u32 = 1;
        const HELPER_B: u32 = 2;
        const TESTER: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            (a as *const u32).read_unaligned()
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            f32::from_bits(rd32(a))
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        // Both helpers take `level` in XMM0 on the original side; the
        // stub transports the rewrite's stack word there. Answers come
        // back as their bit patterns.
        let h1: u32 = lf_checker_rt::callee_cdecl!(HELPER_A, u32, level);
        let h2: u32 = lf_checker_rt::callee_cdecl!(HELPER_B, u32, level);

        // Probe record, word offsets matching the original's frame so the
        // four record pointers have the same layout. Zeroed: the original
        // reads several words it never wrote (stack fill 0).
        let mut rec = [0u32; 28];
        rec[0x14 / 4] = h1;
        rec[0x18 / 4] = 0;
        rec[0x30 / 4] = h1;
        rec[0x34 / 4] = h2;
        rec[0x38 / 4] = 0;
        rec[0x40 / 4] = h2 ^ SIGN;
        rec[0x44 / 4] = h1;
        rec[0x48 / 4] = 0;
        rec[0x50 / 4] = 0;
        rec[0x54 / 4] = 0;
        rec[0x58 / 4] = ONE_BITS;
        let sx = rdf(shape);
        let sy = rdf(shape + 4);
        let sz = rdf(shape + 8);
        let sw = rd32(shape + 12);
        rec[0x60 / 4] = sx.to_bits();
        rec[0x64 / 4] = sy.to_bits();
        rec[0x68 / 4] = sz.to_bits();
        rec[0x6C / 4] = sw;

        let mut probe = sub(sz, HALF);
        let hi = add(sz, HALF);
        // The loop runs only while the top strictly exceeds the probe
        // (ordered comparison; NaN or equal bounds skip it).
        if !(hi > probe) {
            return h2 & 0xFFFF_FF00;
        }
        let base = rec.as_mut_ptr() as u32;
        loop {
            rec[0x68 / 4] = probe.to_bits();
            let r: u32 = lf_checker_rt::callee_cdecl!(
                TESTER,
                u32,
                target,
                base.wrapping_add(0x30),
                base.wrapping_add(0x18),
                base.wrapping_add(0x20),
                base.wrapping_add(0x1C),
                0u32,
                K_A.to_bits(),
                K_B.to_bits(),
                0u32,
                0u32,
                0u32
            );
            if r as u8 != 0 {
                return r;
            }
            probe = add(probe, QUARTER);
            rec[0x14 / 4] = probe.to_bits();
            let hi2 = add(rdf(shape + 8), HALF);
            if !(hi2 > probe) {
                return r;
            }
        }
    }
});
