// original: 0x009272E0 queue_frame_shadow_cmds (proposed)

/// Queue the frame's shadow render commands for slot `idx`.
///
/// Issues four command calls for the frame slot selected by `idx`. The first
/// three go to the block-copy command handler (callee 1) with `(selector,
/// params)`, where the selector is a global dword and `params` is four words
/// built on the scratch area: the first call passes the constant header at
/// `HEADER` (`[2.0, 0, 0, 0]`), the second `[BLEND, 0, F1, NUM / DEN]` with
/// blend factor 0.2, slot float `F1` and the quotient of the constant numerator
/// by the slot divisor, the third `[0, 0, F2, F3]` with two more slot floats.
/// The scratch pointers differ between the original and this rewrite, so the
/// contract skips those arguments and snapshots the four pointed-to words
/// instead; this rewrite builds the same words in locals. The fourth call goes
/// to the int-argument handler (callee 2) with
/// `(selector, SLOT_BASE + idx * SLOT_STRIDE)`; that pointer reaches global
/// memory, so it compares directly. Returns the last call's answer.
///
/// Original: 0x009272E0 (cdecl, one stack word). Four direct calls.
lf_checker_rt::export!(cdecl, rw_009272E0(idx: u32) -> u32 {
    unsafe {
        const HEADER: u32 = 0x00E8_6380;
        const SEL1: u32 = 0x0119_D050;
        const SEL2: u32 = 0x0119_D044;
        const SEL3: u32 = 0x0119_D04C;
        const SEL4: u32 = 0x0119_D034;
        const SLOT_BASE: u32 = 0x0119_F180;
        const SLOT_STRIDE: u32 = 0x110;
        const FIELD_BASE: u32 = 0x0119_F10C;
        const NUM_ADDR: u32 = 0x00FE_8D94;
        const BLEND: u32 = 0x3E4C_CCCD;
        #[inline(always)]
        unsafe fn rd(p: u32) -> u32 {
            unsafe { (p as *const u32).read_unaligned() }
        }
        let h = lf_checker_rt::relocated(HEADER);
        let h0 = rd(h);
        let h1 = rd(h.wrapping_add(4));
        let h2 = rd(h.wrapping_add(8));
        let h3 = rd(h.wrapping_add(12));
        let buf1 = [h0, h1, h2, h3];
        lf_checker_rt::callee_cdecl!(
            1,
            u32,
            rd(lf_checker_rt::relocated(SEL1)),
            buf1.as_ptr() as u32
        );
        let fields = idx
            .wrapping_mul(SLOT_STRIDE)
            .wrapping_add(lf_checker_rt::relocated(FIELD_BASE));
        let f1 = rd(fields);
        let num = f32::from_bits(rd(lf_checker_rt::relocated(NUM_ADDR)));
        let den = f32::from_bits(rd(fields.wrapping_add(8)));
        let quot = core::hint::black_box(num) / core::hint::black_box(den);
        let buf2 = [BLEND, 0u32, f1, quot.to_bits()];
        lf_checker_rt::callee_cdecl!(
            1,
            u32,
            rd(lf_checker_rt::relocated(SEL2)),
            buf2.as_ptr() as u32
        );
        let buf3 = [0u32, 0u32, rd(fields.wrapping_add(4)), rd(fields.wrapping_add(8))];
        lf_checker_rt::callee_cdecl!(
            1,
            u32,
            rd(lf_checker_rt::relocated(SEL3)),
            buf3.as_ptr() as u32
        );
        let slot = idx
            .wrapping_mul(SLOT_STRIDE)
            .wrapping_add(lf_checker_rt::relocated(SLOT_BASE));
        lf_checker_rt::callee_cdecl!(2, u32, rd(lf_checker_rt::relocated(SEL4)), slot)
    }
});
