// original: 0x00ae4880 net_sync_state_update (proposed)

/// Update the network sync singleton for one tick and fold its state out.
///
/// `arg` (called `edi` below) selects a row of the table at `SYNC_TABLE`.
/// `sg`, read from the global at `SINGLETON`, is the sync object; its vtable
/// slot `VT_STATE` answers the current sub-state id and slot `VT_PEER`
/// answers a peer record (or null).
///
/// Behaviour in order: mark the object present (`[sg+0x1b] = 1`); quit early
/// when the shutdown flag `F_QUIT` is set. From the flag word at `sg+0x8e8`
/// build four bit words, each `1 << shift` (shift is `[sg+0x938]`, x86
/// masked) when its flag bit (0x800, 0x10000, 0x8000, 0x2000000) is set.
/// Poll the state slot three times: on 0x15 OR the bit into `G_A` (and into
/// `G_B` when `[sg+0x3c]` is zero), on 0x1f store it into `G_C` and OR it
/// into `G_D`, on 0x11 OR it into `G_D`. Then OR the four bit words into
/// `G_E/F/G/H`. Poll the peer slot twice: when both answers are non-null,
/// the record's word at +0x30 is non-zero and its word at +0x38 is
/// (signed) positive, OR the bit into `G_I`. Quit early when `[sg+0x19]`
/// has bit 2 set.
///
/// Otherwise run the tick: read the rate float at `RATE_TAB + [RATE_IDX] *
/// 0x210`, clear `F_TICK`, publish the rate to `G_RATE`, and run the row
/// reset (intercepted callee 3, thiscall of the table row; the row is
/// zero-filled so the reset is a no-op). Save the position triple at
/// `sg+0x510/4/8` and the negated direction triple at `sg+0x500/4/8`
/// (negation flips the sign bit, exact for NaN). When the state slot answers
/// 0x1f, hand the saved triple to the region lookup (callee 4, cdecl; it
/// only reads). Convert the saved direction and bit word 0 to doubles and
/// hand them to the projection (callee 5, cdecl; its double answer's high
/// word is pinned by contract, see below) and publish the float conversion
/// to `G_PROJ` together with `[sg+0x764]` to `G_PROJ2`. Copy the saved
/// triple, the zero fill slot (uninitialized stack read as 0 under the
/// contract's `stack_fill`), the negated direction and `[sg+0x764]` into
/// `sg+0x910-0x930`, run the quantizer (callee 6, cdecl, read-only), scale
/// `[sg+0x8ec]` by `G_K` into `sg+0x934` and `G_K2`, and copy `[sg+0x8f0]`
/// to `G_K3`.
///
/// When `[sg+0x1a]` is non-zero, run the smoothing block: blend the
/// `sg+0x910/4/8` triple towards the `G_S0/1/2` triple with weights from the
/// negated direction, scaled by `G_MUL`, into `acc`; publish the shifted
/// triples to `G_S0-8/G_T0-3/G_S3`; unless `|acc - G_PREV| >= 1.0`, clamp
/// `acc / 48` at 1.0 from above into `one`, store `one * 75 + 5` to `G_OUT`,
/// and when `acc > 38.0` set `G_HIT` and clear `G_HIT2`. Always store `acc`
/// to `G_PREV`.
///
/// Then run the dispatcher (callee 7), and by `[sg+0x3c]` run the join
/// (callee 8) with (`edi`, 1 or 0) or the leave (callee 9) with (`edi`).
/// When `F_TAIL` is set, copy `G_CNT`'s low byte to `[sg+0x18]`; when
/// `F_MORE` is also set and the count exceeds `[sg+0x17]`, fill that many
/// words at `FILL_TAB + [sg+0x17] * 4` (callee 10, encrypted on disk: the
/// contract models it as a fixed 4-word fill of ones, write-only here).
/// Still within the tail, copy `G_SEQ` to `[sg+0x30]` unconditionally, and
/// when `F_MORE` is set run the notifier (callee 11) on `sg`.
/// Finally clear `G_Z0/G_Z1`.
///
/// Callees 7-9 and 11 genuinely write object and global state the stubbed
/// harness does not reproduce; downstream reads see the fabricated state on
/// both sides, which is what the comparison checks. The return value echoes
/// the last scripted answer and is not compared.
///
/// Original: 0x00ae4880 (cdecl, one stack word). Float and shift order below
/// is the original's, pinned through `black_box` and wrapping arithmetic.
lf_checker_rt::export!(cdecl, rw_00ae4880(arg: u32) -> u32 {
    unsafe {
        const SINGLETON: u32 = 0x012fb1b8;
        const F_QUIT: u32 = 0x015b0e5b;
        const VT_STATE: u32 = 0x24;
        const VT_PEER: u32 = 0x2c;
        const G_A: u32 = 0x0159b760;
        const G_B: u32 = 0x015b0e54;
        const G_C: u32 = 0x0159af24;
        const G_D: u32 = 0x0159af28;
        const G_E: u32 = 0x0159b758;
        const G_F: u32 = 0x0159b754;
        const G_G: u32 = 0x0159b750;
        const G_H: u32 = 0x0159b75c;
        const G_I: u32 = 0x0159af2c;
        const RATE_IDX: u32 = 0x01174790;
        const RATE_TAB: u32 = 0x015e89e0;
        const SYNC_TABLE: u32 = 0x01614c90;
        const F_TICK: u32 = 0x01593bb7;
        const G_RATE: u32 = 0x0103f6c0;
        const G_PROJ: u32 = 0x01593bb8;
        const G_PROJ2: u32 = 0x01593bbc;
        const G_S0: u32 = 0x015c15d0;
        const G_S1: u32 = 0x015c15d4;
        const G_S2: u32 = 0x015c15d8;
        const G_S3: u32 = 0x015c15dc;
        const G_S4: u32 = 0x015c15e0;
        const G_S5: u32 = 0x015c15e4;
        const G_S6: u32 = 0x015c15e8;
        const G_S7: u32 = 0x015c15ec;
        const G_T0: u32 = 0x015c3bb0;
        const G_T1: u32 = 0x015c3bb4;
        const G_T2: u32 = 0x015c3bb8;
        const G_T3: u32 = 0x015c3bbc;
        const G_OUT: u32 = 0x0103f6b8;
        const G_PREV: u32 = 0x01593bd8;
        const G_HIT: u32 = 0x01593bd0;
        const G_HIT2: u32 = 0x01593bd4;
        const G_X: u32 = 0x015b0e68;
        const G_K: u32 = 0x0103f6c4;
        const G_K2: u32 = 0x0103f6bc;
        const G_K3: u32 = 0x0103f6c8;
        const G_K4: u32 = 0x0103f724;
        const G_MUL: u32 = 0x011735c0;
        const G_CNT: u32 = 0x015b2b84;
        const G_SEQ: u32 = 0x01593bc8;
        const G_Z0: u32 = 0x015b0e60;
        const G_Z1: u32 = 0x015b0e64;
        const F_TAIL: u32 = 0x0103f719;
        const F_MORE: u32 = 0x015b0e91;
        const FILL_TAB: u32 = 0x015c3bd8;
        const C_NEGZERO: u32 = 0x00fe8fa0;
        const C_ONE: u32 = 0x00fe88e8;
        const C_ABS: u32 = 0x00fe8f80;
        const C_FIVE: u32 = 0x00fe8ad8;
        const C_INV48: u32 = 0x00ea77d0;
        const C_38: u32 = 0x00ea77d4;
        const ID_ROW_RESET: u32 = 3;
        const ID_REGION: u32 = 4;
        const ID_PROJ: u32 = 5;
        const ID_QUANT: u32 = 6;
        const ID_DISPATCH: u32 = 7;
        const ID_JOIN: u32 = 8;
        const ID_LEAVE: u32 = 9;
        const ID_FILL: u32 = 10;
        const ID_NOTIFY: u32 = 11;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn glob32(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn wglob32(va: u32, v: u32) {
            unsafe { wr32(lf_checker_rt::relocated(va), v) }
        }
        #[inline(always)]
        unsafe fn glob8(va: u32) -> u8 {
            unsafe { rd8(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn wglob8(va: u32, v: u8) {
            unsafe { wr8(lf_checker_rt::relocated(va), v) }
        }
        #[inline(always)]
        unsafe fn globf(va: u32) -> f32 {
            unsafe { f32::from_bits(glob32(va)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fneg_bits(b: u32) -> u32 {
            b ^ 0x8000_0000
        }

        let sg = glob32(SINGLETON);
        wr8(sg.wrapping_add(0x1b), 1);
        if glob8(F_QUIT) != 0 {
            return 0;
        }
        let flags = rd32(sg.wrapping_add(0x8e8));
        let shift = rd32(sg.wrapping_add(0x938));
        let bit = || 1u32.wrapping_shl(shift);
        let w0 = if flags & 0x800 != 0 { bit() } else { 0 };
        let w1 = if flags & 0x10000 != 0 { bit() } else { 0 };
        let w2 = if flags & 0x8000 != 0 { bit() } else { 0 };
        let w3 = if flags & 0x2000000 != 0 { bit() } else { 0 };
        let state: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(rd32(rd32(sg).wrapping_add(VT_STATE)) as usize) };
        let peer: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(rd32(rd32(sg).wrapping_add(VT_PEER)) as usize) };
        if state(sg) == 0x15 {
            wglob32(G_A, glob32(G_A) | bit());
            if rd32(sg.wrapping_add(0x3c)) == 0 {
                wglob32(G_B, glob32(G_B) | bit());
            }
        }
        if state(sg) == 0x1f {
            wglob32(G_C, bit());
            wglob32(G_D, glob32(G_D) | bit());
        }
        if state(sg) == 0x11 {
            wglob32(G_D, glob32(G_D) | bit());
        }
        wglob32(G_E, glob32(G_E) | w2);
        wglob32(G_F, glob32(G_F) | w1);
        wglob32(G_G, glob32(G_G) | w0);
        wglob32(G_H, glob32(G_H) | w3);
        let p = peer(sg);
        if p != 0 {
            let q = peer(sg);
            if rd32(q.wrapping_add(0x30)) != 0 && (rd32(q.wrapping_add(0x38)) as i32) > 0 {
                wglob32(G_I, glob32(G_I) | bit());
            }
        }
        if rd8(sg.wrapping_add(0x19)) & 4 != 0 {
            return 0;
        }
        let edi = arg;
        let idx = glob32(RATE_IDX);
        let rate = globf(RATE_TAB.wrapping_add(idx.wrapping_mul(0x210)));
        let row = SYNC_TABLE.wrapping_add(edi.wrapping_mul(0x54));
        wglob8(F_TICK, 0);
        wglob32(G_RATE, rate.to_bits());
        lf_checker_rt::callee_thiscall!(ID_ROW_RESET, u32, lf_checker_rt::relocated(row),);
        let s510 = rdf(sg.wrapping_add(0x510));
        let s514 = rdf(sg.wrapping_add(0x514));
        let s518 = rdf(sg.wrapping_add(0x518));
        let n500 = f32::from_bits(fneg_bits(rd32(sg.wrapping_add(0x500))));
        let n504 = f32::from_bits(fneg_bits(rd32(sg.wrapping_add(0x504))));
        let n508 = f32::from_bits(fneg_bits(rd32(sg.wrapping_add(0x508))));
        if state(sg) == 0x1f {
            let buf = [s510.to_bits(), s514.to_bits(), s518.to_bits(), 0u32];
            lf_checker_rt::callee_cdecl!(ID_REGION, u32, buf.as_ptr() as u32,);
            core::hint::black_box(&buf);
        }
        // Projection inputs as the original's cvtss2sd/cvtps2pd form them;
        // the stub answers the scripted double (high word pinned, low in eax).
        let d0 = f32::from_bits(fneg_bits(n500.to_bits())) as f64;
        let d1lo = n504 as f64;
        let d1hi = f32::from_bits(w0) as f64;
        let b0 = d0.to_bits();
        let b1 = d1lo.to_bits();
        let b2 = d1hi.to_bits();
        let ans_lo: u32 = lf_checker_rt::callee_cdecl!(
            ID_PROJ, u32,
            b0 as u32, (b0 >> 32) as u32,
            b1 as u32, (b1 >> 32) as u32,
            b2 as u32, (b2 >> 32) as u32,
        );
        let proj = f64::from_bits((ans_lo as u64) | (0x3ff8_0000u64 << 32));
        wglob32(G_PROJ, (proj as f32).to_bits());
        let s764 = rdf(sg.wrapping_add(0x764));
        wglob32(G_PROJ2, s764.to_bits());
        wrf(sg.wrapping_add(0x910), s510);
        wrf(sg.wrapping_add(0x914), s514);
        wrf(sg.wrapping_add(0x918), s518);
        wr32(sg.wrapping_add(0x91c), 0);
        wrf(sg.wrapping_add(0x920), n500);
        wrf(sg.wrapping_add(0x924), n504);
        wrf(sg.wrapping_add(0x928), n508);
        wr32(sg.wrapping_add(0x92c), 0);
        wrf(sg.wrapping_add(0x930), s764);
        {
            let buf = [s510.to_bits(), s514.to_bits(), s518.to_bits(), 0u32];
            lf_checker_rt::callee_cdecl!(ID_QUANT, u32, buf.as_ptr() as u32,);
            core::hint::black_box(&buf);
        }
        let s8ec = rdf(sg.wrapping_add(0x8ec));
        let k = globf(G_K);
        wrf(sg.wrapping_add(0x934), mul(s8ec, k));
        wglob32(G_K2, mul(s8ec, k).to_bits());
        wglob32(G_K3, rd32(sg.wrapping_add(0x8f0)));
        if rd8(sg.wrapping_add(0x1a)) != 0 {
            let s934 = rdf(sg.wrapping_add(0x934));
            let g0 = globf(G_S0);
            let g1 = globf(G_S1);
            let g2 = globf(G_S2);
            wglob32(G_X, s934.to_bits());
            let mut t7 = rdf(sg.wrapping_add(0x910));
            let mut t5 = rdf(sg.wrapping_add(0x914));
            let mut t6 = rdf(sg.wrapping_add(0x918));
            wglob32(G_S4, t7.to_bits());
            wglob32(G_S5, t5.to_bits());
            wglob32(G_S6, t6.to_bits());
            let t4 = rdf(sg.wrapping_add(0x91c));
            wglob32(G_S0, t7.to_bits());
            t7 = sub(t7, g0);
            let g3 = globf(G_S3);
            wglob32(G_S1, t5.to_bits());
            t5 = sub(t5, g1);
            wglob32(G_S2, t6.to_bits());
            t7 = mul(t7, n500);
            t6 = sub(t6, g2);
            t5 = mul(t5, n504);
            wglob32(G_T3, g3.to_bits());
            wglob32(G_T0, g0.to_bits());
            t6 = mul(t6, n508);
            t7 = add(t7, t5);
            let mut one = globf(C_ONE);
            wglob32(G_S7, t4.to_bits());
            wglob32(G_T1, g1.to_bits());
            wglob32(G_T2, g2.to_bits());
            t7 = add(t7, t6);
            wglob32(G_S3, t4.to_bits());
            t7 = mul(t7, globf(G_MUL));
            let diff = f32::from_bits(sub(t7, globf(G_PREV)).to_bits() & glob32(C_ABS));
            if one > diff {
                let t = mul(t7, globf(C_INV48));
                if !(t > one) {
                    one = t;
                }
                wglob32(G_OUT, add(mul(one, globf(G_K4)), globf(C_FIVE)).to_bits());
                if t7 > globf(C_38) {
                    wglob8(G_HIT, 1);
                    wglob32(G_HIT2, 0);
                }
            }
            wglob32(G_PREV, t7.to_bits());
        }
        lf_checker_rt::callee_cdecl!(ID_DISPATCH, u32,);
        let mode = rd32(sg.wrapping_add(0x3c));
        if mode == 1 {
            lf_checker_rt::callee_cdecl!(ID_JOIN, u32, edi, mode,);
        } else if mode == 0 {
            lf_checker_rt::callee_cdecl!(ID_JOIN, u32, edi, mode,);
        } else {
            lf_checker_rt::callee_cdecl!(ID_LEAVE, u32, edi,);
        }
        if glob8(F_TAIL) != 0 {
            let cnt = glob32(G_CNT);
            wr8(sg.wrapping_add(0x18), cnt as u8);
            if glob8(F_MORE) != 0 {
                let dl = rd8(sg.wrapping_add(0x17)) as u32;
                // Original zero-extends both bytes, then signed-compares.
                let cc = (cnt & 0xff).wrapping_sub(dl);
                if (cc as i32) > 0 {
                    let dst = FILL_TAB.wrapping_add(dl.wrapping_mul(4));
                    lf_checker_rt::callee_cdecl!(ID_FILL, u32, cc, lf_checker_rt::relocated(dst), 1u32,);
                }
            }
            // Unconditional within the tail: only the fill and notify are gated.
            wr32(sg.wrapping_add(0x30), glob32(G_SEQ));
            if glob8(F_MORE) != 0 {
                lf_checker_rt::callee_cdecl!(ID_NOTIFY, u32, sg,);
            }
        }
        wglob32(G_Z0, 0);
        wglob32(G_Z1, 0);
        0
    }
});
