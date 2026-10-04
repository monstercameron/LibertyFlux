// original: 0x0094d2c0 segment_test_with_z_prefilter
/// Range-check a candidate against two reference depths, then test it.
///
/// Sorts the two reference z values (a NaN sorts nowhere: the swap only
/// happens on strict greater-than, exactly like the original's `jbe`), then
/// rejects the candidate when it falls outside the closed range or any side
/// is NaN (the original's `jb` is taken on unordered too), reproducing the
/// original's exit EAX residue in each case. Otherwise forwards both
/// reference points, the extra float and the candidate pointer to the worker
/// and returns its low byte as a boolean, preserving the answer residue.
export!(cdecl, rw_0094d2c0(a: *const u32, b: *const u32, c: *const u32, f: u32) -> u32 {
    unsafe {
        let zb = f32::from_bits(*b.add(2));
        let zc = f32::from_bits(*c.add(2));
        let (lo, hi) = if zb > zc { (zc, zb) } else { (zb, zc) };
        let za = f32::from_bits(*a.add(2));
        if za.is_nan() || lo.is_nan() || za < lo {
            return b as u32 & !0xFF;
        }
        if hi.is_nan() || za.is_nan() || hi < za {
            return b as u32 & !0xFF;
        }
        let answer = callee_cdecl!(1, u32, a as u32, *b, *b.add(1), *c, *c.add(1), f);
        if answer & 0xFF == 0 {
            answer & !0xFF
        } else {
            (answer & !0xFF) | 1
        }
    }
});
