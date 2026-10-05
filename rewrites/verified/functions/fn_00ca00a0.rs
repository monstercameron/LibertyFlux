// original: 0x00ca00a0 ped_task_pose_fill (proposed)

/// Fill one pose-output record from a task matrix and an indexed entry.
///
/// `thiscall`: object in `ecx`, stack arguments `idx` (entry index) and
/// `tag` (an opaque selector passed to the helper). Returns the output
/// pointer. Layout read: `this+0x40` is the sub-object `S`; `S+0` points to
/// a table whose slot `0xA0` resolves the entry holder; `S+0x100` is the
/// fallback holder used when the resolver answers null; `holder+4` is the
/// group and `group+0x10` the entry array, one 64-byte entry per index.
/// The helper (callee 1) is called as `(S, tag)` giving the 12-float matrix
/// `M1` (offsets `0x00..0x38` with gaps at `0x0C/0x1C/0x2C`) and as
/// `(S, idx)` giving the output record. Resolution calls slot `0xA0` once
/// to test for null, then again plus slot `0xE0` of its answer on the
/// non-null path. The twelve output floats are fixed sums of products of
/// `M1` and entry floats evaluated in the original's order through scalar
/// SSE; slots `+0x0C`, `+0x1C`, `+0x2C` of the output are left untouched.
/// Edge cases: any `idx` scales by 64 with wraparound; null versus non-null
/// resolver answers select the holder path; NaN, infinities and denormals
/// flow through the arithmetic bit-exactly.
lf_checker_rt::export!(thiscall, rw_00ca00a0(this: u32, idx: u32, tag: u32) -> u32 {
    unsafe {
        const SUB: u32 = 0x40;
        const VT_RESOLVE: u32 = 0xA0;
        const VT_FETCH: u32 = 0xE0;
        const ALT_HOLDER: u32 = 0x100;
        const HOLDER_GROUP: u32 = 0x04;
        const GROUP_ENTRIES: u32 = 0x10;
        const ENTRY_STRIDE: u32 = 6;
        const HELPER: u32 = 1;

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
        unsafe fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let sub = rd32(this + SUB);
        let m1 = lf_checker_rt::callee_thiscall!(HELPER, u32, sub, tag);
        let m2 = lf_checker_rt::callee_thiscall!(HELPER, u32, sub, idx);
        let resolve: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(sub) + VT_RESOLVE) as usize);
        let holder = if resolve(sub) == 0 {
            rd32(sub + ALT_HOLDER)
        } else {
            let target = resolve(sub);
            let fetch: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(target) + VT_FETCH) as usize);
            fetch(target)
        };
        let group = rd32(holder + HOLDER_GROUP);
        let ent = rd32(group + GROUP_ENTRIES).wrapping_add(idx << ENTRY_STRIDE);
        let t1 = rdf(ent + 0x8); // 00CA010C load ENT+0x8
        let t2 = rdf(ent + 0x34); // 00CA0117 load ENT+0x34
        let t3 = rdf(ent + 0x4); // 00CA011C load ENT+0x4
        let t4 = rdf(m1 + 0x14); // 00CA0121 load M1+0x14
        let t5 = rdf(ent + 0x38); // 00CA012C load ENT+0x38
        let t6 = rdf(m1 + 0x4); // 00CA0137 load M1+0x4
        let t7 = rdf(ent + 0x0); // 00CA0142 load ENT+0x0
        let t8 = fmul(t6, t7); // 00CA0142 mulss xmm0, dword ptr [edi]
        let t9 = fmul(t4, t3); // 00CA0146 mulss xmm4, xmm5
        let t10 = rdf(ent + 0x10); // 00CA014A load ENT+0x10
        let t11 = rdf(ent + 0x14); // 00CA014F load ENT+0x14
        let t12 = fadd(t9, t8); // 00CA0154 addss xmm4, xmm0
        let t13 = rdf(m1 + 0x24); // 00CA0158 load M1+0x24
        let t14 = fmul(t13, t1); // 00CA015D mulss xmm0, dword ptr [edi + 8]
        let t15 = rdf(m1 + 0x18); // 00CA0168 load M1+0x18
        let t16 = fadd(t12, t14); // 00CA016D addss xmm4, xmm0
        let t17 = rdf(m1 + 0x8); // 00CA0171 load M1+0x8
        let t18 = fmul(t17, t7); // 00CA017C mulss xmm0, dword ptr [edi]
        let t19 = fmul(t15, t3); // 00CA0189 mulss xmm4, dword ptr [edi + 4]
        let t20 = rdf(ent + 0x18); // 00CA018E load ENT+0x18
        let t21 = rdf(ent + 0x24); // 00CA0193 load ENT+0x24
        let t22 = fadd(t19, t18); // 00CA0198 addss xmm4, xmm0
        let t23 = rdf(m1 + 0x28); // 00CA019C load M1+0x28
        let t24 = fmul(t23, t1); // 00CA01A1 mulss xmm0, dword ptr [edi + 8]
        let t25 = fmul(t15, t11); // 00CA01A6 mulss xmm5, xmm1
        let t26 = fadd(t22, t24); // 00CA01AA addss xmm4, xmm0
        let t27 = rdf(m1 + 0x10); // 00CA01AE load M1+0x10
        let t28 = fmul(t27, t11); // 00CA01B3 mulss xmm0, xmm1
        let t29 = rdf(m1 + 0x0); // 00CA01BD load M1+0x0
        let t30 = fmul(t29, t10); // 00CA01C4 mulss xmm6, xmm3
        let t31 = fadd(t30, t28); // 00CA01CE addss xmm6, xmm0
        let t32 = rdf(m1 + 0x20); // 00CA01D2 load M1+0x20
        let t33 = fmul(t32, t20); // 00CA01D7 mulss xmm0, xmm2
        let t34 = fadd(t31, t33); // 00CA01DB addss xmm6, xmm0
        let t35 = fmul(t6, t10); // 00CA01E5 mulss xmm0, xmm3
        let t36 = fmul(t4, t11); // 00CA01F4 mulss xmm6, xmm1
        let t37 = rdf(ent + 0x28); // 00CA01F8 load ENT+0x28
        let t38 = fadd(t36, t35); // 00CA01FD addss xmm6, xmm0
        let t39 = fmul(t13, t20); // 00CA0206 mulss xmm0, xmm2
        let t40 = fadd(t38, t39); // 00CA020A addss xmm6, xmm0
        let t41 = fmul(t17, t10); // 00CA0214 mulss xmm0, xmm3
        let t42 = rdf(ent + 0x20); // 00CA0223 load ENT+0x20
        let t43 = fadd(t25, t41); // 00CA0228 addss xmm5, xmm0
        let t44 = fmul(t23, t20); // 00CA0231 mulss xmm0, xmm2
        let t45 = fmul(t29, t42); // 00CA0235 mulss xmm4, xmm6
        let t46 = fadd(t43, t44); // 00CA0239 addss xmm5, xmm0
        let t47 = fmul(t27, t21); // 00CA0242 mulss xmm0, xmm7
        let t48 = fmul(t4, t21); // 00CA0246 mulss xmm3, xmm7
        let t49 = fadd(t45, t47); // 00CA024A addss xmm4, xmm0
        let t50 = fmul(t32, t37); // 00CA0253 mulss xmm0, xmm1
        let t51 = fadd(t49, t50); // 00CA0263 addss xmm4, xmm0
        let t52 = fmul(t6, t42); // 00CA026A mulss xmm0, xmm6
        let t53 = fadd(t48, t52); // 00CA026E addss xmm3, xmm0
        let t54 = fmul(t13, t37); // 00CA027C mulss xmm0, xmm1
        let t55 = rdf(ent + 0x30); // 00CA0280 load ENT+0x30
        let t56 = fmul(t6, t55); // 00CA0280 mulss xmm5, dword ptr [edi + 0x30]
        let t57 = fadd(t53, t54); // 00CA0285 addss xmm3, xmm0
        let t58 = fmul(t17, t42); // 00CA028F mulss xmm0, xmm6
        let t59 = fmul(t15, t21); // 00CA0298 mulss xmm2, xmm7
        let t60 = fadd(t59, t58); // 00CA02A1 addss xmm2, xmm0
        let t61 = fmul(t23, t37); // 00CA02A8 mulss xmm0, xmm1
        let t62 = fmul(t29, t55); // 00CA02B2 mulss xmm1, dword ptr [edi + 0x30]
        let t63 = fadd(t60, t61); // 00CA02B7 addss xmm2, xmm0
        let t64 = fmul(t27, t2); // 00CA02C0 mulss xmm0, dword ptr [edi + 0x34]
        let t65 = fmul(t23, t5); // 00CA02C5 mulss xmm6, dword ptr [esp + 0x18]
        let t66 = fadd(t62, t64); // 00CA02CB addss xmm1, xmm0
        let t67 = fmul(t32, t5); // 00CA02D2 mulss xmm0, dword ptr [edi + 0x38]
        let t68 = fmul(t32, t1); // 00CA02D7 mulss xmm7, dword ptr [esp + 0x28]
        let t69 = fadd(t66, t67); // 00CA02DD addss xmm1, xmm0
        let t70 = fmul(t4, t2); // 00CA02E6 mulss xmm0, dword ptr [esp + 0x14]
        let t71 = rdf(m1 + 0x30); // 00CA02EC load M1+0x30
        let t72 = fadd(t69, t71); // 00CA02EC addss xmm1, dword ptr [eax + 0x30]
        let t73 = fadd(t56, t70); // 00CA02F1 addss xmm5, xmm0
        let t74 = fmul(t13, t5); // 00CA02FA mulss xmm0, dword ptr [esp + 0x18]
        let t75 = fadd(t73, t74); // 00CA0300 addss xmm5, xmm0
        let t76 = fmul(t17, t55); // 00CA030A mulss xmm0, dword ptr [edi + 0x30]
        let t77 = rdf(m1 + 0x34); // 00CA030F load M1+0x34
        let t78 = fadd(t75, t77); // 00CA030F addss xmm5, dword ptr [eax + 0x34]
        let t79 = fmul(t15, t2); // 00CA031F mulss xmm0, dword ptr [esp + 0x14]
        let t80 = fadd(t76, t79); // 00CA0331 addss xmm5, xmm0
        let t81 = fadd(t80, t65); // 00CA0339 addss xmm0, xmm6
        let t82 = rdf(m1 + 0x38); // 00CA0349 load M1+0x38
        let t83 = fadd(t81, t82); // 00CA0349 addss xmm0, dword ptr [eax + 0x38]
        let t84 = fmul(t29, t7); // 00CA035A mulss xmm0, dword ptr [edi]
        // 00CA035E (an instruction of the original))
        // 00CA035F (an instruction of the original))
        let t85 = fmul(t27, t3); // 00CA036B mulss xmm0, dword ptr [esp + 0x1c]
        let outp = m2;
        let t86 = fadd(t84, t85); // 00CA037B addss xmm6, xmm0
        let t87 = fadd(t86, t68); // 00CA0383 addss xmm0, xmm7
        wrf(outp + 0x0, t87); // 00CA0393 store OUT+0x0
        wrf(outp + 0x4, t16); // 00CA039D store OUT+0x4
        wrf(outp + 0x8, t26); // 00CA03A8 store OUT+0x8
        wrf(outp + 0x10, t34); // 00CA03B3 store OUT+0x10
        wrf(outp + 0x14, t40); // 00CA03BE store OUT+0x14
        wrf(outp + 0x18, t46); // 00CA03C3 store OUT+0x18
        wrf(outp + 0x20, t51); // 00CA03C8 store OUT+0x20
        wrf(outp + 0x24, t57); // 00CA03CD store OUT+0x24
        wrf(outp + 0x28, t63); // 00CA03D2 store OUT+0x28
        wrf(outp + 0x34, t78); // 00CA03D7 store OUT+0x34
        wrf(outp + 0x30, t72); // 00CA03E1 store OUT+0x30
        wrf(outp + 0x38, t83); // 00CA03E6 store OUT+0x38
        m2
    }
});
