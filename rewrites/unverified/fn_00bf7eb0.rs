// original: 0x00bf7eb0 fire_ped_task_dispatch (proposed)

/// Dispatch a fire task for a ped subject: validate the task id, resolve a
/// target through the task registry, measure and normalize a body/limb
/// offset, fan the resulting blocks out through the shaping calls, run the
/// per-flag setup, and submit one 16-word job to the worker queue.
///
/// `this` (ECX) is the task object: id dword at `+0x08`, floats at `+0x18`
/// and `+0x1c`, selector bytes at `+0x20`/`+0x21`, a small addend at `+0x22`,
/// gate byte at `+0x23`, strength flag bit 1 at `+0x03`. `subject` (arg1) is
/// the subject pointer, `param` (arg2) a float compared against +0.0.
///
/// Stages: the id must equal one of two registry lookups (equality only, so
/// the lookup runs once or twice); the registry resolves a target object
/// (null aborts, reporting an out-flag byte) from the id and the addend plus
/// the subject address. Two geometry lookups keyed by the selector bytes
/// yield anchor triples; their differences are halved toward the first anchor
/// (midpoint lerp) and normalized by one over their length, or zeroed when
/// the squared length is exactly +0.0 (the original tests this with a parity
/// trick on the compare flags, taken for every nonzero value and NaN). The
/// normalized triple, the lerped triple and a fresh block go to the shaping
/// call, whose out-block is gathered (strided groups of three) and mixed with
/// two more calls; a second gather feeds the attach call. When `param` is
/// exactly +0.0 (both zeros; same parity trick) strength and speed attach;
/// otherwise a polled helper may run. The target records 1.0 at `+0x1f0`,
/// upgraded to 3.0 under the strength flag; when the out-flag byte is set a
/// sequencer stores a counter, bumped on equality with the poll answer, at
/// target `+0x1d4`. The full submit needs the gate bit and a zero addend;
/// then a multiplicative generator over a global pair is scaled to range and
/// the 16-word job (consts, struct words, the rolled or constant value, one
/// global word) is submitted and its answer returned. Earlier exits return
/// the last callee answer, or a sequencer value, unchanged (the null-subject
/// exit returns entry EAX instead). All integer compares are equality
/// (against zero, another value, or a callee answer); there is no
/// signed/unsigned-sensitive comparison.
///
/// Original: 0x00bf7eb0 (thiscall, two stack words: subject pointer, float).
/// Frame scratch below the call is compared only where snapshotted; stack
/// addresses passed to callees are skipped with snapshots in the contract,
/// as are the out-block addresses in ECX (compared via snapshots instead).
lf_checker_rt::export!(thiscall, rw_00bf7eb0(esi: u32, edi: u32, farg: u32) -> u32 {
    unsafe {
        const FIRE_PED_BODY: u32 = 0x00EBC4EC;
        const FIRE_PED_LIMB: u32 = 0x00EBC4FC;
        const P_STRENGTH: u32 = 0x00EBC50C;
        const P_SPEED: u32 = 0x00EBC518;
        const TASK_REGISTRY: u32 = 0x01394D60;
        const RNG_MULT: u32 = 0x5CDCFAA7;
        const MANTISSA_MASK: u32 = 0x007FFFFF;
        const JOB_TAG: u32 = 0x201;

        const TASK_ID: u32 = 0x08;
        const TASK_FLOAT: u32 = 0x18;
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
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }

        let edi0 = edi;
        if edi0 == 0 {
            return 0;
        }
        let mut eax_var: u32;
        // (eax_var assigned at the registry resolve below on all live paths)
        let id8 = rd32(esi + TASK_ID);
        let a1: u32 = lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(FIRE_PED_BODY), 0);
        if id8 != a1 {
            let _: u32 = lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(FIRE_PED_LIMB), 0);
        }
        let b22 = rd8(esi + 0x22) as u32;
        let mut flagbuf = [0u8; 4];
        let target: u32 = lf_checker_rt::callee_thiscall!(2, u32, lf_checker_rt::relocated(TASK_REGISTRY),
            b22.wrapping_add(edi0.wrapping_add(2)), id8,
            flagbuf.as_mut_ptr() as u32, 0, 0);
        eax_var = target;
        if target == 0 {
            return eax_var;
        }
        let edi = target;
        // Geometry: two lookups, differences, midpoint lerp, normalize.
        let p1: u32 = lf_checker_rt::callee_thiscall!(3, u32, edi0, rd8(esi + 0x20) as u32);
        let p2: u32 = lf_checker_rt::callee_thiscall!(3, u32, edi0, rd8(esi + 0x21) as u32);
        let d34 = sub(rdf(p2 + 0x34), rdf(p1 + 0x34));
        let d38 = sub(rdf(p2 + 0x38), rdf(p1 + 0x38));
        let d30 = sub(rdf(p2 + 0x30), rdf(p1 + 0x30));
        let khalf = f32::from_bits(g32(0x00FE8830));
        let mut lx1 = mul(d34, khalf);
        lx1 = add(lx1, rdf(p1 + 0x34));
        let mut lx2 = mul(d38, khalf);
        let mut lx0 = mul(d30, khalf);
        lx2 = add(lx2, rdf(p1 + 0x38));
        lx0 = add(lx0, rdf(p1 + 0x30));
        let mut len2 = mul(d30, d30);
        len2 = add(len2, mul(d34, d34));
        len2 = add(len2, mul(d38, d38));
        // Same parity trick: normalize iff len2 != +0.0 exactly.
        let norm: f32;
        if len2 != 0.0 {
            norm = div(f32::from_bits(g32(0x00FE88E8)), len2.sqrt());
        } else {
            norm = 0.0;
        }
        // Note the original's asymmetric operand order (d30*norm but
        // norm*d34 and norm*d38); mirrored exactly for NaN payloads.
        let nd30 = mul(d30, norm);
        let nd34 = mul(norm, d34);
        let nd38 = mul(norm, d38);
        let mut buf_b0 = [0u32; 15];
        let buf_lerp = [lx0.to_bits(), lx1.to_bits(), lx2.to_bits()];
        let buf_norm = [nd30.to_bits(), nd34.to_bits(), nd38.to_bits()];
        let _: u32 = lf_checker_rt::callee_cdecl!(4, u32, buf_b0.as_mut_ptr() as u32,
            buf_lerp.as_ptr() as u32, buf_norm.as_ptr() as u32, 1);
        // Block fan-out: out-block, two gathers, two mixes, attach.
        let mut buf70 = [0u32; 15];
        let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, buf70.as_mut_ptr() as u32, p1);
        // First gather: strided dests (groups of 3 + fill gap at 3,7,11).
        let mut buf130 = [0u32; 15];
        buf130[0] = buf70[0]; buf130[1] = buf70[1]; buf130[2] = buf70[2];
        buf130[4] = buf70[4]; buf130[5] = buf70[5]; buf130[6] = buf70[6];
        buf130[8] = buf70[8]; buf130[9] = buf70[9]; buf130[10] = buf70[10];
        buf130[12] = buf70[12]; buf130[13] = buf70[13]; buf130[14] = buf70[14];
        let _: u32 = lf_checker_rt::callee_thiscall!(6, u32, buf70.as_ptr() as u32,
            buf130.as_ptr() as u32);
        let mut buf_f0 = [0u32; 16];
        buf_f0[0] = buf_b0[0]; buf_f0[1] = buf_b0[1]; buf_f0[2] = buf_b0[2];
        buf_f0[4] = buf_b0[4]; buf_f0[5] = buf_b0[5]; buf_f0[6] = buf_b0[6];
        buf_f0[8] = buf_b0[8]; buf_f0[9] = buf_b0[9]; buf_f0[10] = buf_b0[10];
        buf_f0[12] = buf_b0[12]; buf_f0[13] = buf_b0[13]; buf_f0[14] = buf_b0[14];
        let _: u32 = lf_checker_rt::callee_thiscall!(7, u32, buf_f0.as_ptr() as u32,
            buf70.as_ptr() as u32);
        let _: u32 = lf_checker_rt::callee_thiscall!(8, u32, edi, buf_f0.as_ptr() as u32);
        // Float gate (taken <=> f != 0.0; lahf residue clobbered later).
        let f = f32::from_bits(farg);
        if f != 0.0 {
            if flagbuf[0] == 0 {
                let a10: u32 = lf_checker_rt::callee_thiscall!(10, u32, esi);
                eax_var = a10;
                if (a10 & 0xFF) != 0 {
                    let mut wbuf = [0u32; 2];
                    let _: u32 = lf_checker_rt::callee_cdecl!(11, u32, wbuf.as_mut_ptr() as u32, esi);
                    let a12: u32 = lf_checker_rt::callee_thiscall!(12, u32, esi, edi, wbuf[0], farg);
                    eax_var = a12;
                }
            }
        } else {
            let f18 = rdf(esi + TASK_FLOAT);
            let _: u32 = lf_checker_rt::callee_thiscall!(9, u32, edi, lf_checker_rt::relocated(P_STRENGTH), f18.to_bits());
            let f1c = rdf(esi + 0x1C);
            let a9b: u32 = lf_checker_rt::callee_thiscall!(9, u32, edi, lf_checker_rt::relocated(P_SPEED), f1c.to_bits());
            eax_var = a9b;
        }
        eax_var = lf_checker_rt::callee_thiscall!(13, u32, edi, p1);
        wr32(edi + 0x1F0, 0x3F800000);
        if (rd8(esi + 3) & 2) != 0 {
            wr32(edi + 0x1F0, 0x40400000);
        }
        if flagbuf[0] != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(14, u32, edi);
            let a15: u32 = lf_checker_rt::callee_cdecl!(15, u32,);
            let mut seq = g32(0x011F70C4);
            if g32(0x011F702C) == a15 {
                seq = seq.wrapping_add(1);
            }
            wr32(edi + TARGET_SEQ, seq);
            eax_var = seq;
        }
        if rd8(esi + 0x23) & 1 == 0 {
            return eax_var;
        }
        if b22 != 0 {
            return eax_var;
        }
        // RNG over the global pair, scaled to range (same shape as FN1).
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
        let a16: u32 = lf_checker_rt::callee_cdecl!(16, u32,);
        let x1: f32;
        if (a16 & 0xFF) != 0 {
            x1 = f32::from_bits(g32(0x00EE19DC));
        } else {
            let a17: u32 = lf_checker_rt::callee_cdecl!(17, u32,);
            if (a17 & 0xFF) != 0 {
                x1 = f32::from_bits(g32(0x00EE19DC));
            } else {
                x1 = rng;
            }
        }
        // Job structs: arg3->[S+0x60] (8 bytes below it: the lerped second
        // word + fill), arg4->[S+0x40], arg5->[S+0xE0] (scripted tail of the
        // b0 block), arg6->[S+0x30].
        let blk60a: [u32; 6] = [lx2.to_bits(), 0, 0, 0, 0x3F800000, 0];
        let blk40: [u32; 4] = [0, 0x3F800000, 0, 0];
        let blk30: [u32; 4] = [g32(0x00EE19D0), g32(0x00EE19D4), g32(0x00EE19D8), 0];
        let ans: u32 = lf_checker_rt::callee_cdecl!(
            18, u32, 0, 0, JOB_TAG,
            blk60a.as_ptr().wrapping_add(2) as u32,
            blk40.as_ptr() as u32,
            buf_b0.as_ptr().wrapping_add(12) as u32,
            blk30.as_ptr() as u32,
            x1.to_bits(), 0, g32(0x0103EED4), g32(0x00EE19E4),
            0, 0, 0xFFFFFFFFu32, 0, 0
        );
        ans
    }
});
