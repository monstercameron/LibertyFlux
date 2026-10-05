// original: 0x00C9DBB0 build_vector_query_and_match (proposed)

/// Build a fixed query block from two 3-float vectors and three global floats,
/// pass it to the matcher callee, and report whether the callee returned zero.
///
/// `p1` and `p2` point to three floats each; `a3` is an opaque word passed
/// through to the callee. The block is 30 words: the `p1` triple at words
/// 0-2, the `p2` triple at words 4-6, a zero word at 8, the globals
/// `G1`, `G2`, `G0` repeated at words 12-14, 16-18 and 20-22, zeros at
/// 24-26, `0xffff` at 27 and zero words at 28-29; every other word is left
/// as uninitialized stack (zero under the proof's stack fill). The callee
/// is invoked with eight words: the block base, the block
/// base plus `0x20`, 0, `a3`, -1, 7, 1, 0. The result is 1 when the callee
/// returns 0 and 0 otherwise.
///
/// The original additionally loads the global at `0x12b9c78` into a register
/// it never reads again; that dead load has no observable effect and is not
/// reproduced.
///
/// Original: 0x00C9DBB0 (stdcall, three stack words).
lf_checker_rt::export!(stdcall, rw_00C9DBB0(p1: u32, p2: u32, a3: u32) -> u32 {
    unsafe {
        const G0: u32 = 0x1B4B328;
        const G1: u32 = 0x1B4B320;
        const G2: u32 = 0x1B4B324;
        const CALLEE: u32 = 1;

        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let g0 = rd(lf_checker_rt::relocated(G0));
        let g1 = rd(lf_checker_rt::relocated(G1));
        let g2 = rd(lf_checker_rt::relocated(G2));
        // The whole block, gaps zeroed (the proof fills uninitialized stack
        // with zero, so the original's unwritten words read the same).
        let mut q = [0u32; 30];
        q[0] = rd(p1);
        q[1] = rd(p1 + 4);
        q[2] = rd(p1 + 8);
        q[4] = rd(p2);
        q[5] = rd(p2 + 4);
        q[6] = rd(p2 + 8);
        q[12] = g1;
        q[13] = g2;
        q[14] = g0;
        q[16] = g1;
        q[17] = g2;
        q[18] = g0;
        q[20] = g1;
        q[21] = g2;
        q[22] = g0;
        q[27] = 0xFFFF;
        let base = q.as_mut_ptr() as u32;
        let r: u32 = lf_checker_rt::callee_cdecl!(
            CALLEE,
            u32,
            base,
            base.wrapping_add(0x20),
            0,
            a3,
            0xFFFF_FFFF,
            7,
            1,
            0
        );
        if r == 0 { 1 } else { 0 }
    }
});
