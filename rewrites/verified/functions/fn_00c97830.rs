// original: 0x00c97830 ped_task_aim_pose_update (proposed)

/// Update an aiming pose from indexed sub-pose records, blending toward it.
///
/// `thiscall`: object in `ecx`, one unread stack word, callee pops 4.
/// Returns the last resolver-pair answer (an opaque callee word).
///
/// Layout read: `this+0x40` is the sub-object `S`; `S+0x2E` (signed word)
/// indexes the global object table; `S+0x20` points at three direction
/// floats (`+0x10/0x14/0x18`); `S+0x100` is the fallback holder used when
/// the slot-`0xA0` resolver answers null; `this+0xD8/0xDC` hold the two
/// blended outputs. Each fetched record `M` is twelve floats at offsets
/// `0x00..0x38` with gaps at `0x0C/0x1C/0x2C`.
///
/// Behaviour: six record fetches (virtual slot `0x38` of the table object
/// with a small constant id, then the record helper with that answer) seed
/// the frame; four root-mean-square distances over record triples are
/// blended with the direction floats; two bounded adjust calls (slot
/// `0xA0` resolve, `0xE0` fetch) may shift the result; two evaluator calls
/// consume frame windows; and four resolver-pair calls finish. All float
/// arithmetic is scalar SSE in the original's operand order.
///
/// Comparisons: `(an instruction of the original)` / `(an instruction of the original)` are zero-vs-nonzero checks on
/// callee answers (only the low byte matters for `al`); the loop counter
/// uses SIGNED `jl`/`jle` against a zero-extended word (so the loop is
/// skipped exactly when the bound is 0); the float branches are `comiss`
/// with `jbe`, taken when unordered, i.e. `!(x > y)`.
/// Edge cases: NaN, infinities and denormals flow through bit-exactly;
/// a null resolver answer selects the fallback holder; a zero loop bound
/// skips the loop; the unread stack word is ignored.
lf_checker_rt::export!(thiscall, rw_00c97830(this: u32, _unused: u32) -> u32 {
    unsafe {
        const SUB: u32 = 0x40;
        const SUB_INDEX: u32 = 0x2E;
        const SUB_DIR: u32 = 0x20;
        const SUB_ALT: u32 = 0x100;
        const TABLE: u32 = 0x01295CD8;
        const VT_FETCH: u32 = 0x38;
        const VT_RESOLVE: u32 = 0xA0;
        const VT_PAIR: u32 = 0xE0;
        const OUT_D8: u32 = 0xD8;
        const OUT_DC: u32 = 0xDC;
        const ENTRY_STRIDE: i32 = 0xE0;
        const M_RECORD: u32 = 1;
        const M_EVAL: u32 = 3;
        const M_ADJUST: u32 = 4;
        const M_PAIR_A: u32 = 5;
        const M_PAIR_B: u32 = 6;
        const M_COMBINE: u32 = 7;
        const G_TENTH: u32 = 0x00FE879C;
        const G_WINDOW: u32 = 0x00FE8874;
        const G_STEP: u32 = 0x01050B84;
        const G_ABS: u32 = 0x00FE8F80;
        const G_BLEND: u32 = 0x011735BC;
        const G_EIGHT: u32 = 0x01050B88;
        const G_ZERO: u32 = 0x00FE8628;
        const G_LIMIT: u32 = 0x00FE8734;
        const G_SIGN: u32 = 0x00FE8FA0;
        const G_BIAS: u32 = 0x0171BB5C;

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
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd16s(a: u32) -> i32 {
            unsafe { (a as *const u16).read_unaligned() as i16 as i32 }
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
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
        /// `comiss x, y` followed by `jbe`: taken unless x > y.
        #[inline(always)]
        fn jbe(x: f32, y: f32) -> bool {
            !(x > y)
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

        // Frame slots fr[i] mirror the original's aligned-frame dword at
        // ESP0+4*i (the original realigns esp and restores it via ebp; the
        // pushes inside call setups are folded into the indices below).
        let mut fr = [0u32; 104];
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

        let sub = rd32(this + SUB);
        let abs_mask = g32(G_ABS);
        let sign_mask = g32(G_SIGN);

        // --- record fetches 1..3 (ids 0xf, 0xb, 0xe) ---
        let r1 = v_fetch(table_obj(rd16s(sub + SUB_INDEX)), 0xf);
        let m1 = lf_checker_rt::callee_thiscall!(M_RECORD, u32, sub, r1);
        setf!(fr[72] = rdf(m1));
        setf!(fr[73] = rdf(m1 + 4));
        setf!(fr[74] = rdf(m1 + 8));
        setf!(fr[76] = rdf(m1 + 0x10));
        setf!(fr[77] = rdf(m1 + 0x14));
        setf!(fr[78] = rdf(m1 + 0x18));
        setf!(fr[80] = rdf(m1 + 0x20));
        setf!(fr[81] = rdf(m1 + 0x24));
        setf!(fr[82] = rdf(m1 + 0x28));
        setf!(fr[8] = rdf(m1 + 0x30));
        setf!(fr[84] = rdf(m1 + 0x30));
        setf!(fr[13] = rdf(m1 + 0x34));
        setf!(fr[85] = rdf(m1 + 0x34));
        let m1_38 = rdf(m1 + 0x38);
        let r2 = v_fetch(table_obj(rd16s(sub + SUB_INDEX)), 0xb);
        setf!(fr[14] = m1_38);
        let m2 = lf_checker_rt::callee_thiscall!(M_RECORD, u32, sub, r2);
        setf!(fr[88] = rdf(m2));
        setf!(fr[89] = rdf(m2 + 4));
        setf!(fr[90] = rdf(m2 + 8));
        setf!(fr[92] = rdf(m2 + 0x10));
        setf!(fr[93] = rdf(m2 + 0x14));
        setf!(fr[94] = rdf(m2 + 0x18));
        setf!(fr[96] = rdf(m2 + 0x20));
        setf!(fr[97] = rdf(m2 + 0x24));
        setf!(fr[98] = rdf(m2 + 0x28));
        setf!(fr[4] = rdf(m2 + 0x30));
        setf!(fr[100] = rdf(m2 + 0x30));
        setf!(fr[19] = rdf(m2 + 0x34));
        setf!(fr[101] = rdf(m2 + 0x34));
        let m2_38 = rdf(m2 + 0x38);
        let r3 = v_fetch(table_obj(rd16s(sub + SUB_INDEX)), 0xe);
        setf!(fr[16] = m2_38);
        let m3 = lf_checker_rt::callee_thiscall!(M_RECORD, u32, sub, r3);
        // --- fetches 4..6 (ids 0xa, 0xd, 9) ---
        let r4 = v_fetch(table_obj(rd16s(sub + SUB_INDEX)), 0xa);
        let m4 = lf_checker_rt::callee_thiscall!(M_RECORD, u32, sub, r4);
        let r5 = v_fetch(table_obj(rd16s(sub + SUB_INDEX)), 0xd);
        let m5 = lf_checker_rt::callee_thiscall!(M_RECORD, u32, sub, r5);
        setf!(fr[40] = rdf(m5));
        setf!(fr[41] = rdf(m5 + 4));
        setf!(fr[42] = rdf(m5 + 8));
        setf!(fr[44] = rdf(m5 + 0x10));
        setf!(fr[45] = rdf(m5 + 0x14));
        setf!(fr[46] = rdf(m5 + 0x18));
        setf!(fr[48] = rdf(m5 + 0x20));
        setf!(fr[49] = rdf(m5 + 0x24));
        setf!(fr[50] = rdf(m5 + 0x28));
        setf!(fr[52] = rdf(m5 + 0x30));
        setf!(fr[53] = rdf(m5 + 0x34));
        let m5_38 = rdf(m5 + 0x38);
        let r6 = v_fetch(table_obj(rd16s(sub + SUB_INDEX)), 9);
        setf!(fr[30] = m5_38);
        setf!(fr[54] = m5_38);
        let m6 = lf_checker_rt::callee_thiscall!(M_RECORD, u32, sub, r6);
        setf!(fr[56] = rdf(m6));
        setf!(fr[57] = rdf(m6 + 4));
        let mut x6 = rdf(m6 + 0x30);
        let mut x7 = rdf(m6 + 0x34);
        setf!(fr[58] = rdf(m6 + 8));
        setf!(fr[60] = rdf(m6 + 0x10));
        setf!(fr[61] = rdf(m6 + 0x14));
        setf!(fr[62] = rdf(m6 + 0x18));
        setf!(fr[64] = rdf(m6 + 0x20));
        setf!(fr[65] = rdf(m6 + 0x24));
        setf!(fr[66] = rdf(m6 + 0x28));
        let m6_38 = rdf(m6 + 0x38);
        setf!(fr[68] = x6);
        setf!(fr[69] = x7);
        setf!(fr[34] = m6_38);
        setf!(fr[70] = m6_38);

        // --- distances over record triples (esi=m3, edi=m4) ---
        let mut x3 = rdf(m3 + 0x30);
        let mut x2 = rdf(m3 + 0x34);
        let mut x4 = frf!(fr[52]);
        let mut x5 = frf!(fr[53]);
        let mut x1 = rdf(m3 + 0x38);
        let mut x0 = frf!(fr[30]);
        x0 = fsub(x0, x1);
        x5 = fsub(x5, x2);
        x4 = fsub(x4, x3);
        x2 = fsub(x2, frf!(fr[13]));
        x3 = fsub(x3, frf!(fr[8]));
        x0 = fmul(x0, x0);
        x5 = fmul(x5, x5);
        x4 = fmul(x4, x4);
        x1 = fsub(x1, frf!(fr[14]));
        x2 = fmul(x2, x2);
        x5 = fadd(x5, x4);
        x3 = fmul(x3, x3);
        x1 = fmul(x1, x1);
        x5 = fadd(x5, x0);
        x2 = fadd(x2, x3);
        x3 = rdf(m4 + 0x30);
        x6 = fsub(x6, x3);
        x3 = fsub(x3, frf!(fr[4]));
        x0 = fsqrt(x5);
        x2 = fadd(x2, x1);
        setf!(fr[37] = x0);
        x1 = rdf(m4 + 0x38);
        x6 = fmul(x6, x6);
        x0 = fsqrt(x2);
        x2 = rdf(m4 + 0x34);
        x7 = fsub(x7, x2);
        setf!(fr[36] = x0);
        x0 = frf!(fr[34]);
        x0 = fsub(x0, x1);
        x2 = fsub(x2, frf!(fr[19]));
        x7 = fmul(x7, x7);
        x1 = fsub(x1, frf!(fr[16]));
        x0 = fmul(x0, x0);
        x7 = fadd(x7, x6);
        x2 = fmul(x2, x2);
        x3 = fmul(x3, x3);
        x7 = fadd(x7, x0);
        x1 = fmul(x1, x1);
        x2 = fadd(x2, x3);
        x0 = fsqrt(x7);
        x2 = fadd(x2, x1);
        setf!(fr[38] = x0);
        let vec = rd32(sub + SUB_DIR);
        x3 = gf(G_TENTH);
        x1 = rdf(vec + 0x18);
        x0 = fsqrt(x2);
        x2 = rdf(vec + 0x10);
        setf!(fr[39] = x0);
        x0 = rdf(vec + 0x14);
        x1 = fmul(x1, x3);
        x0 = fmul(x0, x3);
        x1 = fadd(x1, frf!(fr[14]));
        x2 = fmul(x2, x3);
        x0 = fadd(x0, frf!(fr[13]));
        fr[33] = vec;
        x2 = fadd(x2, frf!(fr[8]));
        setf!(fr[26] = x1);
        x1 = rdf(vec + 0x14);
        setf!(fr[25] = x0);
        x0 = rdf(vec + 0x10);
        x1 = fmul(x1, x3);
        setf!(fr[24] = x2);
        x2 = rdf(vec + 0x18);
        x1 = fadd(x1, frf!(fr[19]));
        x0 = fmul(x0, x3);
        x2 = fmul(x2, x3);
        x0 = fadd(x0, frf!(fr[4]));
        setf!(fr[21] = x1);
        x2 = fadd(x2, frf!(fr[16]));
        x1 = 0.0;
        setf!(fr[20] = x0);
        x0 = x1;
        setf!(fr[22] = x2);
        setf!(fr[8] = x0);

        // --- evaluator #1 over frame windows ---
        let e1 = lf_checker_rt::callee_thiscall!(M_EVAL, u32, this,
            slot_ptr(fslot, 24), slot_ptr(fslot, 19), 0u32);
        setf!(fr[4] = x1);
        if (e1 & 0xFF) != 0 {
            x1 = frf!(fr[14]);
            x2 = frf!(fr[19]);
            x3 = gf(G_WINDOW);
            x0 = x1;
            x0 = fsub(x0, x2);
            x0 = fabs(x0, abs_mask);
            if !jbe(x3, x0) {
                x2 = fsub(x2, gf(G_STEP));
                x0 = x2;
                x0 = fsub(x0, x1);
                setf!(fr[14] = x2);
                setf!(fr[8] = x0);
            }
        }

        // --- evaluator #2 over frame windows ---
        let e2 = lf_checker_rt::callee_thiscall!(M_EVAL, u32, this,
            slot_ptr(fslot, 20), slot_ptr(fslot, 19), 0u32);
        if (e2 & 0xFF) != 0 {
            x1 = frf!(fr[16]);
            x3 = frf!(fr[19]);
            x2 = gf(G_WINDOW);
            x0 = x1;
            x0 = fsub(x0, x3);
            x0 = fabs(x0, abs_mask);
            if jbe(x2, x0) {
                x2 = frf!(fr[4]);
            } else {
                x3 = fsub(x3, gf(G_STEP));
                setf!(fr[16] = x3);
                x2 = x3;
                x2 = fsub(x2, x1);
            }
        } else {
            x2 = frf!(fr[4]);
        }

        // --- blend toward the stored outputs ---
        x1 = rdf(this + OUT_DC);
        x0 = gf(G_BLEND);
        x0 = fmul(x0, gf(G_EIGHT));
        x3 = frf!(fr[8]);
        x3 = fsub(x3, x1);
        x3 = fmul(x3, x0);
        x3 = fadd(x3, x1);
        x1 = rdf(this + OUT_D8);
        x2 = fsub(x2, x1);
        wrf(this + OUT_DC, x3);
        x0 = gf(G_BLEND);
        x0 = fmul(x0, gf(G_EIGHT));
        x2 = fmul(x2, x0);
        x0 = x3;
        setf!(fr[13] = x0);
        x2 = fadd(x2, x1);
        setf!(fr[4] = x2);
        wrf(this + OUT_D8, x2);
        if jbe(x0, x2) {
            setf!(fr[8] = x2);
        } else {
            x2 = x0;
            setf!(fr[8] = x0);
        }

        // --- first resolver block with the bounded accumulation loop ---
        if !jbe(x2, gf(G_ZERO)) {
            let holder = if v_resolve(sub) == 0 {
                rd32(sub + SUB_ALT)
            } else {
                v_pair(v_resolve(sub))
            };
            let grp = rd32(holder + 4);
            let bound = rd16(grp + 0x14) as i32;
            if bound > 0 {
                let mut idx: i32 = 0;
                x1 = frf!(fr[8]);
                loop {
                    let rec = lf_checker_rt::callee_thiscall!(M_RECORD, u32, sub, idx as u32);
                    x0 = rdf(rec + 0x38);
                    x0 = fadd(x0, x1);
                    idx += 1;
                    wrf(rec + 0x38, x0);
                    if !(idx < bound) {
                        break;
                    }
                }
            } else {
                x1 = frf!(fr[8]);
            }
            x0 = frf!(fr[13]);
            x0 = fsub(x0, x1);
            setf!(fr[13] = x0);
            x0 = frf!(fr[4]);
            x0 = fsub(x0, x1);
            setf!(fr[4] = x0);
            x0 = frf!(fr[14]);
            x0 = fadd(x0, x1);
            setf!(fr[14] = x0);
            x0 = frf!(fr[16]);
            x0 = fadd(x0, x1);
            setf!(fr[16] = x0);
            x0 = frf!(fr[30]);
            x0 = fadd(x0, x1);
            setf!(fr[54] = x0);
            x0 = frf!(fr[34]);
            x0 = fadd(x0, x1);
            setf!(fr[70] = x0);
        }

        // --- records 7..8 accumulate into their own tails ---
        let r7 = v_fetch(table_obj(rd16s(sub + SUB_INDEX)), 0xf);
        let m7 = lf_checker_rt::callee_thiscall!(M_RECORD, u32, sub, r7);
        x0 = rdf(m7 + 0x38);
        x0 = fadd(x0, frf!(fr[13]));
        let r8 = v_fetch(table_obj(rd16s(sub + SUB_INDEX)), 0xb);
        wrf(m7 + 0x38, x0);
        let m8 = lf_checker_rt::callee_thiscall!(M_RECORD, u32, sub, r8);
        x0 = frf!(fr[4]);
        x0 = fadd(x0, rdf(m8 + 0x38));
        fr[20] = 0;
        fr[21] = 0;
        wrf(m8 + 0x38, x0);

        // --- fetch 9 feeds the second resolver block ---
        let r9 = v_fetch(table_obj(rd16s(sub + SUB_INDEX)), 0xf);
        setf!(fr[22] = frf!(fr[13]));
        let holder2 = if v_resolve(sub) == 0 {
            rd32(sub + SUB_ALT)
        } else {
            v_pair(v_resolve(sub))
        };
        let grp2 = rd32(holder2 + 4);
        let arr2 = rd32(grp2);
        let w2 = rd16(arr2.wrapping_add((r9 as i32).wrapping_mul(ENTRY_STRIDE) as u32).wrapping_add(0x14));
        lf_checker_rt::callee_thiscall!(M_ADJUST, u32, this, w2, slot_ptr(fslot, 20));

        // --- fetch 10 feeds the third resolver block ---
        let r10 = v_fetch(table_obj(rd16s(sub + SUB_INDEX)), 0xb);
        fr[20] = 0;
        fr[21] = 0;
        setf!(fr[22] = frf!(fr[4]));
        let holder3 = if v_resolve(sub) == 0 {
            rd32(sub + SUB_ALT)
        } else {
            v_pair(v_resolve(sub))
        };
        let grp3 = rd32(holder3 + 4);
        let arr3 = rd32(grp3);
        let w3 = rd16(arr3.wrapping_add((r10 as i32).wrapping_mul(ENTRY_STRIDE) as u32).wrapping_add(0x14));
        let mut ans = lf_checker_rt::callee_thiscall!(M_ADJUST, u32, this, w3, slot_ptr(fslot, 20));

        // --- second window check guards the first pair phase ---
        x1 = frf!(fr[13]);
        x0 = frf!(fr[14]);
        x2 = frf!(fr[16]);
        let saved_vec = fr[33];
        x0 = fadd(x0, x1);
        x1 = fabs(x1, abs_mask);
        setf!(fr[86] = x0);
        x0 = frf!(fr[4]);
        x2 = fadd(x2, x0);
        setf!(fr[102] = x2);
        x2 = gf(G_LIMIT);
        if !jbe(x1, x2) {
            x0 = rdf(saved_vec);
            x0 = fxor(x0, sign_mask);
            setf!(fr[20] = x0);
            x0 = rdf(saved_vec + 4);
            x0 = fxor(x0, sign_mask);
            setf!(fr[21] = x0);
            x0 = rdf(saved_vec + 8);
            x0 = fxor(x0, sign_mask);
            x1 = gf(G_BIAS);
            setf!(fr[22] = x0);
            x0 = x1;
            x0 = fadd(x0, frf!(fr[36]));
            x1 = fadd(x1, frf!(fr[37]));
            lf_checker_rt::callee_thiscall!(M_PAIR_A, u32, this,
                slot_ptr(fslot, 24), slot_ptr(fslot, 84), slot_ptr(fslot, 52),
                x1.to_bits(), x0.to_bits(), slot_ptr(fslot, 20));

            // --- record 11 stores the evaluator window back ---
            let r11 = v_fetch(table_obj(rd16s(sub + SUB_INDEX)), 0xe);
            let m11 = lf_checker_rt::callee_thiscall!(M_RECORD, u32, sub, r11);
            wrf(m11 + 0x30, frf!(fr[24]));
            wrf(m11 + 0x34, frf!(fr[25]));
            wrf(m11 + 0x38, frf!(fr[26]));
            wrf(m11 + 0x3c, frf!(fr[27]));

            // --- resolver pairs 1..2 ---
            let s1 = v_fetch(table_obj(rd16s(sub + SUB_INDEX)), 0xe);
            let t1 = v_fetch(table_obj(rd16s(sub + SUB_INDEX)), 0xd);
            lf_checker_rt::callee_thiscall!(M_COMBINE, u32, this, t1, s1);
            let s2 = v_fetch(table_obj(rd16s(sub + SUB_INDEX)), 0xf);
            let t2 = v_fetch(table_obj(rd16s(sub + SUB_INDEX)), 0xe);
            ans = lf_checker_rt::callee_thiscall!(M_COMBINE, u32, this, t2, s2);
        }

        // --- third window check guards the second pair phase (both paths) ---
        x0 = frf!(fr[4]);
        x2 = gf(G_LIMIT);
        x0 = fabs(x0, abs_mask);
        if !jbe(x0, x2) {
            x1 = gf(G_BIAS);
            x0 = x1;
            x0 = fadd(x0, frf!(fr[39]));
            x1 = fadd(x1, frf!(fr[38]));
            lf_checker_rt::callee_thiscall!(M_PAIR_B, u32, this,
                slot_ptr(fslot, 24), slot_ptr(fslot, 100), slot_ptr(fslot, 68),
                x1.to_bits(), x0.to_bits(), saved_vec);

            // --- record 12 stores the evaluator window back ---
            let r12 = v_fetch(table_obj(rd16s(sub + SUB_INDEX)), 0xa);
            let m12 = lf_checker_rt::callee_thiscall!(M_RECORD, u32, sub, r12);
            wrf(m12 + 0x30, frf!(fr[24]));
            wrf(m12 + 0x34, frf!(fr[25]));
            wrf(m12 + 0x38, frf!(fr[26]));
            wrf(m12 + 0x3c, frf!(fr[27]));

            // --- resolver pairs 3..4 ---
            let s3 = v_fetch(table_obj(rd16s(sub + SUB_INDEX)), 0xa);
            let t3 = v_fetch(table_obj(rd16s(sub + SUB_INDEX)), 9);
            lf_checker_rt::callee_thiscall!(M_COMBINE, u32, this, t3, s3);
            let s4 = v_fetch(table_obj(rd16s(sub + SUB_INDEX)), 0xb);
            let t4 = v_fetch(table_obj(rd16s(sub + SUB_INDEX)), 0xa);
            ans = lf_checker_rt::callee_thiscall!(M_COMBINE, u32, this, t4, s4);
        }
        ans
    }
});
