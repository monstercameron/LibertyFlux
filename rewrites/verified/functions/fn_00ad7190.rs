// original: 0x00AD7190 audio_grid_emit (proposed)

/// Emit one audio grid cell selected by a row and column.
///
/// `row` and `col` pick a word from the grid table `GRID` at index
/// `col + 12 * row`; its top two bits choose the path. Top 0 returns at
/// once. Top 1 (cell path) resolves four sample records through the cell's
/// flag block `CELL` (flag word first, four indices below it), converts
/// each record's two coordinates to floats, stages sixteen words and, when
/// the device object (`DEVICE`) has its ready bit and the enable byte is
/// set, submits them through the device hook (id 1, thiscall); a nonzero
/// answer skips the rest. Otherwise the cell is marked, counted through
/// `COUNT_A`, and queued with the producer (id 2); when the device's dump
/// bit is set the four records are also forwarded as twelve floats plus a
/// device word (id 3). Top 2 (flag path) tests one bit of the flag table
/// `FLAGS`, marks it, counts through `COUNT_B` and queues likewise. Top 3
/// walks the chain table `CHAIN` from the cell's low 14 bits while words
/// keep their top bits set: subtype 1 runs the cell shape, subtype 2 the
/// flag shape, anything else advances; a skipped hook answer or a clear
/// dump bit continues the walk instead of leaving. Every path ends at the
/// frame check (id 4, no arguments, registers preserved). One frame word
/// the original never writes reads as zero (see the contract's stack
/// fill); all float steps are exact integer conversions and moves.
///
/// Original: 0x00AD7190 (cdecl, two stack words; no meaningful return).
lf_checker_rt::export!(cdecl, rw_00ad7190(row: u32, col: u32) -> u32 {
    unsafe {
        const GRID: u32 = 0x0154_E358;
        const CHAIN: u32 = 0x0154_E478;
        const FLAGS: u32 = 0x0154_E30E;
        const CELL: u32 = 0x0155_0EBC;
        const CELL_IDX: u32 = 0x0155_0EB4;
        const SAMP: u32 = 0x0158_E860;
        const ENABLE_DW: u32 = 0x0103_F494;
        const DEVICE: u32 = 0x012F_B1B8;
        const COUNT_A: u32 = 0x0155_0DE8;
        const COUNT_B: u32 = 0x0155_0DEC;
        const READY_BIT: u8 = 8;
        const DUMP_BIT: u32 = 0x400;
        const DEV_READY_OFF: u32 = 0x19;
        const DEV_DUMP_OFF: u32 = 0x8e8;
        const DEV_WORD_OFF: u32 = 0x8fc;
        const DEV_HOOK_OFF: u32 = 0x50;
        const PROD_A_CNT: u32 = 0xe60;
        const PROD_B_CNT: u32 = 0xe64;
        const PROD_B_BASE: u32 = 0x7d0;
        const HOOK: u32 = 1;
        const PRODUCER: u32 = 2;
        const DUMP: u32 = 3;
        const FRAMECHK: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u32) {
            unsafe { (a as *mut u16).write_unaligned(v as u16) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        // One sample record: two sign-extended coordinates as floats plus
        // the stored float bits.
        #[inline(always)]
        unsafe fn sample(rec: u32) -> (u32, u32, u32) {
            unsafe {
                let x = rd16(rec) as u16 as i16 as i32 as f32;
                let y = rd16(rec + 2) as u16 as i16 as i32 as f32;
                (x.to_bits(), y.to_bits(), rd32(rec + 4))
            }
        }

        let xb = lf_checker_rt::xbase();
        let g = |va: u32| va.wrapping_sub(0x400000).wrapping_add(xb);
        let idx = col.wrapping_add(row.wrapping_mul(3).wrapping_mul(4));
        let w = rd16(g(GRID).wrapping_add(idx.wrapping_mul(2)));
        let dev = rd32(g(DEVICE));
        let top = w >> 14;

        // Sixteen staged words for the device hook: per record the two
        // converted coordinates, the stored float, and the blank word.
        let mut stage = [0u32; 16];
        let cell_base = |pay: u32| g(CELL).wrapping_add((pay & 0x3fff).wrapping_shl(4));
        unsafe fn fill_stage(stage: &mut [u32; 16], rec_base: u32, samp: u32) {
            unsafe {
                for k in 0..4 {
                    let ix =
                        (rd16(rec_base.wrapping_add((k as u32).wrapping_mul(2))) as u16 as i16
                            as i32)
                            .wrapping_mul(8) as u32;
                    let (x, y, f) = sample(samp.wrapping_add(ix));
                    stage[k * 4] = x;
                    stage[k * 4 + 1] = y;
                    stage[k * 4 + 2] = f;
                    stage[k * 4 + 3] = 0;
                }
            }
        }

        if top == 0 {
            lf_checker_rt::callee_cdecl!(FRAMECHK, u32,);
            return 0;
        }
        if top == 2 {
            // Flag path.
            let fa = g(FLAGS).wrapping_add((w & 0x3fff).wrapping_mul(8));
            let f = rd16(fa);
            if f & 3 != 0 {
                lf_checker_rt::callee_cdecl!(FRAMECHK, u32,);
                return 0;
            }
            wr16(fa, f | 1);
            let ca = rd32(g(COUNT_B));
            wr32(ca, rd32(ca).wrapping_add(1));
            let p = lf_checker_rt::callee_cdecl!(PRODUCER, u32,);
            let n = rd32(p.wrapping_add(PROD_B_CNT));
            wr16(
                p.wrapping_add(PROD_B_BASE).wrapping_add(n.wrapping_mul(2)),
                w & 0x3fff,
            );
            wr32(p.wrapping_add(PROD_B_CNT), n.wrapping_add(1));
            lf_checker_rt::callee_cdecl!(FRAMECHK, u32,);
            return 0;
        }
        if top == 1 {
            // Cell path.
            let base = cell_base(w);
            let f = rd16(base);
            if f & 3 != 0 {
                lf_checker_rt::callee_cdecl!(FRAMECHK, u32,);
                return 0;
            }
            if rd8(dev.wrapping_add(DEV_READY_OFF)) & READY_BIT != 0 {
                fill_stage(&mut stage, base.wrapping_sub(8), g(SAMP));
                if (rd32(g(ENABLE_DW)) >> 8) & 0xff != 0 {
                    let sp = (&stage[0] as *const u32) as u32;
                    let ans = lf_checker_rt::callee_thiscall!(
                        HOOK,
                        u32,
                        dev.wrapping_add(DEV_HOOK_OFF),
                        sp,
                        4
                    );
                    if ans != 0 {
                        lf_checker_rt::callee_cdecl!(FRAMECHK, u32,);
                        return 0;
                    }
                }
            }
            wr16(base, f | 1);
            let ca = rd32(g(COUNT_A));
            wr32(ca, rd32(ca).wrapping_add(1));
            let p = lf_checker_rt::callee_cdecl!(PRODUCER, u32,);
            let n = rd32(p.wrapping_add(PROD_A_CNT));
            wr16(p.wrapping_add(n.wrapping_mul(2)), w & 0x3fff);
            wr32(p.wrapping_add(PROD_A_CNT), n.wrapping_add(1));
            if rd32(dev.wrapping_add(DEV_DUMP_OFF)) & DUMP_BIT == 0 {
                lf_checker_rt::callee_cdecl!(FRAMECHK, u32,);
                return 0;
            }
            let mut outs = [0u32; 12];
            for k in 0..4 {
                let ix = (rd16(base.wrapping_sub(8).wrapping_add((k as u32).wrapping_mul(2)))
                    as u16 as i16 as i32)
                    .wrapping_mul(8) as u32;
                let (x, y, fl) = sample(g(SAMP).wrapping_add(ix));
                outs[k * 3] = x;
                outs[k * 3 + 1] = y;
                outs[k * 3 + 2] = fl;
            }
            let dw = rd32(dev.wrapping_add(DEV_WORD_OFF));
            lf_checker_rt::callee_cdecl!(DUMP, u32, outs[0], outs[1], outs[2], outs[3], outs[4],
                outs[5], outs[6], outs[7], outs[8], outs[9], outs[10], outs[11], dw);
            lf_checker_rt::callee_cdecl!(FRAMECHK, u32,);
            return 0;
        }
        // Chain walk.
        let mut li = w & 0x3fff;
        loop {
            let lw = rd16(g(CHAIN).wrapping_add(li.wrapping_mul(2)));
            if lw & 0xc000 == 0 {
                break;
            }
            let t = lw >> 14;
            if t == 2 {
                let fa = g(FLAGS).wrapping_add((lw & 0x3fff).wrapping_mul(8));
                let f = rd16(fa);
                if f & 3 == 0 {
                    wr16(fa, f | 1);
                    let ca = rd32(g(COUNT_B));
                    wr32(ca, rd32(ca).wrapping_add(1));
                    let p = lf_checker_rt::callee_cdecl!(PRODUCER, u32,);
                    let n = rd32(p.wrapping_add(PROD_B_CNT));
                    wr16(
                        p.wrapping_add(PROD_B_BASE).wrapping_add(n.wrapping_mul(2)),
                        lw & 0x3fff,
                    );
                    wr32(p.wrapping_add(PROD_B_CNT), n.wrapping_add(1));
                }
            } else if t == 1 {
                let base = cell_base(lw);
                let f = rd16(base);
                if f & 3 == 0 {
                    let mut run = true;
                    if rd8(dev.wrapping_add(DEV_READY_OFF)) & READY_BIT != 0 {
                        fill_stage(&mut stage, base.wrapping_sub(8), g(SAMP));
                        if (rd32(g(ENABLE_DW)) >> 8) & 0xff != 0 {
                            let sp = (&stage[0] as *const u32) as u32;
                            let ans = lf_checker_rt::callee_thiscall!(
                                HOOK,
                                u32,
                                dev.wrapping_add(DEV_HOOK_OFF),
                                sp,
                                4
                            );
                            if ans != 0 {
                                run = false;
                            }
                        }
                    }
                    if run {
                        wr16(base, f | 1);
                        let ca = rd32(g(COUNT_A));
                        wr32(ca, rd32(ca).wrapping_add(1));
                        let p = lf_checker_rt::callee_cdecl!(PRODUCER, u32,);
                        let n = rd32(p.wrapping_add(PROD_A_CNT));
                        wr16(p.wrapping_add(n.wrapping_mul(2)), lw & 0x3fff);
                        wr32(p.wrapping_add(PROD_A_CNT), n.wrapping_add(1));
                        if rd32(dev.wrapping_add(DEV_DUMP_OFF)) & DUMP_BIT != 0 {
                            let mut outs = [0u32; 12];
                            for k in 0..4 {
                                let ix = (rd16(base.wrapping_sub(8).wrapping_add(
                                    (k as u32).wrapping_mul(2),
                                )) as u16 as i16
                                    as i32)
                                    .wrapping_mul(8) as u32;
                                let (x, y, fl) = sample(g(SAMP).wrapping_add(ix));
                                outs[k * 3] = x;
                                outs[k * 3 + 1] = y;
                                outs[k * 3 + 2] = fl;
                            }
                            let dw = rd32(dev.wrapping_add(DEV_WORD_OFF));
                            lf_checker_rt::callee_cdecl!(DUMP, u32, outs[0], outs[1], outs[2],
                                outs[3], outs[4], outs[5], outs[6], outs[7], outs[8], outs[9],
                                outs[10], outs[11], dw);
                        }
                    }
                }
            }
            li = li.wrapping_add(1);
            let nw = rd16(g(CHAIN).wrapping_add(li.wrapping_mul(2)));
            if nw & 0xc000 == 0 {
                break;
            }
        }
        lf_checker_rt::callee_cdecl!(FRAMECHK, u32,);
        0
    }
});
