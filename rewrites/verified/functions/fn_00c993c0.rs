// original: 0x00c993c0 ped_task_aim_solve_full (proposed)

/// Solve a full aiming pose from task records, returning its error measure.
///
/// `thiscall`: object in `ecx`, ten stack words, callee pops 40. Returns a
/// float in ST0 (the final error slot). Arguments: two small record indices
/// (`a08`, `a10`), two opaque words passed to callees (`a0c`, `a14`), a
/// three-float direction block (`a18`), a float scratch slot that is read
/// and rewritten (`a1c`), two more floats (`a20`, `a28`), a flag byte in the
/// low byte of `a24` selecting a long first phase, and an unread word.
///
/// Layout read: `this+0x40` is the sub-object `S`; `S+0x2E` (signed word)
/// indexes the global object table; `S+0x100` is the fallback holder used
/// when the slot-`0xA0` resolver answers null; `this+0x138/0x13C` hold two
/// candidate floats. Each fetched record holds twelve floats at offsets
/// `0x00..0x38` with gaps at `0x0C/0x1C/0x2C`.
///
/// Behaviour: four record fetches seed the frame; when the flag byte is
/// nonzero a long first phase blends direction triples and stores blended
/// records back; two resolver blocks scale the indices by 224 and read
/// entry triples; two evaluator calls consume frame windows; two angle
/// phases widen floats to doubles across an atan2-style helper and a
/// float helper; a flag word in global memory selects cached or computed
/// coefficient triples; two structuring calls fill frame windows whose
/// floats are stored into the records; two final calls run, and the error
/// slot is returned. All float arithmetic is scalar SSE in the original's
/// operand order; doubles are exact widenings around the helper calls.
/// NaN selection is destination-first, as the hardware does: when either
/// operand of an add/mul is NaN the first source's NaN (quieted) wins,
/// so the `fadd`/`fmul` helpers select an NaN operand explicitly instead
/// of relying on which source the compiler makes the destination.
///
/// Comparisons: `cmp`+`jne` on the first index against a fetch answer is a
/// plain 32-bit inequality (both paths covered); `comiss`+`jbe` is taken
/// when unordered, i.e. `!(x > y)`; `comiss`+`ja` is ordered `x > y`;
/// `ucomiss`+`lahf`+`(an instruction of the original)` with `jp`/`jnp` tests IEEE
/// inequality/equality (`jp` exactly when the operands differ, NaN included);
/// `(an instruction of the original)` test bits of the global flag word (all four combinations
/// covered). Edge cases: NaN, infinities and denormals flow through
/// bit-exactly; a null resolver answer selects the fallback holder; a zero
/// flag byte skips the first phase; the unread stack word is ignored.
lf_checker_rt::export!(thiscall, rw_00c993c0(this: u32, a08: u32, a0c: u32, a10: u32, a14: u32, a18: u32, a1c: u32, a20: u32, a24: u32, a28: u32, _a2c: u32) -> f32 {
    unsafe {
        const SUB: u32 = 0x40;
        const SUB_INDEX: u32 = 0x2E;
        const SUB_ALT: u32 = 0x100;
        const TABLE: u32 = 0x01295CD8;
        const VT_FETCH: u32 = 0x38;
        const VT_RESOLVE: u32 = 0xA0;
        const VT_PAIR: u32 = 0xE0;
        const CAND_A: u32 = 0x138;
        const CAND_B: u32 = 0x13C;
        const ENTRY_STRIDE: i32 = 0xE0;
        const M_RECORD: u32 = 1;
        const M_COMBINE: u32 = 5;
        const M_FILL: u32 = 6;
        const M_ATAN2: u32 = 7;
        const M_SHAPE: u32 = 8;
        const M_STRUCT: u32 = 9;
        const M_RUN_A: u32 = 10;
        const M_RUN_B: u32 = 11;
        const G_LIMIT: u32 = 0x00FE8734;
        const G_ONE: u32 = 0x00FE88E8;
        const G_TENTH: u32 = 0x01050B78;
        const G_SIGN: u32 = 0x00FE8FA0;
        const G_FOUR_TENTHS: u32 = 0x00FE881C;
        const G_BLEND: u32 = 0x011735BC;
        const G_TWO_A: u32 = 0x01050B6C;
        const G_TWO_B: u32 = 0x00FE8A24;
        const G_TWOHUNDRED: u32 = 0x00FE8BF4;
        const G_PI: u32 = 0x00FE8AA0;
        const G_NPI: u32 = 0x00FE8DC4;
        const G_THREE_Q: u32 = 0x00FE888C;
        const G_NTHREE_Q: u32 = 0x00E9B50C;
        const G_ABS: u32 = 0x00FE8F80;
        const FLAG_BASE: u32 = 0x0171BB70;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { (a as *const f32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut f32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16s(a: u32) -> i32 {
            unsafe { (a as *const u16).read_unaligned() as i16 as i32 }
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            let a = core::hint::black_box(a);
            let b = core::hint::black_box(b);
            // Dest-first NaN selection, matching `addss`: when either
            // operand is NaN the destination operand's NaN (quieted) is
            // the result. `black_box` alone does not pin which operand
            // becomes the destination: LLVM treats SSE add/mul as
            // commutative and may emit the second source as the
            // destination, which flips the NaN sign when both operands
            // are NaN with different signs. Select explicitly so the
            // emitted destination choice cannot matter.
            if a.is_nan() {
                f32::from_bits(a.to_bits() | 0x00400000)
            } else if b.is_nan() {
                f32::from_bits(b.to_bits() | 0x00400000)
            } else {
                a + b
            }
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            let a = core::hint::black_box(a);
            let b = core::hint::black_box(b);
            // Dest-first NaN selection, matching `mulss` (same reason
            // as in `fadd` above: the destination is the first source).
            if a.is_nan() {
                f32::from_bits(a.to_bits() | 0x00400000)
            } else if b.is_nan() {
                f32::from_bits(b.to_bits() | 0x00400000)
            } else {
                a * b
            }
        }
        #[inline(always)]
        fn fdiv(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsqrt(a: f32) -> f32 {
            core::hint::black_box(a).sqrt()
        }
        #[inline(always)]
        fn fabs(a: f32, mask: u32) -> f32 {
            f32::from_bits(a.to_bits() & mask)
        }
        #[inline(always)]
        fn fxor(a: f32, mask: u32) -> f32 {
            f32::from_bits(a.to_bits() ^ mask)
        }
        /// `comiss x, y` + `jbe`: taken unless x > y.
        #[inline(always)]
        fn jbe(x: f32, y: f32) -> bool {
            !(x > y)
        }
        /// `ucomiss x, y` + `lahf` + `(an instruction of the original)` + `jp`: IEEE x != y.
        #[inline(always)]
        fn jne_f(x: f32, y: f32) -> bool {
            x != y
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 {
            unsafe { lf_checker_rt::global::<f32>(va).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn v_fetch(obj: u32, id: u32) -> u32 {
            unsafe {
                let t: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + VT_FETCH) as usize);
                t(obj, id)
            }
        }
        #[inline(always)]
        unsafe fn v_resolve(obj: u32) -> u32 {
            unsafe {
                let t: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + VT_RESOLVE) as usize);
                t(obj)
            }
        }
        #[inline(always)]
        unsafe fn v_pair(obj: u32) -> u32 {
            unsafe {
                let t: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + VT_PAIR) as usize);
                t(obj)
            }
        }
        #[inline(always)]
        unsafe fn table_obj(index: i32) -> u32 {
            unsafe {
                g32(TABLE.wrapping_add(index.wrapping_mul(4) as u32))
            }
        }
        #[inline(always)]
        unsafe fn atan2_call(y: f32, x: f32) -> f32 {
            unsafe {
                let dy = (y as f64).to_bits();
                let dx = (x as f64).to_bits();
                let ans: u64 = lf_checker_rt::callee_cdecl!(
                    M_ATAN2, u64,
                    (dy & 0xffff_ffff) as u32, (dy >> 32) as u32,
                    (dx & 0xffff_ffff) as u32, (dx >> 32) as u32);
                f64::from_bits(ans) as f32
            }
        }
        #[inline(always)]
        unsafe fn shape_call() -> f32 {
            unsafe {
                f32::from_bits(lf_checker_rt::callee_cdecl!(M_SHAPE, u32,))
            }
        }

        // Frame slots fr[i] mirror the original's aligned-frame dword at
        // ESP0+4*i (pushes inside call setups are folded into the indices).
        let mut fr = [0u32; 112];
        let fslot = fr.as_mut_ptr() as u32;
        #[inline(always)]
        fn slot_ptr(fslot: u32, i: usize) -> u32 {
            fslot.wrapping_add((i as u32).wrapping_mul(4))
        }
        macro_rules! frf {
            ($fr:ident[$i:expr]) => {
                f32::from_bits($fr[$i])
            };
        }
        macro_rules! setf {
            ($fr:ident[$i:expr] = $v:expr) => {
                $fr[$i] = ($v).to_bits()
            };
        }

        let abs_mask = g32(G_ABS);
        let sign_mask = g32(G_SIGN);
        let mut v_a1c = f32::from_bits(a1c);

        // --- four record fetches ---
        let sub0 = rd32(this + SUB);
        let r_m1 = lf_checker_rt::callee_thiscall!(M_RECORD, u32, sub0, a0c);
        fr[30] = this;
        let r_m2 = lf_checker_rt::callee_thiscall!(M_RECORD, u32, sub0, a08);
        let m_first = r_m1;
        let m_second = r_m2;
        let r_m3 = lf_checker_rt::callee_thiscall!(M_RECORD, u32, sub0, a10);
        fr[22] = r_m3;
        let r_m4 = lf_checker_rt::callee_thiscall!(M_RECORD, u32, sub0, a14);
        fr[24] = r_m4;
        fr[67] = 0;

        // --- long first phase, skipped when the flag byte is zero ---
        if (a24 & 0xFF) != 0 {
            let mut x1 = f32::from_bits(a1c);
            let mut x0 = f32::from_bits(a20);
            x1 = fadd(x1, x0);
            x0 = fsub(x0, gf(G_LIMIT));
            let mut x6 = gf(G_ONE);
            x1 = fsub(x1, rdf(m_first + 0x38));
            x0 = fdiv(x0, gf(G_TENTH));
            v_a1c = x1;
            x1 = x6;
            x1 = fsub(x1, x0);
            let mut x5 = 0.0f32;
            if !jbe(x5, x1) {
                x1 = x5;
            } else {
                if x1 > x6 {
                    x1 = x6;
                }
            }
            x1 = fmul(x1, f32::from_bits(a28));
            let mut x2 = rdf(m_second);
            let mut x4 = rdf(m_second + 4);
            let mut x3 = rdf(m_second + 8);
            let dir = a18;
            setf!(fr[16] = x1);
            let mut x7 = rdf(dir);
            x1 = x2;
            x1 = fadd(x1, x4);
            x0 = x3;
            x0 = fmul(x0, x6);
            x6 = rdf(dir + 4);
            x1 = fmul(x1, x5);
            x5 = rdf(dir + 8);
            x1 = fsub(x1, x0);
            x0 = gf(G_SIGN);
            x5 = fxor(x5, sign_mask);
            x6 = fxor(x6, sign_mask);
            x7 = fxor(x7, sign_mask);
            x0 = frf!(fr[16]);
            x5 = fsub(x5, x3);
            x7 = fsub(x7, x2);
            x6 = fsub(x6, x4);
            x5 = fmul(x5, x0);
            x6 = fmul(x6, x0);
            x7 = fmul(x7, x0);
            x0 = rdf(m_second + 0x2c);
            x5 = fmul(x5, x1);
            x6 = fmul(x6, x1);
            x7 = fmul(x7, x1);
            x1 = rdf(m_second + 0x24);
            x5 = fadd(x5, x3);
            x7 = fadd(x7, x2);
            x2 = rdf(m_second + 0x28);
            setf!(fr[20] = x0);
            x6 = fadd(x6, x4);
            x4 = rdf(m_second + 0x20);
            x0 = x2;
            x3 = x1;
            x3 = fmul(x3, x5);
            x0 = fmul(x0, x6);
            setf!(fr[40] = x2);
            x2 = fmul(x2, x7);
            x3 = fsub(x3, x0);
            x0 = x4;
            x0 = fmul(x0, x5);
            setf!(fr[8] = x1);
            x2 = fsub(x2, x0);
            x0 = x1;
            x0 = fmul(x0, x7);
            x7 = x1;
            setf!(fr[12] = x2);
            x2 = x4;
            x2 = fmul(x2, x6);
            x6 = frf!(fr[40]);
            x5 = x7;
            x2 = fsub(x2, x0);
            x0 = x4;
            x0 = fmul(x0, x4);
            x5 = fmul(x5, x7);
            x1 = 0.0;
            x5 = fadd(x5, x0);
            x0 = x6;
            x0 = fmul(x0, x6);
            x5 = fadd(x5, x0);
            if jne_f(x5, x1) {
                x0 = 0.0;
                x0 = fsqrt(x5);
                x5 = gf(G_ONE);
                x7 = x5;
                x7 = fdiv(x7, x0);
                setf!(fr[16] = x7);
                x7 = frf!(fr[8]);
                x0 = frf!(fr[16]);
            } else {
                x5 = gf(G_ONE);
                x0 = x1;
            }
            x4 = fmul(x4, x0);
            x6 = fmul(x6, x0);
            x7 = fmul(x7, x0);
            setf!(fr[4] = x4);
            x4 = frf!(fr[12]);
            x0 = x2;
            x2 = fmul(x2, frf!(fr[4]));
            x0 = fmul(x0, x7);
            x4 = fmul(x4, x6);
            setf!(fr[40] = x6);
            setf!(fr[8] = x7);
            x4 = fsub(x4, x0);
            x0 = x3;
            x0 = fmul(x0, x6);
            x3 = fmul(x3, x7);
            x2 = fsub(x2, x0);
            x0 = frf!(fr[12]);
            x0 = fmul(x0, frf!(fr[4]));
            x3 = fsub(x3, x0);
            x0 = x2;
            x0 = fmul(x0, x2);
            setf!(fr[12] = x0);
            x6 = frf!(fr[12]);
            x0 = x4;
            x0 = fmul(x0, x4);
            x6 = fadd(x6, x0);
            x0 = x3;
            x0 = fmul(x0, x3);
            x6 = fadd(x6, x0);
            x0 = x6;
            setf!(fr[12] = x6);
            if jne_f(x0, x1) {
                x0 = fsqrt(x0);
                x5 = fdiv(x5, x0);
            } else {
                x5 = x1;
            }
            x1 = frf!(fr[40]);
            x6 = frf!(fr[4]);
            x2 = fmul(x2, x5);
            x3 = fmul(x3, x5);
            x0 = x1;
            x4 = fmul(x4, x5);
            x0 = fmul(x0, x2);
            x5 = x7;
            x5 = fmul(x5, x3);
            x1 = fmul(x1, x4);
            x5 = fsub(x5, x0);
            x0 = x6;
            x0 = fmul(x0, x3);
            setf!(fr[16] = x4);
            setf!(fr[36] = x2);
            x1 = fsub(x1, x0);
            x0 = x7;
            x0 = fmul(x0, x4);
            setf!(fr[28] = x3);
            setf!(fr[52] = x1);
            x1 = x6;
            x1 = fmul(x1, x2);
            setf!(fr[12] = x5);
            x1 = fsub(x1, x0);
            x0 = v_a1c;
            x0 = fmul(x0, f32::from_bits(a28));
            setf!(fr[44] = x1);
            x1 = gf(G_FOUR_TENTHS);
            v_a1c = x0;
            if x1 > x0 {
                // ja taken: keep x0
            } else {
                x0 = x1;
                v_a1c = x0;
            }

            // --- fetch 5 selects the candidate, blended records stored back ---
            let fetch5 = v_fetch(table_obj(rd16s(rd32(this + SUB) + SUB_INDEX)), 0x12);
            x1 = gf(G_BLEND);
            x3 = v_a1c;
            x1 = fmul(x1, gf(G_TWO_A));
            x4 = x3;
            x5 = if a08 != fetch5 { rdf(this + CAND_B) } else { rdf(this + CAND_A) };
            x4 = fsub(x4, x5);
            x2 = x4;
            x2 = fabs(x2, abs_mask);
            if !(x1 > x2) {
                x0 = x1;
                x0 = fmul(x0, gf(G_TWOHUNDRED));
                if jbe(x2, x0) || !(x1 > gf(G_LIMIT)) {
                    x0 = 0.0;
                    x3 = x5;
                    if jbe(x0, x4) {
                        x3 = fadd(x3, x1);
                    } else {
                        x3 = fsub(x3, x1);
                    }
                }
            }
            x6 = frf!(fr[16]);
            x4 = frf!(fr[36]);
            x5 = frf!(fr[28]);
            x2 = frf!(fr[63]);
            x0 = frf!(fr[12]);
            x1 = frf!(fr[103]);
            wrf(m_second, x6);
            wrf(m_second + 4, x4);
            wrf(m_second + 8, x5);
            wrf(m_second + 0xc, x2);
            wrf(m_second + 0x10, x0);
            x0 = frf!(fr[52]);
            wrf(m_second + 0x14, x0);
            x0 = frf!(fr[44]);
            wrf(m_second + 0x18, x0);
            x0 = frf!(fr[8]);
            x7 = frf!(fr[4]);
            wrf(m_second + 0x1c, x1);
            wrf(m_second + 0x24, x0);
            x0 = frf!(fr[40]);
            wrf(m_second + 0x28, x0);
            x0 = frf!(fr[20]);
            wrf(m_second + 0x2c, x0);
            wrf(m_second + 0x20, x7);
            x0 = rdf(m_second + 0x38);
            x0 = fadd(x0, x3);
            setf!(fr[67] = x3);
            wrf(m_second + 0x38, x0);
            x0 = frf!(fr[12]);
            wrf(m_first, x0);
            x0 = frf!(fr[52]);
            wrf(m_first + 4, x0);
            x0 = frf!(fr[44]);
            wrf(m_first + 8, x0);
            x0 = frf!(fr[8]);
            wrf(m_first + 0xc, x1);
            wrf(m_first + 0x14, x0);
            x0 = frf!(fr[40]);
            wrf(m_first + 0x18, x0);
            x0 = frf!(fr[20]);
            wrf(m_first + 0x1c, x0);
            wrf(m_first + 0x10, x7);
            wrf(m_first + 0x20, x6);
            wrf(m_first + 0x24, x4);
            wrf(m_first + 0x28, x5);
            wrf(m_first + 0x2c, x2);
            x0 = rdf(m_first + 0x38);
            x0 = fadd(x0, x3);
            wrf(m_first + 0x38, x0);
        }

        // --- resolver blocks scale the indices and read entry triples ---
        let sub = rd32(fr[30] + SUB);
        fr[20] = sub;
        let holder_a = if v_resolve(sub) == 0 {
            rd32(sub + SUB_ALT)
        } else {
            v_pair(v_resolve(sub))
        };
        let grp_a = rd32(holder_a + 4);
        let arr_a = rd32(grp_a);
        let off_a = (a10 as i32).wrapping_mul(ENTRY_STRIDE) as u32;
        setf!(fr[36] = rdf(arr_a.wrapping_add(off_a).wrapping_add(0x20)));
        setf!(fr[28] = rdf(arr_a.wrapping_add(off_a).wrapping_add(0x24)));
        setf!(fr[16] = rdf(arr_a.wrapping_add(off_a).wrapping_add(0x28)));
        fr[20] = sub;
        let holder_b = if v_resolve(sub) == 0 {
            rd32(sub + SUB_ALT)
        } else {
            v_pair(v_resolve(sub))
        };
        let grp_b = rd32(holder_b + 4);
        let arr_b = rd32(grp_b);
        let off_b = (a08 as i32).wrapping_mul(ENTRY_STRIDE) as u32;
        let mut x3 = frf!(fr[28]);
        let mut x1 = rdf(arr_b.wrapping_add(off_b).wrapping_add(0x20));
        let mut x2 = rdf(arr_b.wrapping_add(off_b).wrapping_add(0x24));
        let mut x4 = frf!(fr[36]);
        let mut x0 = rdf(arr_b.wrapping_add(off_b).wrapping_add(0x28));
        let m_fourth = fr[24];
        x4 = fmul(x4, x4);
        x2 = fmul(x2, x2);
        x3 = fmul(x3, x3);
        x1 = fmul(x1, x1);
        x3 = fadd(x3, x4);
        x4 = frf!(fr[16]);
        x0 = fmul(x0, x0);
        let mut x5 = rdf(m_fourth + 0x34);
        x5 = fsub(x5, rdf(m_second + 0x34));
        let mut x6 = rdf(m_fourth + 0x38);
        x2 = fadd(x2, x1);
        x6 = fsub(x6, rdf(m_second + 0x38));
        x4 = fmul(x4, x4);
        let mut x7 = x5;
        x2 = fadd(x2, x0);
        x3 = fadd(x3, x4);
        x4 = rdf(m_fourth + 0x30);
        x4 = fsub(x4, rdf(m_second + 0x30));
        x7 = fmul(x7, x5);
        x1 = fsqrt(x2);
        x3 = fsqrt(x3);
        x0 = x4;
        x0 = fmul(x0, x4);
        x2 = x1;
        x2 = fadd(x2, x3);
        x7 = fadd(x7, x0);
        x0 = x6;
        x0 = fmul(x0, x6);
        setf!(fr[4] = x3);
        setf!(fr[24] = x2);
        x7 = fadd(x7, x0);
        x0 = 0.0;
        x0 = fsqrt(x7);
        setf!(fr[36] = x7);
        setf!(fr[28] = x0);
        if x0 > x2 {
            x0 = 0.0;
            if jne_f(x7, x0) {
                x0 = fsqrt(x7);
                x1 = gf(G_ONE);
                x1 = fdiv(x1, x0);
            } else {
                x1 = x0;
            }
            x3 = rdf(m_fourth + 0x30);
            x4 = fmul(x4, x1);
            x5 = fmul(x5, x1);
            x6 = fmul(x6, x1);
            setf!(fr[12] = x4);
            let m_third = fr[22];
            x0 = x4;
            x4 = rdf(m_fourth + 0x34);
            x0 = fmul(x0, x2);
            x1 = x5;
            x1 = fmul(x1, x2);
            x3 = fsub(x3, x0);
            x2 = x6;
            x2 = fmul(x2, frf!(fr[24]));
            x4 = fsub(x4, x1);
            x3 = fsub(x3, rdf(m_second + 0x30));
            x1 = rdf(m_fourth + 0x38);
            x1 = fsub(x1, x2);
            x4 = fsub(x4, rdf(m_second + 0x34));
            x1 = fsub(x1, rdf(m_second + 0x38));
            x0 = x3;
            x0 = fadd(x0, rdf(m_second + 0x30));
            wrf(m_second + 0x30, x0);
            x0 = x4;
            x0 = fadd(x0, rdf(m_second + 0x34));
            wrf(m_second + 0x34, x0);
            x0 = x1;
            x0 = fadd(x0, rdf(m_second + 0x38));
            wrf(m_second + 0x38, x0);
            x1 = fadd(x1, rdf(m_first + 0x38));
            x3 = fadd(x3, rdf(m_first + 0x30));
            x4 = fadd(x4, rdf(m_first + 0x34));
            x0 = frf!(fr[12]);
            wrf(m_first + 0x38, x1);
            x1 = frf!(fr[4]);
            x0 = fmul(x0, x1);
            wrf(m_first + 0x30, x3);
            wrf(m_first + 0x34, x4);
            x2 = rdf(m_fourth + 0x30);
            x6 = fmul(x6, x1);
            x2 = fsub(x2, x0);
            x0 = rdf(m_fourth + 0x38);
            x5 = fmul(x5, x1);
            x1 = rdf(m_fourth + 0x34);
            x0 = fsub(x0, x6);
            x1 = fsub(x1, x5);
            wrf(m_third + 0x30, x2);
            wrf(m_third + 0x38, x0);
            x0 = frf!(fr[63]);
            wrf(m_third + 0x34, x1);
            wrf(m_third + 0x3c, x0);
        } else {
            x7 = frf!(fr[28]);
            x1 = fmul(x1, x1);
            x0 = fmul(x0, x7);
            x3 = fmul(x3, x3);
            setf!(fr[24] = x1);
            x2 = x6;
            setf!(fr[4] = x3);
            x3 = x1;
            x3 = fsub(x3, frf!(fr[4]));
            x1 = x5;
            x3 = fadd(x3, x0);
            x0 = x7;
            x0 = fmul(x0, gf(G_TWO_B));
            x3 = fdiv(x3, x0);
            x0 = x4;
            x0 = fmul(x0, x3);
            x1 = fmul(x1, x3);
            setf!(fr[8] = x0);
            x0 = gf(G_ONE);
            x0 = fdiv(x0, x7);
            x7 = frf!(fr[8]);
            x7 = fmul(x7, x0);
            x1 = fmul(x1, x0);
            x2 = fmul(x2, x3);
            setf!(fr[8] = x7);
            x3 = fmul(x3, x3);
            x2 = fmul(x2, x0);
            x0 = x7;
            x0 = fadd(x0, rdf(m_second + 0x30));
            setf!(fr[8] = x0);
            x0 = rdf(m_second + 0x34);
            x0 = fadd(x0, x1);
            x1 = frf!(fr[24]);
            x1 = fsub(x1, x3);
            setf!(fr[16] = x0);
            x0 = rdf(m_second + 0x38);
            x0 = fadd(x0, x2);
            setf!(fr[52] = x0);
            x0 = 0.0;
            if !(x0 >= x1) {
                x1 = fsqrt(x1);
                setf!(fr[44] = x1);
            } else {
                setf!(fr[44] = x0);
            }
            x7 = frf!(fr[36]);
            if jne_f(x7, x0) {
                x0 = fsqrt(x7);
                x1 = gf(G_ONE);
                x1 = fdiv(x1, x0);
            } else {
                x1 = x0;
            }
            let m_third_b = fr[22];
            x5 = fmul(x5, x1);
            x6 = fmul(x6, x1);
            x4 = fmul(x4, x1);
            setf!(fr[20] = x6);
            x1 = frf!(fr[20]);
            setf!(fr[12] = x4);
            setf!(fr[28] = x5);
            x7 = x5;
            x7 = fmul(x7, rdf(m_third_b + 0x28));
            x0 = x6;
            x0 = fmul(x0, rdf(m_third_b + 0x24));
            x6 = fmul(x6, rdf(m_third_b + 0x20));
            x7 = fsub(x7, x0);
            x0 = x4;
            x0 = fmul(x0, rdf(m_third_b + 0x28));
            x2 = x4;
            x2 = fmul(x2, rdf(m_third_b + 0x24));
            x6 = fsub(x6, x0);
            x0 = x5;
            x0 = fmul(x0, rdf(m_third_b + 0x20));
            x3 = x4;
            x4 = gf(G_SIGN);
            x2 = fsub(x2, x0);
            x3 = fxor(x3, sign_mask);
            x1 = fxor(x1, sign_mask);
            x0 = x5;
            x0 = fxor(x0, sign_mask);
            setf!(fr[24] = x0);
            x0 = fmul(x0, x2);
            x4 = x3;
            x3 = fmul(x3, x6);
            x4 = fmul(x4, x2);
            x2 = frf!(fr[24]);
            x5 = x1;
            x2 = fmul(x2, x7);
            x1 = fmul(x1, x7);
            x2 = fsub(x2, x3);
            x3 = frf!(fr[20]);
            x5 = fmul(x5, x6);
            x4 = fsub(x4, x1);
            x1 = frf!(fr[28]);
            x5 = fsub(x5, x0);
            x6 = x3;
            x0 = x2;
            x0 = fmul(x0, x1);
            x6 = fmul(x6, x4);
            x3 = fmul(x3, x5);
            x6 = fsub(x6, x0);
            x0 = frf!(fr[12]);
            x2 = fmul(x2, x0);
            x0 = fmul(x0, x4);
            x2 = fsub(x2, x3);
            x1 = fmul(x1, x5);
            x1 = fsub(x1, x0);
            x0 = x6;
            x0 = fmul(x0, x6);
            x3 = x2;
            x3 = fmul(x3, x2);
            x3 = fadd(x3, x0);
            x0 = x1;
            x0 = fmul(x0, x1);
            x3 = fadd(x3, x0);
            x0 = 0.0;
            if jne_f(x3, x0) {
                x0 = fsqrt(x3);
                x3 = gf(G_ONE);
                x3 = fdiv(x3, x0);
            } else {
                x3 = x0;
            }
            x0 = frf!(fr[44]);
            x6 = fmul(x6, x3);
            x2 = fmul(x2, x3);
            x1 = fmul(x1, x3);
            x6 = fmul(x6, x0);
            x2 = fmul(x2, x0);
            x6 = fadd(x6, frf!(fr[8]));
            x1 = fmul(x1, x0);
            x2 = fadd(x2, frf!(fr[16]));
            x0 = frf!(fr[63]);
            x1 = fadd(x1, frf!(fr[52]));
            wrf(m_third_b + 0x30, x6);
            wrf(m_third_b + 0x3c, x0);
            wrf(m_third_b + 0x34, x2);
            wrf(m_third_b + 0x38, x1);
        }

        // --- combiner and fill calls, then three more fetch pairs ---
        lf_checker_rt::callee_thiscall!(M_COMBINE, u32, this, a14, a10);
        let m_arg8 = a08;
        lf_checker_rt::callee_thiscall!(M_COMBINE, u32, this, a10, m_arg8);
        lf_checker_rt::callee_thiscall!(M_FILL, u32, this, a0c, m_arg8);
        let fetch_a = v_fetch(table_obj(rd16s(rd32(this + SUB) + SUB_INDEX)), 0x13);
        let rec_a = lf_checker_rt::callee_thiscall!(M_RECORD, u32, sub, fetch_a);
        let fetch_b = v_fetch(table_obj(rd16s(rd32(this + SUB) + SUB_INDEX)), 0x12);
        fr[16] = rec_a;
        let rec_b = lf_checker_rt::callee_thiscall!(M_RECORD, u32, sub, fetch_b);
        let fetch_c = v_fetch(table_obj(rd16s(rd32(this + SUB) + SUB_INDEX)), 0x11);
        let m_b = rec_b;
        let rec_c = lf_checker_rt::callee_thiscall!(M_RECORD, u32, sub, fetch_c);

        // --- long blend of the three new records ---
        let mut ea = rec_c;
        x0 = rdf(m_b + 0x30);
        x1 = rdf(m_b + 4);
        x6 = rdf(ea + 4);
        x4 = rdf(ea);
        x3 = rdf(m_b);
        x5 = rdf(ea + 8);
        setf!(fr[80] = x0);
        x0 = rdf(m_b + 0x34);
        setf!(fr[81] = x0);
        x0 = rdf(m_b + 0x38);
        x2 = rdf(ea + 0x14);
        setf!(fr[82] = x0);
        x0 = x1;
        x0 = fmul(x0, x6);
        x3 = fmul(x3, x4);
        x7 = rdf(m_b + 0x14);
        x2 = fmul(x2, x1);
        x3 = fadd(x3, x0);
        x0 = rdf(m_b + 8);
        x1 = rdf(ea + 0x24);
        x0 = fmul(x0, x5);
        x1 = fmul(x1, rdf(m_b + 4));
        x3 = fadd(x3, x0);
        x0 = rdf(m_b);
        x0 = fmul(x0, rdf(ea + 0x10));
        fr[65] = ea;
        x2 = fadd(x2, x0);
        x0 = rdf(ea + 0x18);
        x0 = fmul(x0, rdf(m_b + 8));
        x2 = fadd(x2, x0);
        x0 = rdf(m_b);
        x0 = fmul(x0, rdf(ea + 0x20));
        x1 = fadd(x1, x0);
        x0 = rdf(ea + 0x28);
        x0 = fmul(x0, rdf(m_b + 8));
        x1 = fadd(x1, x0);
        setf!(fr[68] = x3);
        setf!(fr[69] = x2);
        x2 = rdf(m_b + 0x10);
        x0 = x7;
        x0 = fmul(x0, x6);
        x3 = x2;
        x3 = fmul(x3, x4);
        setf!(fr[70] = x1);
        x1 = rdf(m_b + 0x18);
        x3 = fadd(x3, x0);
        x0 = x1;
        x0 = fmul(x0, x5);
        x3 = fadd(x3, x0);
        x0 = rdf(ea + 0x14);
        x0 = fmul(x0, x7);
        setf!(fr[22] = x3);
        setf!(fr[8] = x0);
        x3 = frf!(fr[8]);
        x0 = x2;
        x0 = fmul(x0, rdf(ea + 0x10));
        x2 = fmul(x2, rdf(ea + 0x20));
        x3 = fadd(x3, x0);
        x0 = rdf(ea + 0x18);
        x0 = fmul(x0, x1);
        x3 = fadd(x3, x0);
        x0 = rdf(ea + 0x28);
        x0 = fmul(x0, x1);
        x1 = rdf(m_b + 0x24);
        setf!(fr[8] = x3);
        x3 = rdf(ea + 0x24);
        x3 = fmul(x3, x7);
        x3 = fadd(x3, x2);
        x2 = rdf(m_b + 0x28);
        x3 = fadd(x3, x0);
        x0 = frf!(fr[8]);
        setf!(fr[73] = x0);
        x0 = x1;
        x0 = fmul(x0, x6);
        setf!(fr[36] = x3);
        setf!(fr[74] = x3);
        x3 = frf!(fr[22]);
        setf!(fr[72] = x3);
        x3 = rdf(m_b + 0x20);
        x7 = x3;
        x7 = fmul(x7, x4);
        x4 = rdf(ea + 0x24);
        x4 = fmul(x4, x1);
        x7 = fadd(x7, x0);
        x0 = x2;
        x0 = fmul(x0, x5);
        x5 = rdf(ea + 0x14);
        x5 = fmul(x5, x1);
        x7 = fadd(x7, x0);
        x0 = x3;
        x0 = fmul(x0, rdf(ea + 0x10));
        x3 = fmul(x3, rdf(ea + 0x20));
        x5 = fadd(x5, x0);
        x0 = rdf(ea + 0x18);
        x0 = fmul(x0, x2);
        x4 = fadd(x4, x3);
        setf!(fr[24] = x7);
        x5 = fadd(x5, x0);
        x0 = rdf(ea + 0x28);
        ea = fr[16];
        x0 = fmul(x0, x2);
        x1 = rdf(ea + 4);
        x3 = rdf(ea);
        x2 = rdf(ea + 8);
        x6 = rdf(ea + 0x10);
        x4 = fadd(x4, x0);
        x0 = x1;
        x0 = fmul(x0, rdf(m_b + 4));
        setf!(fr[28] = x5);
        setf!(fr[4] = x4);
        setf!(fr[78] = x4);
        x4 = rdf(ea + 0x14);
        setf!(fr[77] = x5);
        x5 = rdf(ea + 0x18);
        setf!(fr[76] = x7);
        setf!(fr[12] = x0);
        x7 = frf!(fr[12]);
        x0 = x3;
        x0 = fmul(x0, rdf(m_b));
        x7 = fadd(x7, x0);
        x0 = x2;
        x0 = fmul(x0, rdf(m_b + 8));
        x7 = fadd(x7, x0);
        x0 = x3;
        x0 = fmul(x0, rdf(m_b + 0x10));
        x3 = fmul(x3, rdf(m_b + 0x20));
        setf!(fr[12] = x7);
        x7 = rdf(m_b + 0x14);
        x7 = fmul(x7, x1);
        x7 = fadd(x7, x0);
        x0 = rdf(m_b + 0x18);
        x0 = fmul(x0, x2);
        x7 = fadd(x7, x0);
        x0 = rdf(m_b + 0x28);
        x0 = fmul(x0, x2);
        x2 = x4;
        x2 = fmul(x2, rdf(m_b + 4));
        setf!(fr[91] = x7);
        x7 = rdf(m_b + 0x24);
        x7 = fmul(x7, x1);
        x1 = rdf(ea + 0x28);
        x7 = fadd(x7, x3);
        x3 = rdf(m_b + 0x14);
        x7 = fadd(x7, x0);
        x0 = x6;
        x0 = fmul(x0, rdf(m_b));
        setf!(fr[92] = x7);
        x2 = fadd(x2, x0);
        x0 = x5;
        x0 = fmul(x0, rdf(m_b + 8));
        x7 = rdf(ea + 0x24);
        x3 = fmul(x3, x7);
        x2 = fadd(x2, x0);
        x0 = x6;
        x0 = fmul(x0, rdf(m_b + 0x10));
        setf!(fr[87] = x2);
        x2 = rdf(m_b + 0x14);
        x2 = fmul(x2, x4);
        x2 = fadd(x2, x0);
        x0 = rdf(m_b + 0x18);
        x0 = fmul(x0, x5);
        x2 = fadd(x2, x0);
        x0 = rdf(m_b + 0x20);
        x0 = fmul(x0, x6);
        setf!(fr[95] = x2);
        x2 = rdf(m_b + 0x24);
        x2 = fmul(x2, x4);
        x2 = fadd(x2, x0);
        x0 = rdf(m_b + 0x28);
        x0 = fmul(x0, x5);
        x5 = x7;
        x5 = fmul(x5, rdf(m_b + 4));
        x2 = fadd(x2, x0);
        setf!(fr[88] = x2);
        x2 = rdf(ea + 0x20);
        x0 = x2;
        x0 = fmul(x0, rdf(m_b));
        x5 = fadd(x5, x0);
        x0 = x1;
        x0 = fmul(x0, rdf(m_b + 8));
        x5 = fadd(x5, x0);
        x0 = x2;
        x0 = fmul(x0, rdf(m_b + 0x10));
        x2 = fmul(x2, rdf(m_b + 0x20));
        x3 = fadd(x3, x0);
        x0 = rdf(m_b + 0x18);
        x0 = fmul(x0, x1);
        setf!(fr[66] = x5);
        x3 = fadd(x3, x0);
        x0 = rdf(m_b + 0x28);
        x0 = fmul(x0, x1);
        setf!(fr[57] = x3);
        x3 = rdf(m_b + 0x24);
        x3 = fmul(x3, x7);
        x3 = fadd(x3, x2);
        x3 = fadd(x3, x0);
        setf!(fr[58] = x3);
        x3 = rdf(ea + 0x30);
        x1 = rdf(ea + 0x34);
        x3 = fsub(x3, rdf(m_b + 0x30));
        x1 = fsub(x1, rdf(m_b + 0x34));
        x2 = rdf(ea + 0x38);
        x2 = fsub(x2, rdf(m_b + 0x38));
        x4 = rdf(m_b + 0x14);
        let mut flag = g32(FLAG_BASE + 0x10);
        x4 = fmul(x4, x1);
        x0 = x3;
        x0 = fmul(x0, rdf(m_b));
        x5 = x1;
        x5 = fmul(x5, rdf(m_b + 4));
        x5 = fadd(x5, x0);
        x0 = x2;
        x0 = fmul(x0, rdf(m_b + 8));
        x5 = fadd(x5, x0);
        x0 = x3;
        x0 = fmul(x0, rdf(m_b + 0x10));
        x3 = fmul(x3, rdf(m_b + 0x20));
        x4 = fadd(x4, x0);
        x0 = rdf(m_b + 0x18);
        x0 = fmul(x0, x2);
        setf!(fr[51] = x5);
        x4 = fadd(x4, x0);
        x0 = rdf(m_b + 0x28);
        x0 = fmul(x0, x2);
        setf!(fr[42] = x4);
        x4 = rdf(m_b + 0x24);
        x4 = fmul(x4, x1);
        x4 = fadd(x4, x3);
        x4 = fadd(x4, x0);
        setf!(fr[32] = x4);
        if flag & 1 == 0 {
            x0 = gf(G_NPI);
            x1 = gf(G_NTHREE_Q);
            flag |= 1;
            unsafe { lf_checker_rt::global::<u32>(FLAG_BASE + 0x10).write_unaligned(flag) };
            unsafe { lf_checker_rt::global::<f32>(FLAG_BASE).write_unaligned(x0) };
            unsafe { lf_checker_rt::global::<f32>(FLAG_BASE + 4).write_unaligned(x0) };
            setf!(fr[40] = x1);
            unsafe { lf_checker_rt::global::<f32>(FLAG_BASE + 8).write_unaligned(x1) };
        } else {
            x0 = gf(FLAG_BASE + 8);
            setf!(fr[40] = x0);
            x0 = gf(FLAG_BASE);
        }
        setf!(fr[44] = x0);
        if flag & 2 == 0 {
            x4 = gf(G_PI);
            setf!(fr[52] = x4);
            unsafe { lf_checker_rt::global::<f32>(FLAG_BASE + 0x20).write_unaligned(x4) };
            unsafe { lf_checker_rt::global::<f32>(FLAG_BASE + 0x24).write_unaligned(x4) };
            x4 = gf(G_THREE_Q);
            flag |= 2;
            unsafe { lf_checker_rt::global::<u32>(FLAG_BASE + 0x10).write_unaligned(flag) };
            setf!(fr[30] = x4);
            unsafe { lf_checker_rt::global::<f32>(FLAG_BASE + 0x28).write_unaligned(x4) };
        } else {
            x0 = gf(FLAG_BASE + 0x28);
            setf!(fr[30] = x0);
            x0 = gf(FLAG_BASE + 0x20);
            setf!(fr[52] = x0);
        }
        x5 = frf!(fr[24]);
        x6 = frf!(fr[28]);
        x2 = frf!(fr[4]);
        x0 = x6;
        x3 = x5;
        x0 = fmul(x0, x6);
        x3 = fmul(x3, x5);
        x1 = 0.0;
        x3 = fadd(x3, x0);
        x0 = x2;
        x0 = fmul(x0, x2);
        x3 = fadd(x3, x0);
        if jne_f(x3, x1) {
            x0 = fsqrt(x3);
            x3 = gf(G_ONE);
            x3 = fdiv(x3, x0);
        } else {
            x3 = x1;
        }
        x2 = fmul(x2, x3);
        x6 = fmul(x6, x3);
        x5 = fmul(x5, x3);
        setf!(fr[4] = x2);
        x4 = x2;
        x4 = fmul(x4, frf!(fr[8]));
        x0 = x6;
        x0 = fmul(x0, frf!(fr[36]));
        x3 = x5;
        x3 = fmul(x3, frf!(fr[36]));
        x4 = fsub(x4, x0);
        x0 = x2;
        x0 = fmul(x0, frf!(fr[22]));
        x2 = x6;
        x2 = fmul(x2, frf!(fr[22]));
        x3 = fsub(x3, x0);
        x0 = x5;
        x0 = fmul(x0, frf!(fr[8]));
        x2 = fsub(x2, x0);
        x0 = x4;
        x0 = fmul(x0, x4);
        x7 = x3;
        x7 = fmul(x7, x3);
        x7 = fadd(x7, x0);
        x0 = x2;
        x0 = fmul(x0, x2);
        x7 = fadd(x7, x0);
        if jne_f(x7, x1) {
            x0 = fsqrt(x7);
            x7 = gf(G_ONE);
            x7 = fdiv(x7, x0);
        } else {
            x7 = x1;
        }
        x4 = fmul(x4, x7);
        x3 = fmul(x3, x7);
        x2 = fmul(x2, x7);
        x7 = x3;
        x0 = x4;
        x7 = fmul(x7, x5);
        x0 = fmul(x0, x6);
        x7 = fsub(x7, x0);
        setf!(fr[24] = x2);
        setf!(fr[22] = x7);
        if jne_f(x4, x1) || jne_f(x3, x1) {
            x2 = frf!(fr[24]);
            x1 = atan2_call(x3, x4);
        }
        x2 = fxor(x2, sign_mask);
        setf!(fr[20] = x1);
        x0 = shape_call();
        x1 = frf!(fr[4]);
        x2 = frf!(fr[22]);
        x3 = x0;
        x0 = 0.0;
        setf!(fr[24] = x3);
        if jne_f(x1, x0) || jne_f(x2, x0) {
            x3 = frf!(fr[24]);
            x0 = atan2_call(x2, x1);
        }
        x2 = frf!(fr[44]);
        setf!(fr[60] = x0);
        x1 = frf!(fr[20]);
        setf!(fr[61] = x3);
        setf!(fr[62] = x1);
        if x2 > x0 {
            x0 = x2;
            setf!(fr[60] = x0);
        }
        x2 = frf!(fr[52]);
        if x0 > x2 {
            setf!(fr[60] = x2);
        }
        x0 = gf(FLAG_BASE + 4);
        if x0 > x3 {
            x3 = x0;
            setf!(fr[61] = x3);
        }
        x0 = gf(FLAG_BASE + 0x24);
        if !(x3 > x0) {
            // keep
        } else {
            setf!(fr[61] = x0);
        }
        x0 = frf!(fr[40]);
        if x0 > x1 {
            x1 = x0;
            setf!(fr[62] = x1);
        }
        x0 = frf!(fr[30]);
        if !(x1 > x0) {
            // keep
        } else {
            setf!(fr[62] = x0);
        }
        lf_checker_rt::callee_thiscall!(M_STRUCT, u32, slot_ptr(fslot, 68), slot_ptr(fslot, 60));
        x0 = frf!(fr[68]);
        wrf(m_b + 0, x0);
        x0 = frf!(fr[69]);
        wrf(m_b + 4, x0);
        x0 = frf!(fr[70]);
        wrf(m_b + 8, x0);
        x0 = frf!(fr[72]);
        wrf(m_b + 0x10, x0);
        x0 = frf!(fr[73]);
        wrf(m_b + 0x14, x0);
        x0 = frf!(fr[74]);
        wrf(m_b + 0x18, x0);
        ea = fr[65];
        x6 = rdf(m_b + 4);
        x1 = rdf(m_b);
        x1 = fmul(x1, rdf(ea + 4));
        x3 = rdf(m_b + 8);
        x0 = x6;
        x0 = fmul(x0, rdf(ea + 0x14));
        x5 = rdf(ea + 0x28);
        x2 = rdf(m_b + 0x14);
        x1 = fadd(x1, x0);
        x0 = x3;
        x0 = fmul(x0, rdf(ea + 0x24));
        x7 = rdf(ea + 0x20);
        setf!(fr[24] = x6);
        x1 = fadd(x1, x0);
        x0 = rdf(ea + 0x18);
        x0 = fmul(x0, x6);
        x6 = rdf(m_b + 0x10);
        setf!(fr[65] = x1);
        x1 = rdf(ea + 8);
        x1 = fmul(x1, rdf(m_b));
        setf!(fr[4] = x6);
        x1 = fadd(x1, x0);
        x0 = x5;
        x0 = fmul(x0, x3);
        x1 = fadd(x1, x0);
        x0 = rdf(ea);
        x3 = x0;
        setf!(fr[22] = x0);
        x3 = fmul(x3, x6);
        x0 = x2;
        x0 = fmul(x0, rdf(ea + 0x10));
        setf!(fr[28] = x1);
        x1 = rdf(m_b + 0x18);
        x3 = fadd(x3, x0);
        x0 = x1;
        x0 = fmul(x0, x7);
        x3 = fadd(x3, x0);
        x0 = x2;
        x0 = fmul(x0, rdf(ea + 0x14));
        x2 = fmul(x2, rdf(ea + 0x18));
        setf!(fr[36] = x3);
        x3 = x6;
        x6 = x3;
        x6 = fmul(x6, rdf(ea + 4));
        x3 = fmul(x3, rdf(ea + 8));
        x6 = fadd(x6, x0);
        x0 = x1;
        x0 = fmul(x0, rdf(ea + 0x24));
        x3 = fadd(x3, x2);
        x2 = rdf(m_b + 0x24);
        x1 = fmul(x1, x5);
        x5 = rdf(m_b + 0x20);
        x4 = x5;
        x4 = fmul(x4, frf!(fr[22]));
        x3 = fadd(x3, x1);
        x1 = rdf(m_b + 0x28);
        x6 = fadd(x6, x0);
        setf!(fr[4] = x3);
        x0 = x2;
        x0 = fmul(x0, rdf(ea + 0x10));
        x3 = x5;
        x3 = fmul(x3, rdf(ea + 4));
        x5 = fmul(x5, rdf(ea + 8));
        x4 = fadd(x4, x0);
        x0 = x1;
        x0 = fmul(x0, x7);
        x7 = fmul(x7, rdf(m_b + 8));
        x4 = fadd(x4, x0);
        x0 = x2;
        x0 = fmul(x0, rdf(ea + 0x14));
        x2 = fmul(x2, rdf(ea + 0x18));
        x3 = fadd(x3, x0);
        x0 = x1;
        x0 = fmul(x0, rdf(ea + 0x24));
        x1 = fmul(x1, rdf(ea + 0x28));
        x3 = fadd(x3, x0);
        x0 = frf!(fr[22]);
        x0 = fmul(x0, rdf(m_b));
        x5 = fadd(x5, x2);
        x5 = fadd(x5, x1);
        x1 = rdf(ea + 0x10);
        x1 = fmul(x1, frf!(fr[24]));
        ea = fr[16];
        x0 = fadd(x0, x1);
        x0 = fadd(x0, x7);
        wrf(m_b, x0);
        x0 = frf!(fr[65]);
        wrf(m_b + 4, x0);
        x0 = frf!(fr[28]);
        wrf(m_b + 8, x0);
        x0 = frf!(fr[36]);
        wrf(m_b + 0x10, x0);
        x0 = frf!(fr[12]);
        wrf(m_b + 0x14, x6);
        x6 = frf!(fr[4]);
        wrf(m_b + 0x18, x6);
        wrf(m_b + 0x20, x4);
        wrf(m_b + 0x24, x3);
        wrf(m_b + 0x28, x5);
        wrf(ea, x0);
        x0 = frf!(fr[91]);
        wrf(ea + 4, x0);
        x0 = frf!(fr[92]);
        wrf(ea + 8, x0);
        x0 = frf!(fr[87]);
        wrf(ea + 0x10, x0);
        x0 = frf!(fr[95]);
        wrf(ea + 0x14, x0);
        x0 = frf!(fr[88]);
        wrf(ea + 0x18, x0);
        x0 = frf!(fr[66]);
        wrf(ea + 0x20, x0);
        x0 = frf!(fr[57]);
        wrf(ea + 0x24, x0);
        x0 = frf!(fr[58]);
        wrf(ea + 0x28, x0);
        x0 = frf!(fr[51]);
        wrf(ea + 0x30, x0);
        x0 = frf!(fr[42]);
        wrf(ea + 0x34, x0);
        x0 = frf!(fr[32]);
        wrf(ea + 0x38, x0);
        x4 = rdf(ea + 4);
        x3 = rdf(ea);
        x2 = rdf(ea + 8);
        x1 = x4;
        x1 = fmul(x1, rdf(m_b + 0x14));
        x0 = x3;
        x0 = fmul(x0, rdf(m_b + 4));
        setf!(fr[32] = x4);
        x1 = fadd(x1, x0);
        x0 = x2;
        x0 = fmul(x0, rdf(m_b + 0x24));
        x1 = fadd(x1, x0);
        x0 = x3;
        x0 = fmul(x0, rdf(m_b + 8));
        setf!(fr[42] = x1);
        x1 = x4;
        x1 = fmul(x1, rdf(m_b + 0x18));
        x1 = fadd(x1, x0);
        x0 = x2;
        x0 = fmul(x0, rdf(m_b + 0x28));
        x2 = rdf(ea + 0x10);
        x5 = rdf(ea + 0x34);
        x1 = fadd(x1, x0);
        x0 = rdf(ea + 0x14);
        x3 = x0;
        x3 = fmul(x3, rdf(m_b + 0x10));
        setf!(fr[8] = x0);
        x0 = x2;
        x0 = fmul(x0, rdf(m_b));
        setf!(fr[51] = x1);
        x1 = rdf(ea + 0x18);
        x3 = fadd(x3, x0);
        x0 = x1;
        x0 = fmul(x0, rdf(m_b + 0x20));
        x4 = x5;
        x4 = fmul(x4, rdf(m_b + 0x10));
        x3 = fadd(x3, x0);
        x0 = frf!(fr[8]);
        setf!(fr[58] = x3);
        x3 = x0;
        x3 = fmul(x3, rdf(m_b + 0x14));
        x0 = x2;
        x0 = fmul(x0, rdf(m_b + 4));
        x2 = fmul(x2, rdf(m_b + 8));
        x3 = fadd(x3, x0);
        x0 = x1;
        x0 = fmul(x0, rdf(m_b + 0x24));
        x1 = fmul(x1, rdf(m_b + 0x28));
        x3 = fadd(x3, x0);
        x0 = frf!(fr[8]);
        x0 = fmul(x0, rdf(m_b + 0x18));
        setf!(fr[57] = x3);
        x3 = rdf(ea + 0x24);
        x0 = fadd(x0, x2);
        x2 = rdf(ea + 0x28);
        x7 = x3;
        x7 = fmul(x7, rdf(m_b + 0x10));
        x0 = fadd(x0, x1);
        x1 = rdf(ea + 0x20);
        x6 = x3;
        x6 = fmul(x6, rdf(m_b + 0x14));
        setf!(fr[8] = x0);
        x3 = fmul(x3, rdf(m_b + 0x18));
        x0 = x1;
        x0 = fmul(x0, rdf(m_b));
        x7 = fadd(x7, x0);
        x0 = x2;
        x0 = fmul(x0, rdf(m_b + 0x20));
        x7 = fadd(x7, x0);
        x0 = x1;
        x0 = fmul(x0, rdf(m_b + 4));
        x1 = fmul(x1, rdf(m_b + 8));
        x6 = fadd(x6, x0);
        x0 = x2;
        x0 = fmul(x0, rdf(m_b + 0x24));
        x2 = fmul(x2, rdf(m_b + 0x28));
        x3 = fadd(x3, x1);
        x1 = rdf(ea + 0x30);
        x6 = fadd(x6, x0);
        x0 = x1;
        x0 = fmul(x0, rdf(m_b));
        x3 = fadd(x3, x2);
        x2 = rdf(ea + 0x38);
        x4 = fadd(x4, x0);
        x0 = x2;
        x0 = fmul(x0, rdf(m_b + 0x20));
        setf!(fr[66] = x3);
        x3 = x5;
        x3 = fmul(x3, rdf(m_b + 0x14));
        x4 = fadd(x4, x0);
        x0 = x1;
        x0 = fmul(x0, rdf(m_b + 4));
        x4 = fadd(x4, rdf(m_b + 0x30));
        x3 = fadd(x3, x0);
        x0 = x2;
        x0 = fmul(x0, rdf(m_b + 0x24));
        x3 = fadd(x3, x0);
        x3 = fadd(x3, rdf(m_b + 0x34));
        x1 = fmul(x1, rdf(m_b + 8));
        x5 = fmul(x5, rdf(m_b + 0x18));
        x0 = rdf(ea);
        x0 = fmul(x0, rdf(m_b));
        x2 = fmul(x2, rdf(m_b + 0x28));
        x5 = fadd(x5, x1);
        x1 = frf!(fr[32]);
        x1 = fmul(x1, rdf(m_b + 0x10));
        x5 = fadd(x5, x2);
        x1 = fadd(x1, x0);
        x0 = rdf(ea + 8);
        x0 = fmul(x0, rdf(m_b + 0x20));
        x5 = fadd(x5, rdf(m_b + 0x38));
        x1 = fadd(x1, x0);
        x0 = frf!(fr[42]);
        wrf(ea + 4, x0);
        x0 = frf!(fr[51]);
        wrf(ea + 8, x0);
        x0 = frf!(fr[58]);
        wrf(ea, x1);
        wrf(ea + 0x10, x0);
        x0 = frf!(fr[57]);
        wrf(ea + 0x14, x0);
        x0 = frf!(fr[8]);
        wrf(ea + 0x18, x0);
        x0 = frf!(fr[66]);
        wrf(ea + 0x20, x7);
        wrf(ea + 0x24, x6);
        wrf(ea + 0x28, x0);
        wrf(ea + 0x30, x4);
        wrf(ea + 0x34, x3);
        wrf(ea + 0x38, x5);

        // --- three late fetch pairs feed the second angle phase ---
        let fetch_d = v_fetch(table_obj(rd16s(sub + SUB_INDEX)), 0x17);
        let rec_d = lf_checker_rt::callee_thiscall!(M_RECORD, u32, sub, fetch_d);
        let fetch_e = v_fetch(table_obj(rd16s(sub + SUB_INDEX)), 0x16);
        fr[20] = rec_d;
        let rec_e = lf_checker_rt::callee_thiscall!(M_RECORD, u32, sub, fetch_e);
        let fetch_f = v_fetch(table_obj(rd16s(sub + SUB_INDEX)), 0x15);
        let m_late2 = rec_e;
        let rec_f = lf_checker_rt::callee_thiscall!(M_RECORD, u32, sub, fetch_f);
        let m_late3 = rec_f;
        x0 = rdf(m_late2 + 0x30);
        x3 = rdf(m_late2);
        x1 = rdf(m_late2 + 8);
        x7 = rdf(m_late2 + 0x14);
        setf!(fr[80] = x0);
        x0 = rdf(m_late2 + 0x34);
        setf!(fr[81] = x0);
        x0 = rdf(m_late2 + 0x38);
        setf!(fr[82] = x0);
        x6 = rdf(m_late3 + 4);
        x4 = rdf(m_late3);
        x0 = rdf(m_late2 + 4);
        x5 = rdf(m_late3 + 8);
        x3 = fmul(x3, x4);
        x0 = fmul(x0, x6);
        x3 = fadd(x3, x0);
        x2 = rdf(m_late3 + 0x14);
        x2 = fmul(x2, rdf(m_late2 + 4));
        x0 = x1;
        x0 = fmul(x0, x5);
        x3 = fadd(x3, x0);
        x0 = rdf(m_late2);
        x0 = fmul(x0, rdf(m_late3 + 0x10));
        setf!(fr[68] = x3);
        x2 = fadd(x2, x0);
        x0 = rdf(m_late3 + 0x18);
        x0 = fmul(x0, x1);
        x1 = rdf(m_late3 + 0x24);
        x1 = fmul(x1, rdf(m_late2 + 4));
        x2 = fadd(x2, x0);
        x0 = rdf(m_late2);
        x0 = fmul(x0, rdf(m_late3 + 0x20));
        setf!(fr[69] = x2);
        x2 = rdf(m_late2 + 0x10);
        x1 = fadd(x1, x0);
        x0 = rdf(m_late3 + 0x28);
        x0 = fmul(x0, rdf(m_late2 + 8));
        x3 = x2;
        x3 = fmul(x3, x4);
        x1 = fadd(x1, x0);
        x0 = x7;
        x0 = fmul(x0, x6);
        setf!(fr[70] = x1);
        x1 = rdf(m_late2 + 0x18);
        x3 = fadd(x3, x0);
        x0 = x1;
        x0 = fmul(x0, x5);
        x3 = fadd(x3, x0);
        x0 = x2;
        x0 = fmul(x0, rdf(m_late3 + 0x10));
        x2 = fmul(x2, rdf(m_late3 + 0x20));
        setf!(fr[16] = x3);
        x3 = rdf(m_late3 + 0x14);
        x3 = fmul(x3, x7);
        x3 = fadd(x3, x0);
        x0 = rdf(m_late3 + 0x18);
        x0 = fmul(x0, x1);
        x3 = fadd(x3, x0);
        x0 = rdf(m_late3 + 0x28);
        x0 = fmul(x0, x1);
        x1 = rdf(m_late2 + 0x24);
        setf!(fr[4] = x3);
        x3 = rdf(m_late3 + 0x24);
        x3 = fmul(x3, x7);
        x3 = fadd(x3, x2);
        x2 = rdf(m_late2 + 0x28);
        x3 = fadd(x3, x0);
        x0 = frf!(fr[16]);
        setf!(fr[72] = x0);
        x0 = frf!(fr[4]);
        setf!(fr[73] = x0);
        setf!(fr[51] = x3);
        setf!(fr[74] = x3);
        x3 = rdf(m_late2 + 0x20);
        x7 = x3;
        x7 = fmul(x7, x4);
        x4 = rdf(m_late3 + 0x24);
        x0 = x1;
        x0 = fmul(x0, x6);
        x4 = fmul(x4, x1);
        x7 = fadd(x7, x0);
        x0 = x2;
        x0 = fmul(x0, x5);
        x5 = rdf(m_late3 + 0x14);
        x5 = fmul(x5, x1);
        x7 = fadd(x7, x0);
        x0 = x3;
        x0 = fmul(x0, rdf(m_late3 + 0x10));
        x3 = fmul(x3, rdf(m_late3 + 0x20));
        x5 = fadd(x5, x0);
        x0 = rdf(m_late3 + 0x18);
        x0 = fmul(x0, x2);
        setf!(fr[32] = x7);
        x4 = fadd(x4, x3);
        x5 = fadd(x5, x0);
        x0 = rdf(m_late3 + 0x28);
        setf!(fr[42] = x5);
        x0 = fmul(x0, x2);
        ea = fr[20];
        x3 = rdf(m_late2 + 4);
        x4 = fadd(x4, x0);
        x6 = rdf(ea);
        x2 = rdf(m_late2 + 0x14);
        x0 = x6;
        x0 = fmul(x0, rdf(m_late2));
        setf!(fr[12] = x4);
        setf!(fr[78] = x4);
        x4 = rdf(ea + 4);
        x3 = fmul(x3, x4);
        setf!(fr[77] = x5);
        x5 = rdf(ea + 8);
        x3 = fadd(x3, x0);
        x0 = rdf(m_late2 + 8);
        x0 = fmul(x0, x5);
        x2 = fmul(x2, x4);
        x3 = fadd(x3, x0);
        x0 = x6;
        x0 = fmul(x0, rdf(m_late2 + 0x10));
        x6 = fmul(x6, rdf(m_late2 + 0x20));
        x2 = fadd(x2, x0);
        x0 = rdf(m_late2 + 0x18);
        x0 = fmul(x0, x5);
        x1 = fmul(x1, x4);
        x2 = fadd(x2, x0);
        x0 = rdf(m_late2 + 0x28);
        x4 = rdf(ea + 0x18);
        x0 = fmul(x0, x5);
        x5 = rdf(ea + 0x10);
        x1 = fadd(x1, x6);
        x6 = rdf(m_late2);
        setf!(fr[96] = x3);
        x3 = rdf(m_late2 + 4);
        setf!(fr[76] = x7);
        x7 = rdf(ea + 0x14);
        x1 = fadd(x1, x0);
        x3 = fmul(x3, x7);
        x0 = x6;
        x0 = fmul(x0, x5);
        setf!(fr[97] = x2);
        x2 = rdf(m_late2 + 0x14);
        x3 = fadd(x3, x0);
        x0 = rdf(m_late2 + 8);
        x0 = fmul(x0, x4);
        x2 = fmul(x2, x7);
        x3 = fadd(x3, x0);
        x0 = rdf(m_late2 + 0x10);
        x0 = fmul(x0, x5);
        setf!(fr[98] = x1);
        x1 = rdf(m_late2 + 0x24);
        x2 = fadd(x2, x0);
        x0 = rdf(m_late2 + 0x18);
        x0 = fmul(x0, x4);
        x1 = fmul(x1, x7);
        x2 = fadd(x2, x0);
        x0 = rdf(m_late2 + 0x20);
        x7 = rdf(m_late2 + 8);
        x0 = fmul(x0, x5);
        x5 = rdf(ea + 0x28);
        setf!(fr[100] = x3);
        x3 = rdf(m_late2 + 4);
        x1 = fadd(x1, x0);
        x0 = rdf(m_late2 + 0x28);
        x0 = fmul(x0, x4);
        x4 = rdf(ea + 0x24);
        x3 = fmul(x3, x4);
        x1 = fadd(x1, x0);
        x0 = x6;
        x6 = rdf(ea + 0x20);
        x0 = fmul(x0, x6);
        setf!(fr[101] = x2);
        x2 = rdf(m_late2 + 0x14);
        x3 = fadd(x3, x0);
        x0 = x7;
        x0 = fmul(x0, x5);
        setf!(fr[102] = x1);
        x2 = fmul(x2, x4);
        x3 = fadd(x3, x0);
        x0 = x6;
        x0 = fmul(x0, rdf(m_late2 + 0x10));
        x1 = rdf(m_late2 + 0x24);
        x1 = fmul(x1, x4);
        x2 = fadd(x2, x0);
        x0 = rdf(m_late2 + 0x18);
        x0 = fmul(x0, x5);
        x4 = rdf(ea + 0x38);
        x4 = fsub(x4, rdf(m_late2 + 0x38));
        x2 = fadd(x2, x0);
        x0 = rdf(m_late2 + 0x20);
        x0 = fmul(x0, x6);
        x6 = rdf(m_late2 + 4);
        setf!(fr[104] = x3);
        x3 = rdf(ea + 0x34);
        x3 = fsub(x3, rdf(m_late2 + 0x34));
        x1 = fadd(x1, x0);
        x0 = rdf(m_late2 + 0x28);
        x0 = fmul(x0, x5);
        x5 = rdf(ea + 0x30);
        x5 = fsub(x5, rdf(m_late2 + 0x30));
        x1 = fadd(x1, x0);
        x0 = rdf(m_late2);
        x6 = fmul(x6, x3);
        x0 = fmul(x0, x5);
        setf!(fr[105] = x2);
        x2 = rdf(m_late2 + 0x14);
        x6 = fadd(x6, x0);
        x2 = fmul(x2, x3);
        x0 = x5;
        x0 = fmul(x0, rdf(m_late2 + 0x10));
        setf!(fr[106] = x1);
        x1 = rdf(m_late2 + 0x24);
        x2 = fadd(x2, x0);
        x0 = rdf(m_late2 + 0x18);
        x0 = fmul(x0, x4);
        x1 = fmul(x1, x3);
        x2 = fadd(x2, x0);
        x0 = rdf(m_late2 + 0x20);
        x0 = fmul(x0, x5);
        x5 = frf!(fr[12]);
        x7 = fmul(x7, x4);
        x1 = fadd(x1, x0);
        x0 = rdf(m_late2 + 0x28);
        x0 = fmul(x0, x4);
        x4 = frf!(fr[32]);
        x6 = fadd(x6, x7);
        x1 = fadd(x1, x0);
        x3 = x4;
        x3 = fmul(x3, x4);
        setf!(fr[108] = x6);
        x6 = frf!(fr[42]);
        x0 = x6;
        x0 = fmul(x0, x6);
        setf!(fr[110] = x1);
        x1 = 0.0;
        x3 = fadd(x3, x0);
        x0 = x5;
        x0 = fmul(x0, x5);
        setf!(fr[109] = x2);
        x2 = gf(G_ONE);
        x3 = fadd(x3, x0);
        if jne_f(x3, x1) {
            x0 = fsqrt(x3);
            x3 = x2;
            x3 = fdiv(x3, x0);
        } else {
            x3 = x1;
        }
        x5 = fmul(x5, x3);
        x0 = x3;
        x0 = fmul(x0, x6);
        x7 = x3;
        x7 = fmul(x7, x4);
        x4 = frf!(fr[51]);
        setf!(fr[22] = x0);
        x6 = x5;
        x6 = fmul(x6, frf!(fr[4]));
        x0 = x4;
        x0 = fmul(x0, frf!(fr[22]));
        setf!(fr[12] = x5);
        x4 = fmul(x4, x7);
        x6 = fsub(x6, x0);
        x0 = x5;
        x5 = frf!(fr[16]);
        x0 = fmul(x0, x5);
        x5 = fmul(x5, frf!(fr[22]));
        x4 = fsub(x4, x0);
        x0 = frf!(fr[4]);
        x0 = fmul(x0, x7);
        setf!(fr[32] = x7);
        x5 = fsub(x5, x0);
        x0 = x6;
        x0 = fmul(x0, x6);
        x3 = x4;
        x3 = fmul(x3, x4);
        x3 = fadd(x3, x0);
        x0 = x5;
        x0 = fmul(x0, x5);
        x3 = fadd(x3, x0);
        if jne_f(x3, x1) {
            x0 = fsqrt(x3);
            x2 = fdiv(x2, x0);
        } else {
            x2 = x1;
        }
        x7 = x2;
        x7 = fmul(x7, x6);
        x6 = x2;
        x6 = fmul(x6, x4);
        x3 = x6;
        x3 = fmul(x3, frf!(fr[32]));
        x0 = x7;
        x0 = fmul(x0, frf!(fr[22]));
        x2 = fmul(x2, x5);
        x3 = fsub(x3, x0);
        setf!(fr[42] = x2);
        setf!(fr[16] = x3);
        if jne_f(x7, x1) || jne_f(x6, x1) {
            x2 = frf!(fr[42]);
            x1 = atan2_call(x6, x7);
        }
        x2 = fxor(x2, sign_mask);
        setf!(fr[22] = x1);
        x0 = shape_call();
        x3 = frf!(fr[12]);
        x1 = frf!(fr[16]);
        x2 = x0;
        x0 = 0.0;
        setf!(fr[32] = x2);
        if jne_f(x3, x0) || jne_f(x1, x0) {
            x2 = frf!(fr[32]);
            x0 = atan2_call(x1, x3);
        }
        x3 = gf(FLAG_BASE);
        x1 = frf!(fr[22]);
        setf!(fr[60] = x0);
        setf!(fr[61] = x2);
        setf!(fr[62] = x1);
        if x3 > x0 {
            x0 = x3;
            setf!(fr[60] = x0);
        }
        x3 = gf(FLAG_BASE + 0x20);
        if x0 > x3 {
            setf!(fr[60] = x3);
        }
        x0 = gf(FLAG_BASE + 4);
        if x0 > x2 {
            x2 = x0;
            setf!(fr[61] = x2);
        }
        x0 = gf(FLAG_BASE + 0x24);
        if x2 > x0 {
            setf!(fr[61] = x0);
        }
        x0 = gf(FLAG_BASE + 8);
        if x0 > x1 {
            x1 = x0;
            setf!(fr[62] = x1);
        }
        x0 = gf(FLAG_BASE + 0x28);
        if x1 > x0 {
            setf!(fr[62] = x0);
        }
        lf_checker_rt::callee_thiscall!(M_STRUCT, u32, slot_ptr(fslot, 68), slot_ptr(fslot, 60));
        x0 = frf!(fr[68]);
        wrf(m_late2, x0);
        x0 = frf!(fr[69]);
        wrf(m_late2 + 4, x0);
        x0 = frf!(fr[70]);
        wrf(m_late2 + 8, x0);
        x0 = frf!(fr[72]);
        wrf(m_late2 + 0x10, x0);
        x0 = frf!(fr[73]);
        wrf(m_late2 + 0x14, x0);
        x0 = frf!(fr[74]);
        wrf(m_late2 + 0x18, x0);
        x0 = frf!(fr[76]);
        wrf(m_late2 + 0x20, x0);
        x0 = frf!(fr[77]);
        wrf(m_late2 + 0x24, x0);
        x0 = frf!(fr[78]);
        wrf(m_late2 + 0x28, x0);
        x4 = rdf(m_late2 + 4);
        x1 = rdf(m_late2);
        x1 = fmul(x1, rdf(m_late3 + 4));
        x3 = rdf(m_late2 + 8);
        x0 = x4;
        x0 = fmul(x0, rdf(m_late3 + 0x14));
        x2 = rdf(m_late2 + 0x14);
        x7 = rdf(m_late3 + 0x20);
        x1 = fadd(x1, x0);
        x0 = x3;
        x0 = fmul(x0, rdf(m_late3 + 0x24));
        setf!(fr[42] = x4);
        x5 = rdf(m_late2 + 0x20);
        x1 = fadd(x1, x0);
        x0 = rdf(m_late3 + 0x18);
        x0 = fmul(x0, x4);
        setf!(fr[51] = x1);
        x1 = rdf(m_late3 + 8);
        x1 = fmul(x1, rdf(m_late2));
        x1 = fadd(x1, x0);
        x0 = rdf(m_late3 + 0x28);
        x0 = fmul(x0, x3);
        x3 = rdf(m_late3);
        x4 = x3;
        x1 = fadd(x1, x0);
        x0 = rdf(m_late2 + 0x10);
        x4 = fmul(x4, x0);
        setf!(fr[4] = x0);
        x0 = x2;
        x0 = fmul(x0, rdf(m_late3 + 0x10));
        setf!(fr[58] = x1);
        x1 = rdf(m_late2 + 0x18);
        x4 = fadd(x4, x0);
        x0 = x1;
        x0 = fmul(x0, x7);
        setf!(fr[32] = x3);
        x4 = fadd(x4, x0);
        x0 = x2;
        x0 = fmul(x0, rdf(m_late3 + 0x14));
        x2 = fmul(x2, rdf(m_late3 + 0x18));
        setf!(fr[57] = x4);
        x4 = frf!(fr[4]);
        x6 = x4;
        x4 = fmul(x4, rdf(m_late3 + 8));
        x6 = fmul(x6, rdf(m_late3 + 4));
        x4 = fadd(x4, x2);
        x2 = rdf(m_late2 + 0x24);
        x6 = fadd(x6, x0);
        x0 = x1;
        x1 = fmul(x1, rdf(m_late3 + 0x28));
        x0 = fmul(x0, rdf(m_late3 + 0x24));
        x4 = fadd(x4, x1);
        x1 = rdf(m_late2 + 0x28);
        x6 = fadd(x6, x0);
        x0 = x2;
        x0 = fmul(x0, rdf(m_late3 + 0x10));
        setf!(fr[4] = x4);
        x4 = x5;
        x4 = fmul(x4, x3);
        x4 = fadd(x4, x0);
        x0 = x1;
        x0 = fmul(x0, x7);
        x7 = fmul(x7, rdf(m_late2 + 8));
        x4 = fadd(x4, x0);
        x3 = x5;
        x3 = fmul(x3, rdf(m_late3 + 4));
        x5 = fmul(x5, rdf(m_late3 + 8));
        x0 = x2;
        x0 = fmul(x0, rdf(m_late3 + 0x14));
        x2 = fmul(x2, rdf(m_late3 + 0x18));
        x3 = fadd(x3, x0);
        x0 = x1;
        x0 = fmul(x0, rdf(m_late3 + 0x24));
        x1 = fmul(x1, rdf(m_late3 + 0x28));
        x3 = fadd(x3, x0);
        x0 = frf!(fr[32]);
        x0 = fmul(x0, rdf(m_late2));
        x5 = fadd(x5, x2);

        // --- two final calls, then return the error slot ---
        let run_ecx = fr[20];
        x5 = fadd(x5, x1);
        x1 = rdf(m_late3 + 0x10);
        x1 = fmul(x1, frf!(fr[42]));
        x0 = fadd(x0, x1);
        x0 = fadd(x0, x7);
        wrf(m_late2, x0);
        x0 = frf!(fr[51]);
        wrf(m_late2 + 4, x0);
        x0 = frf!(fr[58]);
        wrf(m_late2 + 8, x0);
        x0 = frf!(fr[57]);
        wrf(m_late2 + 0x10, x0);
        x0 = frf!(fr[4]);
        wrf(m_late2 + 0x14, x6);
        wrf(m_late2 + 0x18, x0);
        wrf(m_late2 + 0x20, x4);
        wrf(m_late2 + 0x24, x3);
        wrf(m_late2 + 0x28, x5);
        lf_checker_rt::callee_thiscall!(M_RUN_A, u32, run_ecx, slot_ptr(fslot, 96));
        lf_checker_rt::callee_thiscall!(M_RUN_B, u32, run_ecx, m_late2);
        frf!(fr[67])
    }
});
