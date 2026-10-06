// original: 0x0096C140 blend_threadrow_4rows

/// Per-thread table lookup, global matrix setup, and four-row blend
/// accumulation into the object (thiscall: `this` in ECX, no stack words).
///
/// The thread-local slot index comes from a global (pristine 0); the slot
/// value's dword at `+0x70` is a row index selecting one of three 64-byte
/// rows read IN PLACE at `TABLE + idx*64` (the table address holds row
/// data, not a pointer: its first word is row 0's first float). Nine
/// floats are read from the row (`+0`, `+4`, `+8`, `+0x10`, `+0x14`,
/// `+0x18`, `+0x20`, `+0x24`, `+0x28`). A flag global gets
/// bit 0 set (read-modify-write, skipped when already set), twelve constant
/// words and four copies of an uninitialised stack slot are written to the
/// neighbouring globals, and a four-by-three constant matrix (plus/minus
/// 0.7071 and zeros) is laid out on the stack. The uninitialised slot reads
/// the stack fill (zero under this proof's contract); its eight stores are
/// never read back, but the four global ones are replicated exactly.
///
/// Each of the four rows then runs four inner taps over the global matrix
/// rows `[1,0,0]`, `[0,-1,0]`, `[-1,0,0]`, `[0,1,0]`: two dot products seeded
/// from the table row, whose squared length picks per-lane between the
/// select global and zero at three identical epsilon thresholds; the
/// normalised pair is blended bitwise with those selections and dotted with
/// the stack-matrix row, clamped at zero from below (NaN passes through).
/// The clamped dot drives `t = ((dot*|ta|)*dot)+tb` with `|ta|` the absolute
/// head combination and `tb = (K1-|ta|)*KQ`, and each tap accumulates three
/// channels from object floats (`A1` at `this+0x293c`, `A2` at `this+0x1718`,
/// `A3` at `this+0x17c0`, each stepping 4 per tap). The `A1` channel is never
/// stored to the object, but its final value is the first argument of the
/// row's inner call (thiscall/2, `this` = `this+0x294c+row*0x1c`, second
/// argument a global, answer in ST0), which is stored to the row output; the
/// `A2` and `A3` channels are stored beside it. Three post calls (thiscall/1
/// each, immediate `this`, answers ignored) consume the outputs. The return
/// value is the last post-call's answer (scripted zero).
///
/// Omitted with no observable effect: the dead stack stores (constant matrix
/// fourth words, per-tap zero slots, uninitialised-slot copies; the stack
/// check is off and every live scratch value is observed downstream), the
/// first channel's accumulation (recomputed but never read back after the
/// loop; still computed here for fault fidelity), and the security-cookie
/// prologue/epilogue (the check runs natively on the original side only, per
/// the r-b09 precedent; the slot is never written).
export!(thiscall, rw_0096C140(this: u32) -> u32 {
    unsafe {
        const TLS_IDX_G: u32 = 0x17aba14;
        const TABLE: u32 = 0x115e3f0;
        const FLAG_G: u32 = 0x121f670;
        const MAT_G: u32 = 0x121f630;
        const XM7_G: u32 = 0x17ad148;
        const TH0: u32 = 0x110dad0;
        const TH1: u32 = 0x110dad4;
        const TH2: u32 = 0x110dad8;
        const ABS_G: u32 = 0xfe8f80;
        const BLEND_G: u32 = 0x110db50;
        const K1_G: u32 = 0xfe88e8;
        const KQ_G: u32 = 0xfe87e4;
        const PUSHC_G: u32 = 0x11618fc;
        const POST_THIS: u32 = 0x115def0;
        const C_INNER: u32 = 1;
        const C_POST1: u32 = 2;
        const C_POST2: u32 = 3;
        const C_POST3: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn fmul(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(y)
        }
        #[inline(always)]
        fn fadd(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) + core::hint::black_box(y)
        }
        #[inline(always)]
        fn fsub(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) - core::hint::black_box(y)
        }
        #[inline(always)]
        fn fdiv(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) / core::hint::black_box(y)
        }
        #[inline(always)]
        fn fsqrt(x: f32) -> f32 {
            core::hint::black_box(x).sqrt()
        }

        // Thread-local table row.
        let slot = tls_slot(rd32(relocated(TLS_IDX_G)) as usize);
        let idx = rd32(slot.wrapping_add(0x70));
        let row = relocated(TABLE).wrapping_add(idx.wrapping_shl(6));
        let r0 = rdf(row);
        let r1 = rdf(row.wrapping_add(4));
        let r2 = rdf(row.wrapping_add(8));
        let r3 = rdf(row.wrapping_add(0x10));
        let r4 = rdf(row.wrapping_add(0x14));
        let r5 = rdf(row.wrapping_add(0x18));
        let r6 = rdf(row.wrapping_add(0x20));
        let r7 = rdf(row.wrapping_add(0x24));
        let r8 = rdf(row.wrapping_add(0x28));

        // Flag bit and global matrix (plus four dead zero stores that the
        // globals comparison still observes).
        let flag = rd32(relocated(FLAG_G));
        if flag & 1 == 0 {
            wr32(relocated(FLAG_G), flag | 1);
        }
        let mg = relocated(MAT_G);
        // Dead zero stores first (the original's order: uninitialised slot).
        wr32(mg.wrapping_add(0xc), 0);
        wr32(mg.wrapping_add(0x1c), 0);
        wr32(mg.wrapping_add(0x2c), 0);
        wr32(mg.wrapping_add(0x3c), 0);
        wr32(mg, 0x3f800000);
        wr32(mg.wrapping_add(4), 0);
        wr32(mg.wrapping_add(8), 0);
        wr32(mg.wrapping_add(0x10), 0);
        wr32(mg.wrapping_add(0x14), 0xbf800000);
        wr32(mg.wrapping_add(0x18), 0);
        wr32(mg.wrapping_add(0x20), 0xbf800000);
        wr32(mg.wrapping_add(0x24), 0);
        wr32(mg.wrapping_add(0x28), 0);
        wr32(mg.wrapping_add(0x30), 0);
        wr32(mg.wrapping_add(0x34), 0x3f800000);
        wr32(mg.wrapping_add(0x38), 0);
        // Read the rows back the way the taps do (same values just written).
        let m0 = [
            rdf(mg.wrapping_add(0)),
            rdf(mg.wrapping_add(4)),
            rdf(mg.wrapping_add(8)),
        ];
        let m1 = [
            rdf(mg.wrapping_add(0x10)),
            rdf(mg.wrapping_add(0x14)),
            rdf(mg.wrapping_add(0x18)),
        ];
        let m2 = [
            rdf(mg.wrapping_add(0x20)),
            rdf(mg.wrapping_add(0x24)),
            rdf(mg.wrapping_add(0x28)),
        ];
        let m3 = [
            rdf(mg.wrapping_add(0x30)),
            rdf(mg.wrapping_add(0x34)),
            rdf(mg.wrapping_add(0x38)),
        ];
        let mats = [m0, m1, m2, m3];

        let xm7 = rdf(relocated(XM7_G));
        let th0 = rdf(relocated(TH0));
        let th1 = rdf(relocated(TH1));
        let th2 = rdf(relocated(TH2));
        let k1 = rdf(relocated(K1_G));
        let kq = rdf(relocated(KQ_G));
        let bg = relocated(BLEND_G);
        let blend = [rd32(bg), rd32(bg.wrapping_add(4)), rd32(bg.wrapping_add(8))];
        let pushc = rd32(relocated(PUSHC_G));

        // Head combinations: the taps use |ta|, and tb is built on K1.
        let ta = fadd(fmul(fadd(r6, r7), 0.0), r8).abs();
        let tb = fmul(fsub(k1, ta), kq);
        // Stack-matrix rows (the 0.7071 constants, exact instruction bits).
        const QP: f32 = f32::from_bits(0x3F350481);
        const QM: f32 = f32::from_bits(0xBF350481);
        let srows = [
            [QP, QP, 0.0],
            [QP, QM, 0.0],
            [QM, QP, 0.0],
            [QM, QM, 0.0],
        ];

        let mut ecxarg = this.wrapping_add(0x294c);
        let mut outp = this.wrapping_add(0x29ec);
        let mut last = 0u32;
        for oi in 0..4usize {
            let mut acc0 = 0.0f32;
            let mut acc1 = 0.0f32;
            let mut acc2 = 0.0f32;
            let mut eax = this.wrapping_add(0x1718);
            for m in mats.iter() {
                // Fresh dots from the table row against this matrix row.
                let x5 = fadd(fadd(fmul(r1, m[1]), fmul(r0, m[0])), fmul(r2, m[2]));
                let x6 = fadd(fadd(fmul(r4, m[1]), fmul(r3, m[0])), fmul(r5, m[2]));
                let d = fadd(fmul(x6, x6), fmul(x5, x5));
                let s0 = if d > th0 { xm7 } else { 0.0 };
                let s1 = if d > th1 { xm7 } else { 0.0 };
                let s2 = if d > th2 { xm7 } else { 0.0 };
                let sq = fsqrt(d);
                let q = fdiv(k1, sq);
                let o0 = fmul(q, x6);
                let o2 = fmul(q, 0.0);
                let o1 = fmul(q, x5);
                // Bitwise blend: lane0 = o1&s0, lane1 = (~s1&blend)|o0&s1,
                // lane2 = o2&s2 (the fourth lanes read stack fill and are
                // masked out, so they contribute nothing).
                let l0 = f32::from_bits(
                    (!s0.to_bits() & blend[0]) | (o1.to_bits() & s0.to_bits()),
                );
                let l1 = f32::from_bits(
                    (!s1.to_bits() & blend[1]) | (o0.to_bits() & s1.to_bits()),
                );
                let l2 = f32::from_bits(
                    (!s2.to_bits() & blend[2]) | (o2.to_bits() & s2.to_bits()),
                );
                let sr = srows[oi];
                let mut dot = fadd(fadd(fmul(sr[0], l0), fmul(sr[1], l1)), fmul(sr[2], l2));
                if dot < 0.0 {
                    dot = 0.0;
                }
                let t = fadd(fmul(fmul(dot, ta), dot), tb);
                acc0 = fadd(fmul(rdf(eax.wrapping_add(0x1224)), t), acc0);
                eax = eax.wrapping_add(4);
                acc1 = fadd(fmul(rdf(eax.wrapping_sub(4)), t), acc1);
                acc2 = fadd(fmul(rdf(eax.wrapping_add(0xa4)), t), acc2);
            }
            let got: f32 =
                callee_thiscall!(C_INNER, f32, ecxarg, acc0.to_bits(), pushc);
            ((outp.wrapping_sub(0x10)) as *mut u32).write_unaligned(got.to_bits());
            (outp as *mut u32).write_unaligned(acc1.to_bits());
            ((outp.wrapping_add(0x10)) as *mut u32).write_unaligned(acc2.to_bits());
            ecxarg = ecxarg.wrapping_add(0x1c);
            outp = outp.wrapping_add(4);
        }
        // The immediate is an absolute data address: the loader relocates it.
        let pt = relocated(POST_THIS);
        callee_thiscall!(C_POST1, u32, pt, this.wrapping_add(0x29dc));
        callee_thiscall!(C_POST2, u32, pt, this.wrapping_add(0x29ec));
        last = callee_thiscall!(C_POST3, u32, pt, this.wrapping_add(0x29fc));
        last
    }
});
