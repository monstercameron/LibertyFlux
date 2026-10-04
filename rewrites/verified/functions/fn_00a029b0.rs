// original: 0x00A029B0 frag_ray_pair_cast_b (proposed)

/// Cast a paired ray query twice through space A and dispatch on motion.
///
/// Twin of the neighbouring cast function, differing only in that both
/// handle lookups run on global space A (`a0` then `a1`). Otherwise
/// identical: frame blocks for direction (`a3..a5`), motion (`a6..a8`) and
/// 16 constant bytes, the two transform callees when any motion word is
/// nonzero (NaN counts as nonzero), then the dispatch callee with
/// (`second`, `a2`, 0x102, direction block, constant block) on the first
/// handle. Scratch words are a zeroed stand-in, uncompared. Returns the
/// dispatch answer.
///
/// Original: 0x00A029B0 (cdecl, nine stack words).
lf_checker_rt::export!(cdecl, rw_00A029B0(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32) -> u32 {
    unsafe {
        const SPACE_A: u32 = 0x01632C60;
        const CONST16: u32 = 0x00FE8F00;
        const DISPATCH_TAG: u32 = 0x102;
        let s1 = lf_checker_rt::callee_thiscall!(1, u32,
            (lf_checker_rt::relocated(SPACE_A) as *const u32).read_unaligned(), a0);
        let s2 = lf_checker_rt::callee_thiscall!(6, u32,
            (lf_checker_rt::relocated(SPACE_A) as *const u32).read_unaligned(), a1);
        let motion = [a6, a7, a8];
        let dir = [a3, a4, a5];
        let base = lf_checker_rt::relocated(CONST16) as *const u32;
        let c16 = [base.read_unaligned(), base.add(1).read_unaligned(),
                   base.add(2).read_unaligned(), base.add(3).read_unaligned()];
        let scratch = [0u32; 4];
        if f32::from_bits(a6) != 0.0 || f32::from_bits(a7) != 0.0 || f32::from_bits(a8) != 0.0 {
            lf_checker_rt::callee_thiscall!(3, u32, scratch.as_ptr() as u32, motion.as_ptr() as u32);
            lf_checker_rt::callee_thiscall!(4, u32, c16.as_ptr() as u32, scratch.as_ptr() as u32);
        }
        lf_checker_rt::callee_thiscall!(5, u32, s1, s2, a2, DISPATCH_TAG,
            dir.as_ptr() as u32, c16.as_ptr() as u32)
    }
});
