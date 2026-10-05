// original: 0x00cded20 CTaskSimpleNMShot::vf27 (symbols)

/// NaturalMotion "shot" task update: forward a setup call, send a hit
/// message, transform the hit point, then run the weapon-fire tail.
///
/// `this` is the task object, `target` the ped it acts on. The task keeps a
/// helper context at `+0x28`, a shot parameter at `+0x48`, an enable word at
/// `+0x4c`, a hit record pointer at `+0x54`, three timer words at
/// `+0x94`/`+0x98`/`+0x9c`, and two flag bytes at `+0xa0`/`+0xa3`. The hit
/// record holds a fallback vector at `+0x10`, a matrix-or-null at `+0x20`, a
/// flag word at `+0x28`, and an info pointer at `+0x38`.
///
/// Behaviour: call the setup helper with the task context and the target. If
/// the hit record is present, load the hit vector (from the matrix offset or
/// the fallback), optionally run it through an executor callee that rewrites
/// four frame words, and when the message flag is set send a boolean/vector
/// message. When the enable word is non-zero, reload the vector, run the
/// executor again, lazily initialise the matrix when null, apply a 3x3
/// transform in the original's order, and hand the results to a sink callee;
/// the message is then sent whether or not the enable word was set.
/// The tail compares a global timer against the three timer words (clearing
/// or skipping stages), then requires a chain of target fields before
/// calling the trajectory and weapon-info callees; the weapon info plus the
/// timer becomes the new third timer word.
///
/// The executor's four rewritten words and the lazy matrix pointer are the
/// only callee-written memory the original reads back. Float order is the
/// original's. Original: thiscall, one stack word, no return value
/// (callee pops the argument).
lf_checker_rt::export!(thiscall, rw_00cded20(this: u32, target: u32) -> u32 {
    unsafe {
        const TASK_HELPER: u32 = 0x28;
        const TASK_PARAM: u32 = 0x48;
        const TASK_ENABLE: u32 = 0x4c;
        const TASK_HIT: u32 = 0x54;
        const TASK_T0: u32 = 0x94;
        const TASK_T1: u32 = 0x98;
        const TASK_T2: u32 = 0x9c;
        const TASK_KIND: u32 = 0xa0;
        const TASK_SEND: u32 = 0xa3;
        const TARGET_MSG_TO: u32 = 0x7b4;
        const TARGET_FLAG: u32 = 0x211;
        const TARGET_DET: u32 = 0x2c4;
        const HIT_FALLBACK: u32 = 0x10;
        const HIT_MAT: u32 = 0x20;
        const HIT_FLAGS: u32 = 0x28;
        const HIT_INFO: u32 = 0x38;
        const FLAG_MASK: u32 = 0x3c0;
        const FLAG_EXEC: u32 = 0xc0;
        const MAT_VEC_OFF: u32 = 0x30;
        const EXEC_ARG0: u32 = 0x4b5;
        const G_TIMER: u32 = 0x0117_35b4;
        const G_BOOL_A: u32 = 0x0105_1e18;
        const G_VEC_A: u32 = 0x0105_1e1c;
        const G_SEND_A: u32 = 0x0105_1dfc;
        const G_BOOL_B: u32 = 0x0105_2008;
        const G_VEC_B: u32 = 0x0105_2018;
        const G_SEND_B: u32 = 0x0105_1fd8;
        const G_TAIL_BOOL: u32 = 0x0105_1cc8;
        const G_TAIL_SEND: u32 = 0x0105_1e88;
        const C_INIT: u32 = 1;
        const C_SET_BOOL: u32 = 2;
        const C_SET_VEC3: u32 = 3;
        const C_SEND2: u32 = 4;
        const C_SEND3: u32 = 5;
        const C_EXEC: u32 = 6;
        const C_SETUP: u32 = 7;
        const C_MK_A: u32 = 8;
        const C_MK_B: u32 = 9;
        const C_SINK: u32 = 10;
        const C_GATE: u32 = 11;
        const C_TRAJ_A: u32 = 12;
        const C_TRAJ_B: u32 = 13;
        const C_WINFO: u32 = 14;
        const C_COOKIE: u32 = 15;
        const F_0C: usize = 0x0c / 4;
        const F_10: usize = 0x10 / 4;
        const F_14: usize = 0x14 / 4;
        const F_18: usize = 0x18 / 4;
        const F_20: usize = 0x20 / 4;
        const F_24: usize = 0x24 / 4;
        const F_28: usize = 0x28 / 4;
        const F_3C: usize = 0x3c / 4;
        const F_40: usize = 0x40 / 4;
        const F_44: usize = 0x44 / 4;
        const F_48: usize = 0x48 / 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
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
        unsafe fn gid(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read() }
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

        let mut fr = [0u32; 0x80];
        let frame = fr.as_mut_ptr() as u32;
        let buf = frame.wrapping_add(0x50);
        let at = |w: usize| frame.wrapping_add((w * 4) as u32);
        macro_rules! frd {
            ($w:expr) => {
                f32::from_bits(fr[$w])
            };
        }

        lf_checker_rt::callee_thiscall!(C_SETUP, u32, this.wrapping_add(TASK_HELPER), target);
        let hit = rd32(this.wrapping_add(TASK_HIT));
        if hit != 0 {
            let m = rd32(hit.wrapping_add(HIT_MAT));
            let base = if m != 0 {
                m.wrapping_add(MAT_VEC_OFF)
            } else {
                hit.wrapping_add(HIT_FALLBACK)
            };
            fr[F_20] = rd32(base);
            fr[F_24] = rd32(base.wrapping_add(4));
            fr[F_28] = rd32(base.wrapping_add(8));
            if rd32(hit.wrapping_add(HIT_FLAGS)) & FLAG_MASK == FLAG_EXEC {
                lf_checker_rt::callee_stdcall!(C_EXEC, u32, at(F_20), EXEC_ARG0);
            }
            lf_checker_rt::callee_thiscall!(C_INIT, u32, buf);
            if rd8(this.wrapping_add(TASK_SEND)) != 0 {
                let to = rd32(target.wrapping_add(TARGET_MSG_TO));
                if rd8(this.wrapping_add(TASK_KIND)) != 0 {
                    lf_checker_rt::callee_thiscall!(C_SET_BOOL, u32, buf, gid(G_BOOL_A), 1);
                    lf_checker_rt::callee_thiscall!(
                        C_SET_VEC3, u32, buf, gid(G_VEC_A), fr[F_20], fr[F_24], fr[F_28]
                    );
                    lf_checker_rt::callee_thiscall!(C_SEND2, u32, to, gid(G_SEND_A), buf);
                } else {
                    lf_checker_rt::callee_thiscall!(C_SET_BOOL, u32, buf, gid(G_BOOL_B), 1);
                    lf_checker_rt::callee_thiscall!(
                        C_SET_VEC3, u32, buf, gid(G_VEC_B), fr[F_20], fr[F_24], fr[F_28]
                    );
                    lf_checker_rt::callee_thiscall!(C_SEND2, u32, to, gid(G_SEND_B), buf);
                }
            }
            if rd32(this.wrapping_add(TASK_ENABLE)) != 0 {
                let p = rd32(this.wrapping_add(TASK_HIT));
                let m2 = rd32(p.wrapping_add(HIT_MAT));
                let base2 = if m2 != 0 {
                    m2.wrapping_add(MAT_VEC_OFF)
                } else {
                    p.wrapping_add(HIT_FALLBACK)
                };
                fr[F_10] = rd32(base2);
                fr[F_14] = rd32(base2.wrapping_add(4));
                fr[F_18] = rd32(base2.wrapping_add(8));
                if rd32(p.wrapping_add(HIT_FLAGS)) & FLAG_MASK == FLAG_EXEC {
                    lf_checker_rt::callee_stdcall!(
                        C_EXEC, u32, at(F_10),
                        rd32(this.wrapping_add(TASK_PARAM))
                    );
                }
                let mut x2 = frd!(F_10);
                let mut x3 = frd!(F_14);
                let mut x4 = frd!(F_18);
                fr[F_40] = x2.to_bits();
                fr[F_44] = x3.to_bits();
                fr[F_48] = x4.to_bits();
                fr[F_0C] = p;
                if rd32(p.wrapping_add(HIT_MAT)) == 0 {
                    lf_checker_rt::callee_thiscall!(C_MK_A, u32, p);
                    lf_checker_rt::callee_thiscall!(
                        C_MK_B, u32, p.wrapping_add(HIT_FALLBACK),
                        rd32(p.wrapping_add(HIT_MAT))
                    );
                    x2 = frd!(F_10);
                    x3 = frd!(F_14);
                    x4 = frd!(F_18);
                }
                let q = rd32(fr[F_0C].wrapping_add(HIT_MAT));
                x2 = sub(x2, rdf(q.wrapping_add(0x30)));
                x3 = sub(x3, rdf(q.wrapping_add(0x34)));
                let mut t1 = rdf(q.wrapping_add(4));
                let mut t0 = rdf(q);
                x4 = sub(x4, rdf(q.wrapping_add(0x38)));
                t0 = mul(t0, x2);
                t1 = mul(t1, x3);
                t1 = add(t1, t0);
                t0 = rdf(q.wrapping_add(8));
                t0 = mul(t0, x4);
                t1 = add(t1, t0);
                fr[F_10] = t1.to_bits();
                t0 = x2;
                t0 = mul(t0, rdf(q.wrapping_add(0x10)));
                t1 = rdf(q.wrapping_add(0x14));
                t1 = mul(t1, x3);
                t1 = add(t1, t0);
                t0 = rdf(q.wrapping_add(0x18));
                t0 = mul(t0, x4);
                t1 = add(t1, t0);
                fr[F_14] = t1.to_bits();
                t0 = rdf(q.wrapping_add(0x20));
                t1 = rdf(q.wrapping_add(0x24));
                t0 = mul(t0, x2);
                t1 = mul(t1, x3);
                t1 = add(t1, t0);
                t0 = rdf(q.wrapping_add(0x28));
                t0 = mul(t0, x4);
                t1 = add(t1, t0);
                fr[F_18] = t1.to_bits();
                let info = rd32(rd32(this.wrapping_add(TASK_HIT)).wrapping_add(HIT_INFO));
                let w = rd16(info.wrapping_add(8));
                lf_checker_rt::callee_stdcall!(C_SINK, u32, target, at(F_10), at(F_40), w);
            }
            lf_checker_rt::callee_thiscall!(C_SEND3, u32, buf);
        }
        let mut timer = gid(G_TIMER);
        let w94 = rd32(this.wrapping_add(TASK_T0));
        if w94 != 0 && timer > w94 {
            lf_checker_rt::callee_thiscall!(C_INIT, u32, buf);
            lf_checker_rt::callee_thiscall!(C_SET_BOOL, u32, buf, gid(G_TAIL_BOOL), 0);
            lf_checker_rt::callee_thiscall!(
                C_SEND2, u32,
                rd32(target.wrapping_add(TARGET_MSG_TO)),
                gid(G_TAIL_SEND),
                buf
            );
            wr32(this.wrapping_add(TASK_T0), 0);
            lf_checker_rt::callee_thiscall!(C_SEND3, u32, buf);
            timer = gid(G_TIMER);
        }
        let w98 = rd32(this.wrapping_add(TASK_T1));
        if w98 == 0 {
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
            return 0;
        }
        if timer >= w98 {
            wr32(this.wrapping_add(TASK_T1), 0);
            wr32(this.wrapping_add(TASK_T2), 0);
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
            return 0;
        }
        if timer < rd32(this.wrapping_add(TASK_T2)) {
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
            return 0;
        }
        if rd8(target.wrapping_add(TARGET_FLAG)) != 0 {
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
            return 0;
        }
        let det = rd32(target.wrapping_add(TARGET_DET));
        fr[F_0C] = det;
        if det == 0 {
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
            return 0;
        }
        let ent = rd32(det.wrapping_add(0x25c));
        fr[F_3C] = ent;
        let gate: u32 = lf_checker_rt::callee_thiscall!(C_GATE, u32, ent);
        if gate & 0xff == 0 {
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
            return 0;
        }
        if rd32(ent.wrapping_add(0x1c)) != 0 {
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
            return 0;
        }
        // The original re-reads both frame slots here (not the registers).
        let dw = rd32(fr[F_0C].wrapping_add(0x20));
        lf_checker_rt::callee_thiscall!(C_TRAJ_A, u32, ent, target, dw, at(F_20), at(F_40));
        lf_checker_rt::callee_thiscall!(
            C_TRAJ_B, u32, fr[F_3C], target, dw, at(F_20), at(F_40), 0, 0, 0, 0, 0xbf80_0000
        );
        let wi: u32 = lf_checker_rt::callee_cdecl!(C_WINFO, u32, rd32(ent.wrapping_add(0x18)));
        wr32(
            this.wrapping_add(TASK_T2),
            rd32(wi.wrapping_add(0x8c)).wrapping_add(gid(G_TIMER)),
        );
        lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
        0
    }
});
