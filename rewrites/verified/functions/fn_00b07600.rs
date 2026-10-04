// original: 0x00b07600 cam_gate_and_recenter (proposed)

/// Camera recenter gate (thiscall, `this` in ECX, no stack arguments, no
/// meaningful return: the single known caller ignores EAX). A chain of seven
/// gated checks must all pass before a recenter step runs: a provider callee
/// returns a status object whose xor-folded bytes must exceed 0x7F; a watcher
/// callee returns an object whose virtual slot at +0x28 must answer exactly 9
/// (called through the object exactly like the original, intercepted by a
/// planted stub); a singleton must exist and its task-finder callee (selector
/// 0x41E) must answer non-null; two flag callees must both answer zero. Any
/// failure leaves the object with bit 0x02 of the flag byte at +0x1C5 cleared
/// and returns.
/// The recenter step negates the three floats at +0x20/+0x24/+0x28 (xor with
/// the sign mask, bit-exact) into a four-float frame buffer whose fourth slot
/// the original fills from an uninitialized stack word (the contract defines
/// the stack fill as 0 there), asks a transform callee to fill the buffer
/// (frame pointer skipped, contents snapshotted and scripted), scales the
/// first three by a global factor and subtracts them from the floats at
/// +0x40/+0x44/+0x48, stores the fourth to +0x4C, and sets flag bit 0x02.
/// Float operation order is the original's.
lf_checker_rt::export!(thiscall, rw_00b07600(this: u32) -> u32 {
    unsafe {
        const RC_VEC: u32 = 0x20;
        const RC_POS: u32 = 0x40;
        const RC_POS_W: u32 = 0x4C;
        const RC_FLAGS: u32 = 0x1C5;
        const RC_DONE: u8 = 0x02;
        const RC_VT_SLOT: u32 = 0x28;
        const RC_WANT: u32 = 9;
        const RC_FIND_SEL: u32 = 0x41E;
        const G_SIGN: u32 = 0x00FE8FA0;
        const G_SCALE: u32 = 0x00FE87E8;
        const ST_XOR_A: u32 = 0x270C;
        const ST_XOR_B: u32 = 0x270E;
        const SING_TASK: u32 = 0x224;
        const TASK_BIAS: u32 = 0x44;

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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        let st: u32 = lf_checker_rt::callee_cdecl!(1, u32, 0, 0);
        let mut go = false;
        if st != 0 {
            let x = rd8(st + ST_XOR_B) ^ rd8(st + ST_XOR_A);
            go = x > 0x7F;
        }
        wr8(this + RC_FLAGS, rd8(this + RC_FLAGS) & !RC_DONE);
        if go {
            let w: u32 = lf_checker_rt::callee_thiscall!(2, u32, this);
            if w != 0 {
                let slot: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(w) + RC_VT_SLOT) as usize);
                if slot(w) == RC_WANT {
                    let sing: u32 = lf_checker_rt::callee_cdecl!(4, u32,);
                    if sing != 0 {
                        let tasker = rd32(sing + SING_TASK).wrapping_add(TASK_BIAS);
                        let found: u32 =
                            lf_checker_rt::callee_thiscall!(5, u32, tasker, RC_FIND_SEL);
                        if found != 0 {
                            let f1: u32 = lf_checker_rt::callee_cdecl!(6, u32, 1);
                            if (f1 as u8) == 0 {
                                let f2: u32 = lf_checker_rt::callee_cdecl!(7, u32,);
                                if (f2 as u8) == 0 {
                                    let sign = rd32(lf_checker_rt::relocated(G_SIGN));
                                    let b0 = rd32(this + RC_VEC) ^ sign;
                                    let b1 = rd32(this + RC_VEC + 4) ^ sign;
                                    let b2 = rd32(this + RC_VEC + 8) ^ sign;
                                    // Fourth slot: the original reads an
                                    // uninitialized stack word here; the
                                    // contract sets the defined fill to 0.
                                    let mut buf = [b0, b1, b2, 0u32];
                                    lf_checker_rt::callee_thiscall!(8, u32, this + 0x10,
                                        buf.as_mut_ptr() as u32);
                                    let scale = rdf(lf_checker_rt::relocated(G_SCALE));
                                    let r0 = mul(f32::from_bits(buf[0]), scale);
                                    let r1 = mul(f32::from_bits(buf[1]), scale);
                                    let r2 = mul(f32::from_bits(buf[2]), scale);
                                    wrf(this + RC_POS, sub(rdf(this + RC_POS), r0));
                                    wrf(this + RC_POS + 4, sub(rdf(this + RC_POS + 4), r1));
                                    wrf(this + RC_POS + 8, sub(rdf(this + RC_POS + 8), r2));
                                    wr32(this + RC_POS_W, buf[3]);
                                    wr8(this + RC_FLAGS, rd8(this + RC_FLAGS) | RC_DONE);
                                }
                            }
                        }
                    }
                }
            }
        }
        0
    }
});
