// original: 0x00c96c00 task_dual_slot_update (proposed)
//
// Updates two task slots (A and B) of a ped task object from a subject
// record, blending matrices through scripted engine helpers. `thiscall`:
// `this` points at the task, whose word at +0x40 points at the subject.
// Flags at +0xf0 select the work: bit 2 runs slot A, bit 3 runs slot B,
// bit 1 adds two extra resolver calls in slot A, and bit 0 is cleared on
// exit. Floats at +0xf4 (blend) and +0xf8 (scale) weight the slot math.
// Returns whatever the last helper call returned, or zero when no slot ran (the prologue masks the
// flags by 8 into the return register).
//
// Each slot resolves two small ids through a global object table indexed
// by a signed word at subject+0x2e, calling each resolved object's slot
// at +0x38; feeds those ids to a pair-lookup helper (twice) yielding a
// matrix pointer and a vector-struct pointer; normalises the struct's
// leading vec3 (zero stays zero), crosses it with the global up vector,
// normalises again, scales by the slot scale and hands the triple plus
// scale to a pose helper; copies nine words (offsets 0/4/8/10/14/18/20/
// 24/28) of the struct into a target from a target-lookup helper; then
// left-multiplies the target's three rows by the matrix. Each slot ends
// by querying the subject's +0xa0 slot (null means use the fallback at
// subject+0x100, otherwise one more +0xa0 call and the result's +0xe0
// slot) and passing base[4]+id*0xe0 to a final helper.
//
// Two original behaviours are dead and not reproduced: a stack word the
// original reads before ever writing is only ever written back to itself
// (below the incoming stack pointer, so unobserved), and a scratch
// register pushed ahead of the pose helper is overwritten in its slot by
// the scale argument before the call. The float operation order, including
// which operand of each multiply comes first and the differing orders
// between the two slots' matrix code, is the original's.
lf_checker_rt::export!(thiscall, rw_00c96c00(this: u32) -> u32 {
    unsafe {
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
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
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
        unsafe fn table_obj(sub: u32) -> u32 {
            unsafe {
                let w = (sub.wrapping_add(0x2e) as *const u16).read_unaligned();
                let idx = (w as i16) as i32 as u32;
                let base = lf_checker_rt::relocated(0x01295cd8);
                (base.wrapping_add(idx.wrapping_mul(4)) as *const u32).read_unaligned()
            }
        }
        #[inline(always)]
        unsafe fn vcall38(obj: u32, arg: u32) -> u32 {
            unsafe {
                let vt = (obj as *const u32).read_unaligned();
                let slot = (vt.wrapping_add(0x38) as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(obj, arg)
            }
        }
        #[inline(always)]
        unsafe fn vcall_a0(obj: u32) -> u32 {
            unsafe {
                let vt = (obj as *const u32).read_unaligned();
                let slot = (vt.wrapping_add(0xa0) as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(obj)
            }
        }
        #[inline(always)]
        unsafe fn vcall_e0(obj: u32) -> u32 {
            unsafe {
                let vt = (obj as *const u32).read_unaligned();
                let slot = (vt.wrapping_add(0xe0) as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(obj)
            }
        }
        #[inline(always)]
        unsafe fn copy9(dst: u32, src: u32) {
            unsafe {
                for off in [0u32, 4, 8, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28] {
                    wr32(dst.wrapping_add(off), rd32(src.wrapping_add(off)));
                }
            }
        }
        const SUB: u32 = 0x40;
        const SUB_FLAGS: u32 = 0x29c;
        const SUB_ALT: u32 = 0x100;
        const FLAGS: u32 = 0xf0;
        const BLEND: u32 = 0xf4;
        const SCALE: u32 = 0xf8;
        const FLAG_EXTRA: u32 = 2;
        const FLAG_A: u32 = 4;
        const FLAG_B: u32 = 8;
        let half: f32 = (lf_checker_rt::global::<f32>(0x00fe8830) as *const f32).read();
        let one: f32 = (lf_checker_rt::global::<f32>(0x00fe88e8) as *const f32).read();
        let gx: f32 = (lf_checker_rt::global::<f32>(0x0110db70) as *const f32).read();
        let gy: f32 = (lf_checker_rt::global::<f32>(0x0110db74) as *const f32).read();
        let gz: f32 = (lf_checker_rt::global::<f32>(0x0110db78) as *const f32).read();
        let norm_or_zero = |len2: f32| -> f32 {
            if len2 == 0.0 {
                0.0
            } else {
                core::hint::black_box(one) / core::hint::black_box(len2.sqrt())
            }
        };
        let sub = rd32(this.wrapping_add(SUB));
        wr32(
            sub.wrapping_add(SUB_FLAGS),
            rd32(sub.wrapping_add(SUB_FLAGS)) | 0x300000,
        );
        let f = rd32(this.wrapping_add(FLAGS));
        let run_a = f & FLAG_A != 0;
        let run_b = f & FLAG_B != 0;
        let f4 = rdf(this.wrapping_add(BLEND));
        let f8v = rdf(this.wrapping_add(SCALE));
        let mut x1 = 0.0f32;
        if run_a {
            x1 = f4;
        }
        let mut blend0 = 0.0f32;
        if run_b {
            blend0 = f4;
        }
        if run_a && run_b {
            x1 = fmul(f4, half);
            blend0 = fsub(f4, x1);
        }
        let scale_a = fmul(f8v, x1);
        // eax from here on: the prologue masked the flags with 8, so a
        // trial that runs neither phase returns 0, not the flags word.
        let mut ret = f & FLAG_B;
        if run_a {
            let r1 = vcall38(table_obj(sub), 0x0a);
            let mut ebx_small = r1;
            let r2 = vcall38(table_obj(sub), 0x0b);
            let mut esi_small = r2;
            if f & FLAG_EXTRA != 0 {
                ebx_small = vcall38(table_obj(sub), 0x0e);
                esi_small = vcall38(table_obj(sub), 0x0b);
            }
            let m1 = lf_checker_rt::callee_thiscall!(1, u32, sub, ebx_small);
            let m2 = lf_checker_rt::callee_thiscall!(1, u32, sub, esi_small);
            let vx = rdf(m2);
            let vy = rdf(m2.wrapping_add(4));
            let vz = rdf(m2.wrapping_add(8));
            let xx = fmul(vx, vx);
            let yy = fmul(vy, vy);
            let mut len2a = fadd(xx, yy);
            let zz = fmul(vz, vz);
            len2a = fadd(len2a, zz);
            let na = norm_or_zero(len2a);
            let sxa = fmul(vx, na);
            let sya = fmul(vy, na);
            let sza = fmul(vz, na);
            let t_a = fmul(gz, sya);
            let u_a = fmul(sya, gx);
            let mut wxa = t_a;
            let v_a = fmul(gy, sza);
            let w_a = fmul(sza, gx);
            let z_a = fmul(gz, sxa);
            wxa = fsub(wxa, v_a);
            let wya = fsub(w_a, z_a);
            let t2_a = fmul(gy, sxa);
            let wza = fsub(t2_a, u_a);
            let wy2 = fmul(wya, wya);
            let wx2 = fmul(wxa, wxa);
            let mut wlen2a = fadd(wy2, wx2);
            let wz2 = fmul(wza, wza);
            wlen2a = fadd(wlen2a, wz2);
            let ma = norm_or_zero(wlen2a);
            let tx = fmul(wxa, ma);
            let ty = fmul(wya, ma);
            let tz = fmul(wza, ma);
            let mut triple = [tx.to_bits(), ty.to_bits(), tz.to_bits()];
            lf_checker_rt::callee_thiscall!(2, u32, m2, triple.as_mut_ptr() as u32, scale_a.to_bits());
            let tgt = lf_checker_rt::callee_thiscall!(3, u32, sub, esi_small);
            copy9(tgt, m2);
            let t0 = rdf(tgt);
            let t1 = rdf(tgt.wrapping_add(4));
            let t2 = rdf(tgt.wrapping_add(8));
            let m0 = rdf(m1);
            let m1v = rdf(m1.wrapping_add(4));
            let m2v = rdf(m1.wrapping_add(8));
            let m3v = rdf(m1.wrapping_add(0x10));
            let m4v = rdf(m1.wrapping_add(0x14));
            let m5v = rdf(m1.wrapping_add(0x18));
            let m6v = rdf(m1.wrapping_add(0x20));
            let m7v = rdf(m1.wrapping_add(0x24));
            let m8v = rdf(m1.wrapping_add(0x28));
            let mut o0 = fmul(m0, t0);
            let q0 = fmul(t1, m1v);
            o0 = fadd(o0, q0);
            let q1 = fmul(t2, m2v);
            o0 = fadd(o0, q1);
            let mut o1 = fmul(m3v, t0);
            let q2 = fmul(m4v, t1);
            o1 = fadd(o1, q2);
            let q3 = fmul(m5v, t2);
            o1 = fadd(o1, q3);
            let mut o2 = fmul(t0, m6v);
            let q4 = fmul(t1, m7v);
            o2 = fadd(o2, q4);
            let q5 = fmul(t2, m8v);
            o2 = fadd(o2, q5);
            wrf(tgt, o0);
            wrf(tgt.wrapping_add(4), o1);
            wrf(tgt.wrapping_add(8), o2);
            let t3 = rdf(tgt.wrapping_add(0x10));
            let t4 = rdf(tgt.wrapping_add(0x14));
            let t5 = rdf(tgt.wrapping_add(0x18));
            let mut o3 = fmul(m0, t3);
            let r0 = fmul(t4, m1v);
            o3 = fadd(o3, r0);
            let r1 = fmul(t5, m2v);
            o3 = fadd(o3, r1);
            let mut o4 = fmul(t3, m3v);
            let r2 = fmul(t4, m4v);
            o4 = fadd(o4, r2);
            let r3 = fmul(t5, m5v);
            o4 = fadd(o4, r3);
            let mut o5 = fmul(t3, m6v);
            let r4 = fmul(t4, m7v);
            o5 = fadd(o5, r4);
            let r5 = fmul(t5, m8v);
            o5 = fadd(o5, r5);
            wrf(tgt.wrapping_add(0x10), o3);
            wrf(tgt.wrapping_add(0x14), o4);
            wrf(tgt.wrapping_add(0x18), o5);
            let t6 = rdf(tgt.wrapping_add(0x20));
            let t7 = rdf(tgt.wrapping_add(0x24));
            let t8 = rdf(tgt.wrapping_add(0x28));
            let mut o6 = fmul(t7, m1v);
            let s0 = fmul(m0, t6);
            o6 = fadd(o6, s0);
            let s1 = fmul(t8, m2v);
            o6 = fadd(o6, s1);
            let mut o7 = fmul(t6, m3v);
            let s2 = fmul(t7, m4v);
            o7 = fadd(o7, s2);
            let s3 = fmul(t8, m5v);
            o7 = fadd(o7, s3);
            let mut o8 = fmul(t6, m6v);
            let s4 = fmul(t7, m7v);
            o8 = fadd(o8, s4);
            let s5 = fmul(t8, m8v);
            o8 = fadd(o8, s5);
            wrf(tgt.wrapping_add(0x20), o6);
            wrf(tgt.wrapping_add(0x24), o7);
            wrf(tgt.wrapping_add(0x28), o8);
            let p = vcall_a0(sub);
            let ecx_a = if p != 0 {
                let q = vcall_a0(sub);
                vcall_e0(q)
            } else {
                rd32(sub.wrapping_add(SUB_ALT))
            };
            let base = rd32(ecx_a.wrapping_add(4));
            let arg_a = esi_small
                .wrapping_mul(0xe0)
                .wrapping_add(rd32(base));
            ret = lf_checker_rt::callee_thiscall!(6, u32, ecx_a, arg_a);
        }
        let scale_b = fmul(f8v, blend0);
        if run_b {
            let r5 = vcall38(table_obj(sub), 0x03);
            let esi5 = r5;
            let r6 = vcall38(table_obj(sub), 0x04);
            let ebx6 = r6;
            let m3 = lf_checker_rt::callee_thiscall!(1, u32, sub, esi5);
            let m4 = lf_checker_rt::callee_thiscall!(1, u32, sub, ebx6);
            let vx = rdf(m4);
            let vy = rdf(m4.wrapping_add(4));
            let vz = rdf(m4.wrapping_add(8));
            let yy = fmul(vy, vy);
            let xx = fmul(vx, vx);
            let mut len2b = fadd(yy, xx);
            let zz = fmul(vz, vz);
            len2b = fadd(len2b, zz);
            let nb = norm_or_zero(len2b);
            let syb = fmul(vy, nb);
            let szb = fmul(vz, nb);
            let sxb = fmul(vx, nb);
            let mut wyb = fmul(gz, syb);
            let mut wxb = fmul(gx, szb);
            let q_b = fmul(gx, syb);
            let r_b = fmul(gz, sxb);
            let s_b = fmul(gy, szb);
            wxb = fsub(wxb, r_b);
            let mut wzb = fmul(gy, sxb);
            wyb = fsub(wyb, s_b);
            wzb = fsub(wzb, q_b);
            let x2 = fmul(wxb, wxb);
            let y2 = fmul(wyb, wyb);
            let mut wlen2b = fadd(x2, y2);
            let z2 = fmul(wzb, wzb);
            wlen2b = fadd(wlen2b, z2);
            let mb = norm_or_zero(wlen2b);
            let tyb = fmul(wyb, mb);
            let txb = fmul(wxb, mb);
            let tzb = fmul(wzb, mb);
            let mut triple_b = [tyb.to_bits(), txb.to_bits(), tzb.to_bits()];
            lf_checker_rt::callee_thiscall!(2, u32, m4, triple_b.as_mut_ptr() as u32, scale_b.to_bits());
            let tgt = lf_checker_rt::callee_thiscall!(3, u32, sub, ebx6);
            copy9(tgt, m4);
            let t0 = rdf(tgt);
            let t1 = rdf(tgt.wrapping_add(4));
            let t2 = rdf(tgt.wrapping_add(8));
            let m0 = rdf(m3);
            let m1v = rdf(m3.wrapping_add(4));
            let m2v = rdf(m3.wrapping_add(8));
            let m3v = rdf(m3.wrapping_add(0x10));
            let m4v = rdf(m3.wrapping_add(0x14));
            let m5v = rdf(m3.wrapping_add(0x18));
            let m6v = rdf(m3.wrapping_add(0x20));
            let m7v = rdf(m3.wrapping_add(0x24));
            let m8v = rdf(m3.wrapping_add(0x28));
            let mut o0 = fmul(t0, m0);
            let q0 = fmul(t1, m1v);
            o0 = fadd(o0, q0);
            let q1 = fmul(t2, m2v);
            o0 = fadd(o0, q1);
            let mut o1 = fmul(t0, m3v);
            let q2 = fmul(t1, m4v);
            o1 = fadd(o1, q2);
            let q3 = fmul(t2, m5v);
            o1 = fadd(o1, q3);
            let mut o2 = fmul(t0, m6v);
            let q4 = fmul(t1, m7v);
            o2 = fadd(o2, q4);
            let q5 = fmul(t2, m8v);
            o2 = fadd(o2, q5);
            wrf(tgt, o0);
            wrf(tgt.wrapping_add(4), o1);
            wrf(tgt.wrapping_add(8), o2);
            let t3 = rdf(tgt.wrapping_add(0x10));
            let t4 = rdf(tgt.wrapping_add(0x14));
            let t5 = rdf(tgt.wrapping_add(0x18));
            let mut o3 = fmul(t3, m0);
            let r0 = fmul(t4, m1v);
            o3 = fadd(o3, r0);
            let r1 = fmul(t5, m2v);
            o3 = fadd(o3, r1);
            let mut o4 = fmul(t3, m3v);
            let r2 = fmul(t4, m4v);
            o4 = fadd(o4, r2);
            let r3 = fmul(t5, m5v);
            o4 = fadd(o4, r3);
            let mut o5 = fmul(t3, m6v);
            let r4 = fmul(t4, m7v);
            o5 = fadd(o5, r4);
            let r5 = fmul(t5, m8v);
            o5 = fadd(o5, r5);
            wrf(tgt.wrapping_add(0x10), o3);
            wrf(tgt.wrapping_add(0x14), o4);
            wrf(tgt.wrapping_add(0x18), o5);
            let t6 = rdf(tgt.wrapping_add(0x20));
            let t7 = rdf(tgt.wrapping_add(0x24));
            let t8 = rdf(tgt.wrapping_add(0x28));
            let mut o6 = fmul(t6, m0);
            let s0 = fmul(t7, m1v);
            o6 = fadd(o6, s0);
            let s1 = fmul(t8, m2v);
            o6 = fadd(o6, s1);
            let mut o7 = fmul(t6, m3v);
            let s2 = fmul(t7, m4v);
            o7 = fadd(o7, s2);
            let s3 = fmul(t8, m5v);
            o7 = fadd(o7, s3);
            let mut o8 = fmul(t6, m6v);
            let s4 = fmul(t7, m7v);
            o8 = fadd(o8, s4);
            let s5 = fmul(t8, m8v);
            o8 = fadd(o8, s5);
            wrf(tgt.wrapping_add(0x20), o6);
            wrf(tgt.wrapping_add(0x24), o7);
            wrf(tgt.wrapping_add(0x28), o8);
            let p = vcall_a0(sub);
            let ecx_b = if p != 0 {
                let q = vcall_a0(sub);
                vcall_e0(q)
            } else {
                rd32(sub.wrapping_add(SUB_ALT))
            };
            let base = rd32(ecx_b.wrapping_add(4));
            let arg_b = ebx6.wrapping_mul(0xe0).wrapping_add(rd32(base));
            ret = lf_checker_rt::callee_thiscall!(6, u32, ecx_b, arg_b);
        }
        wr32(this.wrapping_add(FLAGS), f & 0xfffffffe);
        ret
    }
});
