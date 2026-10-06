// original: 0x00bf7a90 fire_task_dispatch (proposed)

/// Dispatch a fire task for a subject: validate the task id, resolve a target
/// through the task registry, run the per-flag setup calls, optionally place
/// it, roll a ranged random value, and submit one 16-word job to the worker
/// queue.
///
/// `this` (ECX) is the task object: id dword at `+0x08`, a float parameter at
/// `+0x18`, flag byte at `+0x2c` (bit 0 = vtable setup, bit 1 = placement,
/// bit 2 = full submit). `subject` (arg1) is the subject: vtable pointer at
/// `+0x00`, placement struct at `+0x10`, placement pointer at `+0x20`.
/// `param` (arg2) is a float compared against +0.0.
///
/// Stages: the id must equal one of three registry lookups (compared with
/// equality only, so the count of lookup calls varies from one to three);
/// a triple is fetched into scratch; the registry resolves a target object
/// (null aborts) and reports an out-flag byte. When `param` is exactly +0.0
/// (both zeros; the original tests this with a parity trick on the compare
/// flags, taken for every nonzero value and NaN) a strength value attaches;
/// otherwise a polled helper may run. The vtable slot at `+0xec` of the
/// subject's table yields a vector triple copied to the target at `+0x190`.
/// With placement, the placement pointer is ensured, registered and attached;
/// when the out-flag byte is set a sequencer runs and stores a counter,
/// bumped when it equals the poll answer, at target `+0x1d4`. Without the
/// full-submit flag the function returns the last answer with its low byte
/// replaced by the flags (the null-subject exit returns entry EAX instead).
/// Otherwise the scratch triple is placed: copied directly, or transformed by
/// the placement matrix (original multiply/add order, bit-exact), with a
/// carry word from the vtable scratch; a multiplicative generator over a
/// global pair is scaled to range; and the 16-word job (consts, the four
/// outputs, the rolled or constant value, one global word) is submitted and
/// its answer returned. All integer compares in the function are equality
/// (against zero, another value, or a callee answer); there is no
/// signed/unsigned-sensitive comparison.
///
/// Original: 0x00bf7a90 (thiscall, two stack words: subject pointer, float).
/// Frame scratch below the call is compared only where snapshotted; stack
/// addresses passed to callees are skipped with snapshots in the contract.
lf_checker_rt::export!(thiscall, rw_00bf7a90(esi: u32, edi: u32, farg: u32) -> u32 {
    unsafe {
        const FIRE_VEHICLE: u32 = 0x00EBC534;
        const FIRE_OBJECT: u32 = 0x00EBC544;
        const FIRE_MAP: u32 = 0x00EBC550;
        const STRENGTH: u32 = 0x00EBC55C;
        const TASK_REGISTRY: u32 = 0x01394D60;
        const RNG_MULT: u32 = 0x5CDCFAA7;
        const MANTISSA_MASK: u32 = 0x007FFFFF;
        const JOB_TAG: u32 = 0x201;

        const TASK_ID: u32 = 0x08;
        const TASK_FLOAT: u32 = 0x18;
        const TASK_FLAGS: u32 = 0x2C;
        const FLAG_HAS_VTABLE_SETUP: u8 = 0x01;
        const FLAG_HAS_PLACEMENT: u8 = 0x02;
        const FLAG_FULL_SUBMIT: u8 = 0x04;
        const SUBJ_VTABLE: u32 = 0x00;
        const SUBJ_PLACEMENT: u32 = 0x10;
        const SUBJ_PLACE_PTR: u32 = 0x20;
        const VTASK_SLOT: u32 = 0xEC;
        const TARGET_VEC: u32 = 0x190;
        const TARGET_SEQ: u32 = 0x1D4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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
        unsafe fn g32(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }

        let mut eax_var: u32;
        if edi == 0 {
            return 0;
        }
        // (eax_var assigned at the registry resolve below on all live paths)
        // Id gates. All compares are equality (je); no signedness.
        let id8 = rd32(esi + TASK_ID);
        let a1: u32 = lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(FIRE_VEHICLE), 0);
        if id8 != a1 {
            let a2: u32 = lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(FIRE_OBJECT), 0);
            if id8 != a2 {
                let _: u32 = lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(FIRE_MAP), 0);
            }
        }
        // Triple fetch into scratch; triple lands at buf[12..15].
        let mut buf60 = [0u32; 16];
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, esi, buf60.as_mut_ptr() as u32);
        // Registry resolve; flagbuf[0] is the out-flag byte (pre-state 0).
        let mut flagbuf = [0u8; 4];
        let edx: u32 = lf_checker_rt::callee_thiscall!(
            3, u32, lf_checker_rt::relocated(TASK_REGISTRY),
            edi.wrapping_add(2), id8, flagbuf.as_mut_ptr() as u32, 0, 0
        );
        eax_var = edx;
        if edx == 0 {
            return eax_var;
        }
        let flags = rd8(esi + TASK_FLAGS);
        // Float gate. ucomiss(xmm0,+0.0); lahf; (an instruction of the original); jp is a parity
        // trick: jp <=> parity(AH&0x44) even <=> ZF==PF <=> the comparison
        // was NOT ordered-equal <=> f != +0.0 (nonzero or NaN). Rust
        // `f == 0.0` matches exactly (+0.0 and -0.0 equal; NaN unequal).
        // The lahf residue in AH is clobbered on every path before any exit.
        let f = f32::from_bits(farg);
        if f != 0.0 {
            if flagbuf[0] == 0 {
                let a6: u32 = lf_checker_rt::callee_thiscall!(6, u32, esi);
                eax_var = a6;
                if (a6 & 0xFF) != 0 {
                    let mut wbuf = [0u32; 2];
                    let _: u32 = lf_checker_rt::callee_cdecl!(7, u32, wbuf.as_mut_ptr() as u32, esi);
                    let a8: u32 = lf_checker_rt::callee_thiscall!(8, u32, esi, edx, wbuf[0], farg);
                    eax_var = a8;
                }
            }
        } else {
            if flags & FLAG_HAS_PLACEMENT == 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, edx, buf60.as_mut_ptr() as u32);
            }
            // (zero path: strength attach below)
            let f18 = rdf(esi + TASK_FLOAT);
            let a5: u32 =
                lf_checker_rt::callee_thiscall!(5, u32, edx, lf_checker_rt::relocated(STRENGTH), f18.to_bits());
            eax_var = a5;
        }
        // Vtable setup through the subject's table; answer points at the
        // vector triple copied to the target. vtbuf[3] doubles as the
        // mat3 source below (zeroed = frame fill when no call ran).
        let mut vtbuf = [0u32; 16];
        if flags & FLAG_HAS_VTABLE_SETUP != 0 {
            let slot: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(edi + SUBJ_VTABLE) + VTASK_SLOT) as usize);
            let va = slot(edi, vtbuf.as_mut_ptr() as u32);
            let w0 = rd32(va);
            let w1 = rd32(va + 4);
            let w2 = rd32(va + 8);
            wr32(edx + TARGET_VEC, w0);
            wr32(edx + 0x194, w1);
            wr32(edx + 0x198, w2);
            eax_var = w2;
        }
        // Placement ensure + register + attach.
        if flags & FLAG_HAS_PLACEMENT != 0 {
            if rd32(edi + SUBJ_PLACE_PTR) == 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(10, u32, edi);
                let e = rd32(edi + SUBJ_PLACE_PTR);
                let _: u32 = lf_checker_rt::callee_thiscall!(11, u32, edi.wrapping_add(SUBJ_PLACEMENT), e);
            }
            let e2 = rd32(edi + SUBJ_PLACE_PTR);
            let _: u32 = lf_checker_rt::callee_thiscall!(12, u32, edx, e2);
            let a13: u32 = lf_checker_rt::callee_thiscall!(13, u32, lf_checker_rt::relocated(TASK_REGISTRY), edx, edi, 0);
            eax_var = a13;
        }
        // Sequencer. Equality compare only (cmp, jne).
        if flagbuf[0] != 0 {
            if flags & FLAG_HAS_PLACEMENT != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(14, u32, edx, buf60.as_mut_ptr() as u32);
            }
            let _: u32 = lf_checker_rt::callee_thiscall!(15, u32, edx);
            // id 16 takes no compared ECX: the original's ECX here is the
            // exit residue of id 15, which really clobbers ECX (nested calls),
            // so under stubbing that residue is unknowable; contract cdecl/0.
            let a16: u32 = lf_checker_rt::callee_cdecl!(16, u32,);
            let mut seq = g32(0x011F70C4);
            if g32(0x011F702C) == a16 {
                seq = seq.wrapping_add(1);
            }
            wr32(edx + TARGET_SEQ, seq);
            eax_var = edx;
        }
        // Submit gate.
        if flags & FLAG_FULL_SUBMIT == 0 {
            return (eax_var & 0xFFFFFF00) | (flags as u32);
        }
        // Placement triple. The fourth coeff ([esp+0x9c]) is dead on the
        // matrix path (loaded, then xmm0 is clobbered and the slot remade).
        let (m0, m1, m2, m3);
        if flags & FLAG_HAS_PLACEMENT == 0 {
            m0 = f32::from_bits(buf60[12]);
            m1 = f32::from_bits(buf60[13]);
            m2 = f32::from_bits(buf60[14]);
            m3 = f32::from_bits(buf60[15]);
        } else {
            let c4 = f32::from_bits(buf60[12]);
            let c5 = f32::from_bits(buf60[13]);
            let c6 = f32::from_bits(buf60[14]);
            if rd32(edi + SUBJ_PLACE_PTR) == 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(10, u32, edi);
                let e = rd32(edi + SUBJ_PLACE_PTR);
                let _: u32 = lf_checker_rt::callee_thiscall!(11, u32, edi.wrapping_add(SUBJ_PLACEMENT), e);
            }
            let mp = rd32(edi + SUBJ_PLACE_PTR);
            // Original operand order throughout (bit-exact, NaN-preserving).
            let mut x0 = rdf(mp);
            let mut x3 = rdf(mp + 0x10);
            let mut x2 = rdf(mp + 0x14);
            x0 = mul(x0, c4);
            let mut x1 = rdf(mp + 0x18);
            x3 = mul(x3, c5);
            x2 = mul(x2, c5);
            x3 = add(x3, x0);
            x0 = rdf(mp + 0x20);
            x0 = mul(x0, c6);
            x1 = mul(x1, c5);
            x3 = add(x3, x0);
            x0 = rdf(mp + 4);
            x0 = mul(x0, c4);
            x3 = add(x3, rdf(mp + 0x30));
            x2 = add(x2, x0);
            x0 = rdf(mp + 0x24);
            x0 = mul(x0, c6);
            x2 = add(x2, x0);
            x0 = rdf(mp + 8);
            x0 = mul(x0, c4);
            x2 = add(x2, rdf(mp + 0x34));
            x1 = add(x1, x0);
            x0 = rdf(mp + 0x28);
            x0 = mul(x0, c6);
            x1 = add(x1, x0);
            let x0b = f32::from_bits(vtbuf[3]);
            x1 = add(x1, rdf(mp + 0x38));
            m0 = x3;
            m1 = x2;
            m2 = x1;
            m3 = x0b;
        }
        // Multiplicative RNG over the global pair, scaled to range.
        let ra = rd32(lf_checker_rt::global::<u32>(0x011101A0) as u32);
        let rb = rd32(lf_checker_rt::global::<u32>(0x011101A4) as u32);
        let t = (ra as u64)
            .wrapping_mul(RNG_MULT as u64)
            .wrapping_add(rb as u64);
        let na = t as u32;
        let nd = (t >> 32) as u32;
        wr32(lf_checker_rt::global::<u32>(0x011101A0) as u32, na);
        let mut f2 = (na & MANTISSA_MASK) as f32;
        f2 = mul(f2, f32::from_bits(g32(0x00FE864C)));
        let mut f0 = f32::from_bits(g32(0x00EE19E0));
        f0 = sub(f0, f32::from_bits(g32(0x00EE19DC)));
        f2 = mul(f2, f0);
        f2 = add(f2, f32::from_bits(g32(0x00EE19DC)));
        wr32(lf_checker_rt::global::<u32>(0x011101A4) as u32, nd);
        let rng = f2;
        // Pick const or rolled value, submit the 16-word job.
        let a17: u32 = lf_checker_rt::callee_cdecl!(17, u32,);
        let x1: f32;
        if (a17 & 0xFF) != 0 {
            x1 = f32::from_bits(g32(0x00EE19DC));
        } else {
            let a18: u32 = lf_checker_rt::callee_cdecl!(18, u32,);
            if (a18 & 0xFF) != 0 {
                x1 = f32::from_bits(g32(0x00EE19DC));
            } else {
                x1 = rng;
            }
        }
        // Job structs mirror the original's frame layout at the mega-call:
        // arg3 -> [S+0x30] (zero/one consts + the [S+0x3c] carry word, with
        // the RNG word visible 8 bytes below it), arg5 -> [S+0x10] (the four
        // placement outputs + two fill words). All values deterministic.
        let mutblk_a: [u32; 10] = [
            0, rng.to_bits(), 0, 0, 0x3F800000, vtbuf[3], 0, 0x3F800000, 0, 0,
        ];
        let blk_b: [u32; 4] = [0, 0x3F800000, 0, 0];
        let blk_c: [u32; 6] = [
            m0.to_bits(), m1.to_bits(), m2.to_bits(), m3.to_bits(), 0, 0,
        ];
        let blk_d: [u32; 4] = [g32(0x00EE19D0), g32(0x00EE19D4), g32(0x00EE19D8), 0];
        let ans: u32 = lf_checker_rt::callee_cdecl!(
            19, u32, 0, 0, JOB_TAG,
            mutblk_a.as_ptr().wrapping_add(2) as u32,
            blk_b.as_ptr() as u32,
            blk_c.as_ptr() as u32,
            blk_d.as_ptr() as u32,
            x1.to_bits(), 0, g32(0x0103EED4), g32(0x00EE19E4),
            0, 0, 0xFFFFFFFFu32, 0, 0
        );
        ans
    }
});
