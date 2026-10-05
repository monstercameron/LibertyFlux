// original: 0x00CB7CF0 peds_task_state_consumer (proposed)

/// Consume one vector of the shared ped-task state block and maybe raise its flag.
///
/// `obj` points to a task object: dword at `+0x00` is its vtable, dword at
/// `+0x20` points to a record holding three floats at `+0x30/+0x34/+0x38`.
/// The shared state is a block of globals holding two 3-vectors (the gate
/// vector A and the work vector B), three biases, a gate reference value, a
/// gate upper bound and one flag byte.
///
/// The function first gates on the third component: it continues only when
/// `4.0 > |v2 - gate_ref|`. It then dots A with the record vector plus the
/// gate bias and continues only when that dot lies in `[0, range_hi]`
/// (either comparison false on NaN keeps it running, matching the original's
/// jump-on-above). Past the gates it calls the vtable slot at `+0xEC` with a
/// five-word scratch block, dots B with the first three answered words into
/// one local and B with the record vector plus the out bias into another,
/// issues a nine-word worker call and a two-address check call whose low
/// byte steers the tail, and stores 1 into the flag byte when the tail
/// comparisons say so. Always returns 1 (low byte only; the upper bytes are
/// the check call's answer residue). Original: cdecl with one stack word.
lf_checker_rt::export!(cdecl, rw_00cb7cf0(obj: u32) -> u32 {
    unsafe {
        const OBJ_VTABLE: u32 = 0x00;
        const OBJ_VEC: u32 = 0x20;
        const VEC_X: u32 = 0x30;
        const VEC_Y: u32 = 0x34;
        const VEC_Z: u32 = 0x38;
        const VTASK_SLOT: u32 = 0xEC;
        // Shared state block (file VAs) and the two addresses passed to the check call.
        const ST_FLAG: u32 = 0x0171_BBE0;
        const ST_OUT_BIAS: u32 = 0x0171_BBE4;
        const ST_GATE_BIAS: u32 = 0x0171_BBE8;
        const ST_RANGE_HI: u32 = 0x0171_BBEC;
        const ST_A0: u32 = 0x0171_BC10;
        const ST_A1: u32 = 0x0171_BC14;
        const ST_A2: u32 = 0x0171_BC18;
        const ST_B0: u32 = 0x0171_BF20;
        const ST_B1: u32 = 0x0171_BF24;
        const ST_B2: u32 = 0x0171_BF28;
        const ST_GATE_REF: u32 = 0x0171_BF18;
        const ST_CHECK_ARG0: u32 = 0x0171_BF10;
        const ST_CHECK_ARG1: u32 = 0x0171_BF00;
        // Read-only constants by value: 4.0, |.| mask, 0.25, 0.5, -0.5.
        const GATE_LIMIT: f32 = 4.0;
        const ABS_MASK: u32 = 0x7FFF_FFFF;
        const QUARTER: f32 = 0.25;
        const HALF: f32 = 0.5;
        const NEG_HALF: f32 = -0.5;
        const CALLEE_VTASK: u32 = 1;
        const CALLEE_WORKER: u32 = 2;
        const CALLEE_CHECK: u32 = 3;
        const CALLEE_COOKIE: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits(g32(va)) }
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

        let vec = rd32(obj + OBJ_VEC);
        let v0 = rdf(vec + VEC_X);
        let v1 = rdf(vec + VEC_Y);
        let v2 = rdf(vec + VEC_Z);
        // Gate 1: continue only when 4.0 is strictly above |v2 - ref|.
        let gap = f32::from_bits(sub(v2, gf(ST_GATE_REF)).to_bits() & ABS_MASK);
        if GATE_LIMIT > gap {
            // Gate 2: the A-dot must land in [0, range_hi].
            let gate = add(
                add(add(mul(gf(ST_A1), v1), mul(gf(ST_A0), v0)), mul(gf(ST_A2), v2)),
                gf(ST_GATE_BIAS),
            );
            if !(0.0 > gate) && !(gate > gf(ST_RANGE_HI)) {
                // B dotted with the record vector, plus the out bias.
                let rout = add(
                    add(
                        add(mul(gf(ST_B1), v1), mul(gf(ST_B0), v0)),
                        mul(gf(ST_B2), v2),
                    ),
                    gf(ST_OUT_BIAS),
                );
                // Vtable task call: five-word scratch block, first three read back.
                let vt = rd32(obj + OBJ_VTABLE);
                let vtask: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(vt + VTASK_SLOT) as usize);
                let mut out = [0u32; 5];
                vtask(obj, out.as_mut_ptr() as u32);
                let o0 = f32::from_bits(out[0]);
                let o1 = f32::from_bits(out[1]);
                let o2 = f32::from_bits(out[2]);
                // B dotted with the answered words (no bias).
                let rwork = add(add(mul(gf(ST_B1), o1), mul(gf(ST_B0), o0)), mul(gf(ST_B2), o2));
                // Worker call: scratch word 4 in ecx, object, v2, 0.25, six zeros.
                lf_checker_rt::callee_thiscall!(
                    CALLEE_WORKER,
                    u32,
                    (&out[4] as *const u32) as u32,
                    obj,
                    v2.to_bits(),
                    QUARTER.to_bits(),
                    0u32,
                    0u32,
                    0u32,
                    0u32,
                    0u32,
                    0u32
                );
                // Check call: two state addresses; its low byte steers the tail.
                let ok = lf_checker_rt::callee_stdcall!(
                    CALLEE_CHECK,
                    u32,
                    lf_checker_rt::relocated(ST_CHECK_ARG0),
                    lf_checker_rt::relocated(ST_CHECK_ARG1)
                ) & 0xFF;
                let mut raise = false;
                if ok == 0 && HALF > rwork {
                    raise = true;
                }
                if !raise {
                    if rout > 0.0 {
                        if NEG_HALF > rwork {
                            raise = true;
                        }
                    }
                    if !raise && 0.0 > rout && rwork > HALF {
                        raise = true;
                    }
                }
                if raise {
                    lf_checker_rt::global::<u8>(ST_FLAG).write(1);
                }
            }
        }
        // CRT security-cookie check: preserves every register, always runs.
        lf_checker_rt::callee_cdecl!(CALLEE_COOKIE, u32,);
        1
    }
});
