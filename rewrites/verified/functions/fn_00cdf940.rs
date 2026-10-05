// original: 0x00cdf940 unknown-peds-tasks-handler (proposed)

/// Ped-task handler that gates on task flags and calibrated distances, then
/// resolves a target object and publishes a result vector into the task.
///
/// `this` is the task object, `target` the ped it acts on. The task carries
/// flag words at `+0x60` and `+0x68` and receives the result in words
/// `+0x0..+0x3c`. Two sampler callees fill frame scratch with candidate
/// vectors; the flag bits select up to two distance tests (squared length
/// against file constants), each able to keep the task alive, while flag
/// bit 0 kills it outright. A mixer callee then combines a second sample
/// set, a locator callee resolves an object pointer (or null), and a
/// trajectory callee plus a weapon-info callee shape the working set.
/// Lazily initialised matrices feed two 3x3 transforms in the original's
/// order; a final dot product against the original target's matrix picks one
/// of two result layouts, both ending in a sink call. Every path returns 0.
///
/// The sampler/tjectory callees' rewritten frame words are the only
/// callee-written memory the original reads back; the lazy matrix pointer
/// likewise. Float order is the original's (row sums add x, then y, then z).
/// NaN inputs follow the original's unordered-compare branches. Original:
/// thiscall, one stack word, byte return always 0 (callee pops the
/// argument; every path returns 0).
lf_checker_rt::export!(thiscall, rw_00cdf940(this: u32, target: u32) -> u32 {
    unsafe {
        const TASK_FLAGS: u32 = 0x60;
        const TASK_SLOT: u32 = 0x68;
        const TARGET_MSG_TO: u32 = 0x7b4;
        const TARGET_MAT: u32 = 0x20;
        const OBJ_MAT: u32 = 0x20;
        const OBJ_FLAGS: u32 = 0x28;
        const OBJ_LIVE: u32 = 0x24;
        const OBJ_KIND: u32 = 0x1e2;
        const OBJ_STATE: u32 = 0x22b;
        const FLAG_B1: u32 = 1 << 1;
        const FLAG_B1_HI: u32 = 1 << 17;
        const FLAG_B2: u32 = 1 << 2;
        const FLAG_B2_M: u32 = 0x70000;
        const FLAG_KILL: u32 = 1;
        const G_D1: u32 = 0x0105_2220;
        const G_D2: u32 = 0x0105_2224;
        const G_MIX: u32 = 0x0105_2244;
        const G_PUSH: u32 = 0x0105_2240;
        const G_BIAS: u32 = 0x0171_d768;
        const G_LOCATOR_CTX: u32 = 0x012b_9c78;
        const C_SAMP_A1: u32 = 1;
        const C_SAMP_A2: u32 = 2;
        const C_SAMP_B: u32 = 3;
        const C_MIX: u32 = 4;
        const C_LOC: u32 = 5;
        const C_TRAJ: u32 = 6;
        const C_MK_A: u32 = 7;
        const C_MK_B: u32 = 8;
        const C_KEEP: u32 = 9;
        const C_LINK: u32 = 10;
        const C_SINK: u32 = 11;
        const F_0C: usize = 0x0c / 4;
        const F_10: usize = 0x10 / 4;
        const F_28: usize = 0x28 / 4;
        const F_30: usize = 0x30 / 4;
        const F_3C: usize = 0x3c / 4;
        const F_40: usize = 0x40 / 4;
        const F_50: usize = 0x50 / 4;
        const F_54: usize = 0x54 / 4;
        const F_58: usize = 0x58 / 4;
        const F_60: usize = 0x60 / 4;
        const F_64: usize = 0x64 / 4;
        const F_68: usize = 0x68 / 4;
        const F_6C: usize = 0x6c / 4;
        const F_70: usize = 0x70 / 4;
        const F_74: usize = 0x74 / 4;
        const F_78: usize = 0x78 / 4;
        const F_80: usize = 0x80 / 4;
        const F_90: usize = 0x90 / 4;
        const F_94: usize = 0x94 / 4;
        const F_98: usize = 0x98 / 4;
        const F_A0: usize = 0xa0 / 4;
        const F_A4: usize = 0xa4 / 4;
        const F_A8: usize = 0xa8 / 4;
        const F_F0: usize = 0xf0 / 4;
        const F_F4: usize = 0xf4 / 4;
        const F_F8: usize = 0xf8 / 4;
        const F_110: usize = 0x110 / 4;
        const F_114: usize = 0x114 / 4;
        const F_118: usize = 0x118 / 4;
        const F_E0: usize = 0xe0 / 4;
        const F_120: usize = 0x120 / 4;

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
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
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
        let at = |w: usize| frame.wrapping_add((w * 4) as u32);
        macro_rules! frd {
            ($w:expr) => {
                f32::from_bits(fr[$w])
            };
        }
        let orig = target;

        lf_checker_rt::callee_thiscall!(C_SAMP_A1, u32, target, at(F_120), 0);
        fr[F_60] = 0;
        fr[F_64] = 0;
        fr[F_68] = 0;
        lf_checker_rt::callee_thiscall!(C_SAMP_B, u32, target, at(F_60), at(F_50), 0, 0);
        lf_checker_rt::callee_thiscall!(C_SAMP_A2, u32, target, at(F_E0), 0x36a1);
        fr[F_60] = 0;
        fr[F_64] = 0;
        fr[F_68] = 0;
        lf_checker_rt::callee_thiscall!(C_SAMP_B, u32, target, at(F_70), at(F_50), 0, 0x0a);

        let mut x5 = frd!(F_68);
        let mut x3 = frd!(F_64);
        let mut x4 = frd!(F_60);
        let x6 = frd!(F_78);
        let x7 = frd!(F_74);
        let mut x0 = frd!(F_70);
        let flags = rd32(this.wrapping_add(TASK_FLAGS));
        let mut dl = 0u32;
        if flags & FLAG_B1 != 0 && flags & FLAG_B1_HI != 0 {
            let mut x1 = x4;
            x1 = add(x1, x0);
            let mut x2 = x3;
            x2 = add(x2, x7);
            x0 = x5;
            x0 = add(x0, x6);
            x1 = mul(x1, x1);
            x2 = mul(x2, x2);
            x0 = mul(x0, x0);
            x2 = add(x2, x1);
            x2 = add(x2, x0);
            let mut t = f32::from_bits(gid(G_D1));
            t = mul(t, t);
            dl = if x2 > t { 1 } else { 0 };
        }
        let mut cont = dl != 0;
        if flags & FLAG_B2 != 0 && flags & FLAG_B2_M != 0 {
            x4 = add(x4, x0);
            let mut t = f32::from_bits(gid(G_D2));
            x3 = add(x3, x7);
            x5 = add(x5, x6);
            t = mul(t, t);
            x4 = mul(x4, x4);
            x3 = mul(x3, x3);
            x5 = mul(x5, x5);
            x3 = add(x3, x4);
            x3 = add(x3, x5);
            if x3 > t {
                cont = true;
            }
        }
        if !cont {
            return 0;
        }
        if flags & FLAG_KILL != 0 {
            return 0;
        }
        let c = f32::from_bits(gid(G_MIX));
        let mut m4 = frd!(F_118);
        let mut m3 = frd!(F_F0);
        let mut m1 = frd!(F_F4);
        let mut m2 = frd!(F_F8);
        m4 = add(m4, f32::from_bits(gid(G_BIAS)));
        m3 = mul(m3, c);
        m1 = mul(m1, c);
        m3 = add(m3, frd!(F_110));
        m2 = mul(m2, c);
        m1 = add(m1, frd!(F_114));
        m2 = add(m2, m4);
        fr[F_50] = m3.to_bits();
        fr[F_54] = m1.to_bits();
        fr[F_58] = m2.to_bits();
        lf_checker_rt::callee_thiscall!(C_MIX, u32, at(F_80));
        let pushed = f32::from_bits(gid(G_PUSH));
        let ans: u32 = lf_checker_rt::callee_thiscall!(
            C_LOC, u32, gid(G_LOCATOR_CTX), at(F_50), pushed.to_bits(), at(F_80),
            rd32(target.wrapping_add(TARGET_MSG_TO)), 0x8e, 0xffff_ffff, 7, 1, 0
        );
        if ans == 0 {
            return 0;
        }
        fr[F_10] = fr[F_90];
        fr[F_0C] = fr[F_94];
        fr[F_28] = fr[F_98];
        fr[F_40] = fr[F_A0];
        fr[F_3C] = fr[F_A4];
        fr[F_30] = fr[F_A8];
        let mut obj: u32 =
            lf_checker_rt::callee_cdecl!(C_TRAJ, u32, fr[F_80]);
        let of = rd32(obj.wrapping_add(OBJ_FLAGS));
        let sel = (of >> 6) & 0xf;
        if sel <= 1 || sel >= 5 {
            obj = 0;
        } else {
            if rd32(obj.wrapping_add(OBJ_MAT)) == 0 {
                lf_checker_rt::callee_thiscall!(C_MK_A, u32, obj);
                lf_checker_rt::callee_thiscall!(
                    C_MK_B, u32, obj.wrapping_add(0x10),
                    rd32(obj.wrapping_add(OBJ_MAT))
                );
            }
            let mut q = rd32(obj.wrapping_add(OBJ_MAT));
            let mut t0 = frd!(F_10);
            t0 = sub(t0, rdf(q.wrapping_add(0x30)));
            let mut t4 = rdf(q.wrapping_add(4));
            let mut t3 = t0;
            t0 = frd!(F_0C);
            t0 = sub(t0, rdf(q.wrapping_add(0x34)));
            let mut t1 = t0;
            t0 = frd!(F_28);
            t0 = sub(t0, rdf(q.wrapping_add(0x38)));
            t4 = mul(t4, t1);
            let mut t2 = t0;
            t0 = t3;
            t0 = mul(t0, rdf(q));
            t4 = add(t4, t0);
            t0 = rdf(q.wrapping_add(8));
            t0 = mul(t0, t2);
            t4 = add(t4, t0);
            t0 = rdf(q.wrapping_add(0x10));
            t0 = mul(t0, t3);
            t3 = mul(t3, rdf(q.wrapping_add(0x20)));
            fr[F_10] = t4.to_bits();
            t4 = rdf(q.wrapping_add(0x14));
            t4 = mul(t4, t1);
            t4 = add(t4, t0);
            t0 = rdf(q.wrapping_add(0x18));
            t0 = mul(t0, t2);
            t4 = add(t4, t0);
            t0 = rdf(q.wrapping_add(0x28));
            t0 = mul(t0, t2);
            fr[F_0C] = t4.to_bits();
            t4 = rdf(q.wrapping_add(0x24));
            t4 = mul(t4, t1);
            t4 = add(t4, t3);
            t4 = add(t4, t0);
            fr[F_28] = t4.to_bits();
            if q == 0 {
                lf_checker_rt::callee_thiscall!(C_MK_A, u32, obj);
                lf_checker_rt::callee_thiscall!(
                    C_MK_B, u32, obj.wrapping_add(0x10),
                    rd32(obj.wrapping_add(OBJ_MAT))
                );
            }
            q = rd32(obj.wrapping_add(OBJ_MAT));
            let mut u5 = frd!(F_3C);
            let mut u0 = rdf(q);
            let mut u4 = frd!(F_40);
            let mut u3 = rdf(q.wrapping_add(4));
            let mut u6 = frd!(F_30);
            let mut u2 = rdf(q.wrapping_add(0x14));
            u0 = mul(u0, u4);
            let mut u1 = rdf(q.wrapping_add(0x24));
            u3 = mul(u3, u5);
            u2 = mul(u2, u5);
            u3 = add(u3, u0);
            u0 = rdf(q.wrapping_add(8));
            u0 = mul(u0, u6);
            u1 = mul(u1, u5);
            u3 = add(u3, u0);
            u0 = u4;
            u0 = mul(u0, rdf(q.wrapping_add(0x10)));
            fr[F_40] = u3.to_bits();
            u2 = add(u2, u0);
            u0 = rdf(q.wrapping_add(0x18));
            u0 = mul(u0, u6);
            u2 = add(u2, u0);
            u0 = rdf(q.wrapping_add(0x20));
            u0 = mul(u0, u4);
            fr[F_3C] = u2.to_bits();
            u1 = add(u1, u0);
            u0 = rdf(q.wrapping_add(0x28));
            u0 = mul(u0, u6);
            u1 = add(u1, u0);
            fr[F_30] = u1.to_bits();
            let g = rd32(obj.wrapping_add(OBJ_FLAGS)) & 0x3c0;
            if g != 0x100 {
                // fall through to live check
            } else if rd8(obj.wrapping_add(OBJ_STATE)) != 0 {
                obj = 0;
            }
            if obj != 0 {
                if rd8(obj.wrapping_add(OBJ_LIVE)) & 1 == 0 {
                    return 0;
                }
                if rd8(obj.wrapping_add(OBJ_KIND)) & 0xf >= 2 {
                    return 0;
                }
            }
        }
        let slot = rd32(this.wrapping_add(TASK_SLOT));
        if slot != 0 {
            lf_checker_rt::callee_thiscall!(C_KEEP, u32, slot, this.wrapping_add(TASK_SLOT));
        }
        wr32(this.wrapping_add(TASK_SLOT), obj);
        if obj != 0 {
            lf_checker_rt::callee_thiscall!(C_LINK, u32, obj, this.wrapping_add(TASK_SLOT));
        }
        let q2 = rd32(orig.wrapping_add(TARGET_MAT));
        let mut v0 = frd!(F_94);
        v0 = sub(v0, rdf(q2.wrapping_add(0x34)));
        let mut v3 = frd!(F_90);
        v3 = sub(v3, rdf(q2.wrapping_add(0x30)));
        let mut v1 = rdf(q2.wrapping_add(4));
        let mut v2 = frd!(F_98);
        v2 = sub(v2, rdf(q2.wrapping_add(0x38)));
        v1 = mul(v1, v0);
        v0 = rdf(q2);
        v0 = mul(v0, v3);
        v3 = frd!(F_28);
        v1 = add(v1, v0);
        v0 = rdf(q2.wrapping_add(8));
        v0 = mul(v0, v2);
        v2 = frd!(F_0C);
        v1 = add(v1, v0);
        let w1 = frd!(F_10);
        let w0 = frd!(F_6C);
        if !(v1 > 0.0) {
            wr32(this.wrapping_add(8), 0);
            wr32(this.wrapping_add(4), 0);
            wr32(this.wrapping_add(0), 0);
            wr32(this.wrapping_add(0x18), 0);
            wr32(this.wrapping_add(0x14), 0);
            wr32(this.wrapping_add(0x10), 0);
            wrf(this.wrapping_add(0x20), w1);
            wrf(this.wrapping_add(0x24), v2);
            wrf(this.wrapping_add(0x28), v3);
            wrf(this.wrapping_add(0x2c), w0);
            wrf(this.wrapping_add(0x30), w1);
            wrf(this.wrapping_add(0x34), v2);
            wrf(this.wrapping_add(0x38), v3);
            wrf(this.wrapping_add(0x3c), w0);
            lf_checker_rt::callee_thiscall!(C_SINK, u32, this, orig);
            return 0;
        }
        wrf(this.wrapping_add(0x0c), w0);
        wrf(this.wrapping_add(0), w1);
        wrf(this.wrapping_add(4), v2);
        wrf(this.wrapping_add(8), v3);
        wrf(this.wrapping_add(0x10), frd!(F_40));
        wrf(this.wrapping_add(0x14), frd!(F_3C));
        wrf(this.wrapping_add(0x18), frd!(F_30));
        wrf(this.wrapping_add(0x1c), w0);
        wr32(this.wrapping_add(0x28), 0);
        wr32(this.wrapping_add(0x24), 0);
        wr32(this.wrapping_add(0x20), 0);
        lf_checker_rt::callee_thiscall!(C_SINK, u32, this, orig);
        wr32(this.wrapping_add(0x38), 0);
        wr32(this.wrapping_add(0x34), 0);
        wr32(this.wrapping_add(0x30), 0);
        0
    }
});
