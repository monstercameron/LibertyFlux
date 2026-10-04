// original: 0x00967c20 timing_slot_configure (proposed)

/// Copy two 16-byte descriptors into this timing slot, record two integer
/// parameters, then run a two-stage configure sequence driven by scripted
/// callees.
///
/// `this` points to a slot of at least 0x54 bytes. `desc_a`/`desc_b` point
/// to 16 bytes each, copied to `+0x00` and `+0x10`. `param` goes to `+0x44`,
/// `mode` to `+0x48`. The flag word at `+0x50` is first forced to have bit 0
/// set and bit 1 clear, then bit 1 is set on every path that reaches the
/// second stage.
///
/// When the low byte of `flags` is zero, a probe callee (id 1, cdecl, seven
/// arguments: both descriptors, 1, 5, `param`, `mode`, 0) runs first; its
/// answer is stored at `+0x40` and a non-zero answer returns immediately.
/// Otherwise the main callee (id 2, cdecl, seven arguments: both
/// descriptors, `param`, a 21-word frame struct, 6, 1, 4) fills in the
/// second stage. The frame struct holds a zero word, three copies of a
/// global float triple, zero padding, and a `{0, 0, 0, 0xffff, 0}` trailer;
/// words `+0x10..+0x2c` of it are copied to `+0x20..+0x3c` when the main
/// callee answers non-zero (setting flag bit 2), while a zero answer clears
/// bit 2. Returns the last callee answer.
///
/// The two stack slots the original never initialises read as zero under the
/// checker's defined stack fill; the rewrite zero-initialises the struct.
///
/// Original: thiscall, five stack words, callee cleans them.
lf_checker_rt::export!(thiscall, rw_00967c20(this: u32, desc_a: u32, desc_b: u32, param: u32, mode: u32, flags: u32) -> u32 {
    unsafe {
        const PROBE_CALLEE: u32 = 1;
        const MAIN_CALLEE: u32 = 2;
        const TRIPLE_BASE: u32 = 0x01b4_b320;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        for w in 0..4u32 {
            wr32(this.wrapping_add(w * 4), rd32(desc_a.wrapping_add(w * 4)));
            wr32(this.wrapping_add(0x10).wrapping_add(w * 4), rd32(desc_b.wrapping_add(w * 4)));
        }
        wr32(this.wrapping_add(0x48), mode);
        let mut slot_flags = (rd32(this.wrapping_add(0x50)) & !2) | 1;
        wr32(this.wrapping_add(0x44), param);
        wr32(this.wrapping_add(0x50), slot_flags);
        if (flags & 0xff) == 0 {
            let probe: u32 = lf_checker_rt::callee_cdecl!(
                PROBE_CALLEE, u32, desc_a, desc_b, 1, 5, param, mode, 0
            );
            wr32(this.wrapping_add(0x40), probe);
            if probe != 0 {
                return probe;
            }
        }
        slot_flags |= 2;
        wr32(this.wrapping_add(0x50), slot_flags);

        let g0 = rd32(lf_checker_rt::relocated(TRIPLE_BASE));
        let g1 = rd32(lf_checker_rt::relocated(TRIPLE_BASE.wrapping_add(4)));
        let g2 = rd32(lf_checker_rt::relocated(TRIPLE_BASE.wrapping_add(8)));
        let mut frame = [0u32; 21];
        frame[4] = g0;
        frame[5] = g1;
        frame[6] = g2;
        frame[8] = g0;
        frame[9] = g1;
        frame[10] = g2;
        frame[12] = g0;
        frame[13] = g1;
        frame[14] = g2;
        frame[19] = 0xffff;
        let answer: u32 = lf_checker_rt::callee_cdecl!(
            MAIN_CALLEE, u32, desc_a, desc_b, param, frame.as_mut_ptr() as u32, 6, 1, 4
        );
        if answer == 0 {
            wr32(this.wrapping_add(0x50), slot_flags & !4);
            return 0;
        }
        for w in 0..8u32 {
            wr32(this.wrapping_add(0x20).wrapping_add(w * 4), frame[4 + w as usize]);
        }
        wr32(this.wrapping_add(0x50), slot_flags | 4);
        answer
    }
});
