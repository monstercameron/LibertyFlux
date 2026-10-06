// original: 0x00bf8430 fire_ped_smoke_dispatch (proposed)

/// Dispatch a ped-smoke fire task for a subject: resolve a target through
/// the task registry, poll two vtable alternates for anchor objects, measure
/// and normalize their offset, fan the resulting blocks out through the
/// shaping calls, attach, and record a sequencer value.
///
/// `this` (ECX) is the task object (id dword at `+0x08 only). `subject`
/// (arg1, the only stack word) is the subject: vtable pointer at `+0x00`,
/// fallback anchor pointer at `+0x100`.
///
/// Stages: one registry lookup runs (its answer is discarded); the registry
/// resolves a target object from the id (null aborts). Two identical
/// alternate blocks poll vtable slot `+0xa0` of the subject: on a nonzero
/// answer the slot is polled again and vtable slot `+0xe0` of that answer
/// runs, else the `+0x100` fallback is used; the word after the resulting
/// pointer plus a per-block tag (0x4b3, 0x36a1) goes to the shaping call,
/// whose answers become the two geometry anchors. Their differences are
/// halved toward the first anchor (midpoint lerp) and normalized by one over
/// their length, or zeroed when the squared length is exactly +0.0 (the
/// original tests this with jnp over a parity-tested compare, taken exactly
/// for ordered equality). The normalized triple, the lerped triple and a
/// fresh block go to the shaping call, whose out-block is gathered (strided
/// groups of three) and mixed with two more calls; a second gather feeds the
/// attach call. The target is attached with the first anchor, submitted with
/// the subject, and run through the poll; a sequencer value, bumped on
/// equality with the poll answer, is stored at target `+0x1d4` and returned.
/// Earlier exits return 0 (the null-subject exit returns entry EAX instead).
/// All integer compares are equality (against zero or another value); there
/// is no signed/unsigned-sensitive comparison.
///
/// Original: 0x00bf8430 (thiscall, one stack word: subject pointer). Frame
/// scratch below the call is compared only where snapshotted; stack
/// addresses passed to callees are skipped with snapshots in the contract,
/// as are the out-block addresses in ECX (compared via snapshots instead).
lf_checker_rt::export!(thiscall, rw_00bf8430(esi: u32, edi: u32) -> u32 {
    unsafe {
        const FIRE_PED_SMOKE: u32 = 0x00EBC448;
        const TASK_REGISTRY: u32 = 0x01394D60;
        const TASK_ID: u32 = 0x08;
        const TARGET_SEQ: u32 = 0x1D4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        let id8 = rd32(esi + TASK_ID);
        // Lookup; answer unused (EAX clobbered by the next call).
        let _: u32 = lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(FIRE_PED_SMOKE), 0);
        let target: u32 = lf_checker_rt::callee_thiscall!(2, u32, lf_checker_rt::relocated(TASK_REGISTRY), id8, 0, 0);
        if target == 0 {
            return 0;
        }
        // Two vtable-alternate blocks: poll slot +0xa0; on nonzero poll
        // again and run slot +0xe0 through the answer, else use +0x100.
        let v1a: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(edi0) + 0xA0) as usize);
        let r1 = v1a(edi0);
        let ea: u32;
        if r1 == 0 {
            ea = rd32(edi0 + 0x100);
        } else {
            let r2 = v1a(edi0);
            let v2a: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(r2) + 0xE0) as usize);
            ea = v2a(r2);
        }
        let ans_a: u32 = lf_checker_rt::callee_cdecl!(5, u32, rd32(ea + 4), 0x4B3);
        let r3 = v1a(edi0);
        let eb: u32;
        if r3 == 0 {
            eb = rd32(edi0 + 0x100);
        } else {
            let r4 = v1a(edi0);
            let v2b: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(r4) + 0xE0) as usize);
            eb = v2b(r4);
        }
        let ans_b: u32 = lf_checker_rt::callee_cdecl!(5, u32, rd32(eb + 4), 0x36A1);
        // Geometry over the two answers.
        let m1: u32 = lf_checker_rt::callee_thiscall!(6, u32, edi0, ans_a);
        let m2: u32 = lf_checker_rt::callee_thiscall!(6, u32, edi0, ans_b);
        let d34 = sub(rdf(m2 + 0x34), rdf(m1 + 0x34));
        let d38 = sub(rdf(m2 + 0x38), rdf(m1 + 0x38));
        let d30 = sub(rdf(m2 + 0x30), rdf(m1 + 0x30));
        let khalf = f32::from_bits(g32(0x00FE8830));
        let mut lx1 = mul(d34, khalf);
        lx1 = add(lx1, rdf(m1 + 0x34));
        let mut lx2 = mul(d38, khalf);
        // Note the reversed first lerp: mem + (d30*khalf), unlike siblings.
        let lx0 = add(rdf(m1 + 0x30), mul(d30, khalf));
        lx2 = add(lx2, rdf(m1 + 0x38));
        let mut len2 = mul(d30, d30);
        len2 = add(len2, mul(d34, d34));
        len2 = add(len2, mul(d38, d38));
        // jnp after the parity test: normalize iff len2 != +0.0 exactly.
        let norm: f32;
        if len2 != 0.0 {
            norm = div(f32::from_bits(g32(0x00FE88E8)), len2.sqrt());
        } else {
            norm = 0.0;
        }
        // Asymmetric operand order (d30*norm but norm*d34, norm*d38).
        let nd30 = mul(d30, norm);
        let nd34 = mul(norm, d34);
        let nd38 = mul(norm, d38);
        let mut buf_f0 = [0u32; 15];
        let buf_lerp = [lx0.to_bits(), lx1.to_bits(), lx2.to_bits()];
        let buf_norm = [nd30.to_bits(), nd34.to_bits(), nd38.to_bits()];
        let _: u32 = lf_checker_rt::callee_cdecl!(7, u32, buf_f0.as_mut_ptr() as u32,
            buf_lerp.as_ptr() as u32, buf_norm.as_ptr() as u32, 1);
        // Block fan-out: out-block, two gathers, two mixes, attach, submit.
        let mut buf30 = [0u32; 15];
        let _: u32 = lf_checker_rt::callee_thiscall!(8, u32, buf30.as_mut_ptr() as u32, m1);
        let mut buf_b0 = [0u32; 16];
        buf_b0[0] = buf30[0]; buf_b0[1] = buf30[1]; buf_b0[2] = buf30[2];
        buf_b0[4] = buf30[4]; buf_b0[5] = buf30[5]; buf_b0[6] = buf30[6];
        buf_b0[8] = buf30[8]; buf_b0[9] = buf30[9]; buf_b0[10] = buf30[10];
        buf_b0[12] = buf30[12]; buf_b0[13] = buf30[13]; buf_b0[14] = buf30[14];
        let _: u32 = lf_checker_rt::callee_thiscall!(9, u32, buf30.as_ptr() as u32,
            buf_b0.as_ptr() as u32);
        let mut buf70 = [0u32; 15];
        buf70[0] = buf_f0[0]; buf70[1] = buf_f0[1]; buf70[2] = buf_f0[2];
        buf70[4] = buf_f0[4]; buf70[5] = buf_f0[5]; buf70[6] = buf_f0[6];
        buf70[8] = buf_f0[8]; buf70[9] = buf_f0[9]; buf70[10] = buf_f0[10];
        buf70[12] = buf_f0[12]; buf70[13] = buf_f0[13]; buf70[14] = buf_f0[14];
        let _: u32 = lf_checker_rt::callee_thiscall!(10, u32, buf70.as_ptr() as u32,
            buf30.as_ptr() as u32);
        let _: u32 = lf_checker_rt::callee_thiscall!(11, u32, target, m1);
        let _: u32 = lf_checker_rt::callee_thiscall!(12, u32, target, buf70.as_ptr() as u32);
        let _: u32 = lf_checker_rt::callee_thiscall!(13, u32, lf_checker_rt::relocated(TASK_REGISTRY), target, edi0, 0);
        let _: u32 = lf_checker_rt::callee_thiscall!(14, u32, target);
        let a15: u32 = lf_checker_rt::callee_cdecl!(15, u32,);
        let mut seq = g32(0x011F70C4);
        if g32(0x011F702C) == a15 {
            seq = seq.wrapping_add(1);
        }
        wr32(target + TARGET_SEQ, seq);
        seq
    }
});
