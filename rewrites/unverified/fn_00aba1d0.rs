// original: 0x00aba1d0 cap_use_handler (proposed)

/// Handle a capture-use request in two phases.
///
/// `arg0` points to the requester; the signed word at `arg0+0x2e` selects a
/// record through a global table, and `arg0+0x21c` seeds the working object.
/// `arg1`, `arg2` and `arg3` are flag bytes. Each phase resolves an index,
/// gathers four float quads through a chain of small calls, creates
/// a target object, tunes its flag words, optionally runs a matrix-vector
/// refinement from a matrix at `arg0+0x20` against two global seeds, submits
/// the quads, and records the gathered words into the target at `+0x240`
/// through `+0x24e`.
///
/// Control flow: a null requester returns at once. Either phase is skipped
/// when its resolve call answers -1; phase one additionally needs a flag
/// matrix over `arg1`, `arg3` and bit 0x8000 of `arg0+0x264`, and phase two
/// needs `arg2` set. A zero secondary index skips the quad-gathering block,
/// and a -1 table entry skips the target-tune block. The matrix refinement
/// runs only when the target's word at `+0x38` is non-zero (and, in phase
/// one, when `arg3` with the flag bit selects it).
///
/// The working buffers are shared across the phases exactly like the
/// original's stack slots: later quads overwrite earlier ones in place, and
/// words no phase writes keep their zero fill. Float operation order follows
/// the original exactly. Table, seed and timer addresses come from the
/// original image. Original is cdecl with four stack words and returns
/// nothing meaningful.
lf_checker_rt::export!(cdecl, rw_00aba1d0(arg0: u32, arg1: u32, arg2: u32, arg3: u32) -> u32 {
    unsafe {
        const IDX_WORD_OFF: u32 = 0x2e;
        const SEED_OFF: u32 = 0x21c;
        const FLAG_OFF: u32 = 0x264;
        const FLAG_BIT: u32 = 0x8000;
        const MAT_OFF: u32 = 0x20;
        const IDX_TABLE: u32 = 0x1295cd8;
        const REC_ARR_OFF: u32 = 0x12c;
        const SEED_B: u32 = 0xfe8b48;
        const SEED_A: u32 = 0xfe8b40;
        const TIMER: u32 = 0x11735b4;
        const TIMER_BIAS: u32 = 0x7530;
        const FMT_A: u32 = 0xea573c;
        const FMT_B: u32 = 0xea5748;
        const TGT_38: u32 = 0x38;
        const TGT_210: u32 = 0x210;
        const TGT_214: u32 = 0x214;
        const TGT_224: u32 = 0x224;
        const TGT_240: u32 = 0x240;
        const C_RESOLVE: u32 = 1;
        const C_GATHER: u32 = 2;
        const C_BYTES: u32 = 3;
        const C_INDEX: u32 = 4;
        const C_LOOKUP: u32 = 5;
        const C_DEREF: u32 = 6;
        const C_QUADLO: u32 = 7;
        const C_QUADHI: u32 = 8;
        const C_TUNE0: u32 = 9;
        const C_TUNE1: u32 = 10;
        const C_TUNE2: u32 = 11;
        const C_SUBMIT0: u32 = 12;
        const C_MODE: u32 = 13;
        const C_FORMAT: u32 = 14;
        const C_CREATE: u32 = 15;
        const C_ATTACH: u32 = 17;
        const C_LINK: u32 = 18;
        const C_REGISTER: u32 = 19;
        const C_SUBMIT1: u32 = 20;

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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        if arg0 == 0 {
            return 0;
        }
        let idx = ((rd32(arg0 + IDX_WORD_OFF) & 0xffff) as u16) as i16 as i32;
        let mut obj = rd32(arg0 + SEED_OFF).wrapping_add(0x80);
        let rec = rd32(
            lf_checker_rt::relocated(IDX_TABLE).wrapping_add((idx as u32).wrapping_mul(4)),
        );
        // Working buffers shared across both phases, like the original's slots.
        let mut gate = [0u32; 2];
        let mut gate2 = [0u32; 2];
        let mut quad_lo = [0u32; 4];
        let mut quad_hi = [0u32; 4];
        let mut scratch = [0u32; 4];
        let flag = rd32(arg0 + FLAG_OFF) & FLAG_BIT != 0;
        // Phase one.
        let r = lf_checker_rt::callee_thiscall!(C_RESOLVE, u32, obj, 0u32);
        if r != 0xffffffff {
            let run = if (arg1 as u8) == 0 {
                (arg3 as u8) != 0 && flag
            } else if !flag {
                true
            } else {
                (arg3 as u8) != 0 && flag
            };
            if run {
                lf_checker_rt::callee_thiscall!(
                    C_GATHER,
                    u32,
                    obj,
                    0u32,
                    gate.as_mut_ptr() as u32,
                    gate2.as_mut_ptr() as u32
                );
                let mut bbuf = [0u8; 8];
                lf_checker_rt::callee_thiscall!(
                    C_BYTES,
                    u32,
                    obj,
                    0u32,
                    bbuf.as_mut_ptr().add(1) as u32,
                    bbuf.as_mut_ptr() as u32
                );
                let index = lf_checker_rt::callee_thiscall!(C_INDEX, u32, obj, 0u32);
                let arr = rd32(gate2[1].wrapping_add(REC_ARR_OFF));
                let entry = rd32(arr.wrapping_add(index.wrapping_mul(4)));
                if entry != 0xffffffff {
                    let lk = lf_checker_rt::callee_thiscall!(C_LOOKUP, u32, arg0);
                    let base = rd32(lk + 0xc);
                    let d = lf_checker_rt::callee_thiscall!(
                        C_DEREF,
                        u32,
                        base.wrapping_add(entry.wrapping_mul(80))
                    );
                    if index != 0 {
                        lf_checker_rt::callee_thiscall!(
                            C_QUADLO,
                            u32,
                            obj,
                            0u32,
                            quad_lo.as_mut_ptr() as u32
                        );
                        lf_checker_rt::callee_thiscall!(
                            C_QUADHI,
                            u32,
                            obj,
                            0u32,
                            quad_hi.as_mut_ptr() as u32
                        );
                        lf_checker_rt::callee_thiscall!(
                            C_TUNE0,
                            u32,
                            scratch.as_mut_ptr() as u32
                        );
                        lf_checker_rt::callee_thiscall!(
                            C_TUNE1,
                            u32,
                            scratch.as_mut_ptr().add(1) as u32,
                            quad_hi.as_mut_ptr() as u32
                        );
                        lf_checker_rt::callee_thiscall!(
                            C_TUNE2,
                            u32,
                            scratch.as_mut_ptr().add(1) as u32,
                            d
                        );
                        lf_checker_rt::callee_thiscall!(
                            C_SUBMIT0,
                            u32,
                            scratch.as_mut_ptr() as u32,
                            scratch.as_mut_ptr() as u32
                        );
                    } else {
                        lf_checker_rt::callee_thiscall!(
                            C_SUBMIT0,
                            u32,
                            scratch.as_mut_ptr() as u32,
                            d
                        );
                    }
                }
                lf_checker_rt::callee_thiscall!(C_MODE, u32, obj, 0u32, 0u32);
                let tmp = lf_checker_rt::callee_cdecl!(
                    C_FORMAT,
                    u32,
                    lf_checker_rt::relocated(FMT_A),
                    3u32,
                    1u32,
                    1u32,
                    1u32,
                    0xffffffff
                );
                let tgt = lf_checker_rt::callee_cdecl!(
                    C_CREATE,
                    u32,
                    tmp,
                    3u32,
                    1u32,
                    1u32,
                    1u32,
                    0xffffffff
                );
                let vt: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(tgt) + 4) as usize);
                vt(tgt, scratch.as_mut_ptr() as u32, 0, 0);
                wr32(tgt + TGT_214, rd32(tgt + TGT_214) & 0xffffffef);
                wr32(
                    tgt + TGT_224,
                    rd32(lf_checker_rt::relocated(TIMER)).wrapping_add(TIMER_BIAS),
                );
                let mut fl = rd32(tgt + TGT_214);
                if (arg3 as u8) != 0 && flag {
                    fl |= 0x20;
                } else {
                    fl &= !0x20u32;
                }
                wr32(tgt + TGT_214, fl);
                lf_checker_rt::callee_thiscall!(C_ATTACH, u32, tgt, arg0, 0x40u32);
                lf_checker_rt::callee_thiscall!(C_LINK, u32, tgt, arg0, 0u32);
                lf_checker_rt::callee_cdecl!(C_REGISTER, u32, tgt, 0u32);
                if rd32(tgt + TGT_38) != 0 {
                    quad_hi[0] = 0x3e4ccccd;
                    quad_hi[1] = 0;
                    quad_hi[2] = 0xbc23d70a;
                    if (arg3 as u8) != 0 && flag {
                        let m = rd32(arg0 + MAT_OFF);
                        let c3 = rdf(lf_checker_rt::relocated(SEED_B));
                        let c2 = rdf(lf_checker_rt::relocated(SEED_A));
                        let t0 = mul(rdf(m), c3);
                        let mut v5 = mul(rdf(m + 0x10), 0.0);
                        let mut v4 = mul(rdf(m + 0x14), 0.0);
                        v5 = sub(v5, t0);
                        let t1 = mul(rdf(m + 0x20), c2);
                        let mut v1 = mul(rdf(m + 0x18), 0.0);
                        v5 = sub(v5, t1);
                        v4 = sub(v4, mul(rdf(m + 4), c3));
                        v4 = sub(v4, mul(rdf(m + 0x24), c2));
                        v1 = sub(v1, mul(rdf(m + 8), c3));
                        let t2 = mul(rdf(m + 0x28), c2);
                        quad_lo[0] = v5.to_bits();
                        quad_lo[1] = v4.to_bits();
                        v1 = sub(v1, t2);
                        quad_lo[3] = quad_hi[3];
                        quad_lo[2] = v1.to_bits();
                    } else {
                        quad_lo[0] = 0x40a00000;
                        quad_lo[1] = 0;
                        quad_lo[2] = 0x41c80000;
                    }
                    lf_checker_rt::callee_thiscall!(
                        C_SUBMIT1,
                        u32,
                        tgt,
                        quad_lo.as_mut_ptr() as u32,
                        quad_hi.as_mut_ptr() as u32,
                        0u32
                    );
                }
                wr32(tgt + TGT_210, rd32(tgt + TGT_210) | 0x8000000);
                wr32(tgt + TGT_240, idx as u32);
                wr32(tgt + TGT_240 + 4, gate[0]);
                wr32(tgt + TGT_240 + 8, gate2[0]);
                wr8(tgt + TGT_240 + 12, bbuf[1]);
                wr8(tgt + TGT_240 + 13, bbuf[0]);
                wr8(tgt + TGT_240 + 14, 0);
                obj = gate[1];
            }
        }
        // Phase two always runs; it shares the quad buffers with phase one
        // but gets fresh byte slots.
        {
            let mut bbuf2 = [0u8; 8];
            {
                let r2 = lf_checker_rt::callee_thiscall!(C_RESOLVE, u32, obj, 1u32);
                if r2 != 0xffffffff && (arg2 as u8) != 0 {
                    lf_checker_rt::callee_thiscall!(
                        C_GATHER,
                        u32,
                        obj,
                        1u32,
                        gate.as_mut_ptr() as u32,
                        gate2.as_mut_ptr() as u32
                    );
                    lf_checker_rt::callee_thiscall!(
                        C_BYTES,
                        u32,
                        obj,
                        0u32,
                        bbuf2.as_mut_ptr() as u32,
                        bbuf2.as_mut_ptr().add(1) as u32
                    );
                    let arr2 = rd32(gate2[1].wrapping_add(REC_ARR_OFF));
                    let entry2 = rd32(arr2.wrapping_add(4));
                    if entry2 != 0xffffffff {
                        let lk2 = lf_checker_rt::callee_thiscall!(C_LOOKUP, u32, arg0);
                        let base2 = rd32(lk2 + 0xc);
                        let d2 = lf_checker_rt::callee_thiscall!(
                            C_DEREF,
                            u32,
                            base2.wrapping_add(entry2.wrapping_mul(80))
                        );
                        lf_checker_rt::callee_thiscall!(
                            C_SUBMIT0,
                            u32,
                            scratch.as_mut_ptr() as u32,
                            d2
                        );
                    }
                    lf_checker_rt::callee_thiscall!(C_MODE, u32, obj, 1u32, 0u32);
                    let tmp2 = lf_checker_rt::callee_cdecl!(
                        C_FORMAT,
                        u32,
                        lf_checker_rt::relocated(FMT_B),
                        3u32,
                        1u32,
                        1u32,
                        1u32,
                        0xffffffff
                    );
                    let tgt2 = lf_checker_rt::callee_cdecl!(
                        C_CREATE,
                        u32,
                        tmp2,
                        3u32,
                        1u32,
                        1u32,
                        1u32,
                        0xffffffff
                    );
                    let vt2: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                        core::mem::transmute(rd32(rd32(tgt2) + 4) as usize);
                    vt2(tgt2, scratch.as_mut_ptr() as u32, 0, 0);
                    wr32(tgt2 + TGT_214, rd32(tgt2 + TGT_214) & 0xffffffef);
                    wr32(
                        tgt2 + TGT_224,
                        rd32(lf_checker_rt::relocated(TIMER)).wrapping_add(TIMER_BIAS),
                    );
                    lf_checker_rt::callee_thiscall!(C_ATTACH, u32, tgt2, arg0, 0x40u32);
                    lf_checker_rt::callee_thiscall!(C_LINK, u32, tgt2, arg0, 0u32);
                    lf_checker_rt::callee_cdecl!(C_REGISTER, u32, tgt2, 0u32);
                    quad_lo[0] = 0x40a00000;
                    quad_lo[1] = 0;
                    quad_lo[2] = 0x40400000;
                    quad_hi[0] = 0x3dcccccd;
                    quad_hi[1] = 0;
                    quad_hi[2] = 0;
                    if rd32(tgt2 + TGT_38) != 0 {
                        lf_checker_rt::callee_thiscall!(
                            C_SUBMIT1,
                            u32,
                            tgt2,
                            quad_lo.as_mut_ptr() as u32,
                            quad_hi.as_mut_ptr() as u32,
                            0u32
                        );
                    }
                    wr32(tgt2 + TGT_210, rd32(tgt2 + TGT_210) | 0x8000000);
                    wr32(tgt2 + TGT_240, idx as u32);
                    wr32(tgt2 + TGT_240 + 4, gate2[0]);
                    wr32(tgt2 + TGT_240 + 8, gate[0]);
                    wr8(tgt2 + TGT_240 + 12, bbuf2[0]);
                    wr8(tgt2 + TGT_240 + 13, bbuf2[1]);
                    wr8(tgt2 + TGT_240 + 14, 1);
                }
            }
        }
        0
    }
});
