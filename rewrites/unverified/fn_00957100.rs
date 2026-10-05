// original: 0x00957100 task_seq_player (proposed) -- STAGE 2 of a staged rewrite.
// Stage 2 covers entry, dispatch, halt (opcode 5) and five handlers
// (opcodes 0, 2, 3, 4, 6). All other opcodes are unimplemented and disclosed
// in results.json; see the lane report for the per-handler plan.

/// Play one task-sequence buffer: read the opcode byte at base+cursor,
/// dispatch through the opcode table, run the handler (which advances the
/// base past its entry), and repeat until the halt opcode.
///
/// `ctx` points to the player state: the buffer base at +0, the cursor at
/// +4. `arg1` is a float word handed to one handler; `arg2` (low byte) is
/// a flag the halt path reads. Each handler advances the base by its own
/// entry stride (0x50, 4, 4, 0xc for the four covered here) and jumps back
/// to dispatch. The halt path advances the base by 4, and when the flag is
/// set bumps a counter global and runs a reset worker, then returns with
/// the low byte cleared: the prior word is the reset worker's answer when
/// the flag is set, otherwise the context pointer itself.
///
/// Stage-1 coverage: opcodes 2 (one thiscall taking the entry pointer and
/// the float word), 3 (one 4-word call of three entry bytes plus -1), 4
/// (one 1-word call of the first entry byte, then a float multiply into a
/// global), 6 (a float multiply of the first entry byte into a global, no
/// calls) and 5 (halt). Unhandled opcodes abort: they never occur under the
/// stage-1 contract, which pins every reachable byte to a covered opcode.
///
/// Original: 0x00957100 (cdecl, three stack words; the second is read as a
/// float, the third only as its low byte).
lf_checker_rt::export!(cdecl, rw_00957100(ctx: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const C_ENTER: u32 = 1;
        const C_OP2: u32 = 2;
        const C_OP3: u32 = 3;
        const C_OP4: u32 = 4;
        const C_RESET: u32 = 5;
        const C_OP0_LOOKUP: u32 = 6;
        const C_OP0_DRAIN: u32 = 7;

        const G_OP4_OUT: u32 = 0x12ddeb4;
        const G_OP6_OUT: u32 = 0x11f7058;
        const G_HALT_COUNT: u32 = 0x11f70c4;
        const C_DISPATCH_XMM: u32 = 0xfe86e8;
        const G_OP0_KIND_TAB: u32 = 0x11f6ff0;
        const G_OP0_DIVISOR: u32 = 0x11f6ffb;
        const G_OP0_NEXT_TAB: u32 = 0x11f6f7c;
        const G_OP0_INDEX: u32 = 0x118eff0;
        const G_OP0_ENTRY_TAB: u32 = 0x118e7f8;

        #[inline(always)]
        unsafe fn m_rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn m_rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn m_wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn m_wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn g_rd8(a: u32) -> u8 {
            unsafe { (lf_checker_rt::global::<u8>(a) as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn g_rd32(a: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(a) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn g_wr32(a: u32, v: u32) {
            unsafe { (lf_checker_rt::global::<u32>(a) as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn g_rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(g_rd32(a)) }
        }
        #[inline(always)]
        unsafe fn g_wrf(a: u32, v: f32) {
            unsafe { g_wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        let _: u32 = lf_checker_rt::callee_cdecl!(C_ENTER, u32,);
        let xmm1 = g_rdf(C_DISPATCH_XMM);
        loop {
            let base = m_rd32(ctx);
            let cursor = m_rd32(ctx.wrapping_add(4));
            let entry = base.wrapping_add(cursor);
            let op = m_rd8(entry);
            if op == 5 {
                break;
            }
            match op {
                0 => {
                    let b = m_rd8(ctx.wrapping_add(8)) as u32;
                    if g_rd8(G_OP0_KIND_TAB.wrapping_add(b)) == 2 {
                        let i = g_rd32(G_OP0_INDEX);
                        let e = g_rd32(G_OP0_ENTRY_TAB.wrapping_add(i.wrapping_mul(4)));
                        let a: u32 = lf_checker_rt::callee_thiscall!(
                            C_OP0_LOOKUP, u32, e
                        );
                        if (a & 0xff) != 0 {
                            let d: u32 =
                                lf_checker_rt::callee_cdecl!(C_OP0_DRAIN, u32,);
                            return (d & 0xffff_ff00) | 1;
                        }
                        return (a & 0xffff_ff00) | 1;
                    }
                    let div = g_rd8(G_OP0_DIVISOR) as u32;
                    let rem = (b.wrapping_add(1)) % div;
                    m_wr32(ctx, 0);
                    m_wr8(ctx.wrapping_add(8), rem as u8);
                    m_wr32(
                        ctx.wrapping_add(4),
                        g_rd32(G_OP0_NEXT_TAB.wrapping_add(rem.wrapping_mul(4))),
                    );
                }
                2 => {
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        C_OP2, u32, entry, arg1
                    );
                    m_wr32(ctx, base.wrapping_add(0x50));
                }
                3 => {
                    let b1 = m_rd8(entry.wrapping_add(1)) as u32;
                    let b2 = m_rd8(entry.wrapping_add(2)) as u32;
                    let b3 = m_rd8(entry.wrapping_add(3)) as u32;
                    let _: u32 = lf_checker_rt::callee_cdecl!(
                        C_OP3, u32, b1, b2, b3, 0xffff_ffff
                    );
                    m_wr32(ctx, base.wrapping_add(4));
                }
                4 => {
                    let b1 = m_rd8(entry.wrapping_add(1)) as u32;
                    let _: u32 = lf_checker_rt::callee_cdecl!(C_OP4, u32, b1);
                    let b2 = m_rd8(entry.wrapping_add(2)) as u32;
                    g_wrf(G_OP4_OUT, mul(b2 as f32, xmm1));
                    m_wr32(ctx, base.wrapping_add(4));
                }
                6 => {
                    let b1 = m_rd8(entry.wrapping_add(1)) as u32;
                    g_wrf(G_OP6_OUT, mul(b1 as f32, xmm1));
                    m_wr32(ctx, base.wrapping_add(0x0c));
                }
                _ => unreachable!("stage 1: opcode not covered"),
            }
        }
        m_wr32(ctx, m_rd32(ctx).wrapping_add(4));
        let mut r = ctx;
        if (arg2 & 0xff) != 0 {
            g_wr32(G_HALT_COUNT, g_rd32(G_HALT_COUNT).wrapping_add(1));
            r = lf_checker_rt::callee_cdecl!(C_RESET, u32,);
        }
        r & 0xffff_ff00
    }
});
