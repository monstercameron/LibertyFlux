// original: 0x009BE040 spawn_effect_slot_alloc (proposed)

/// Claim a projectile/effect slot from the game's fixed slot table and fill
/// it in, choosing the initial direction by a type-selected computation.
///
/// Arguments (cdecl, eight stack words): `owner` (object the effect is
/// attached to, may be null), `kind` (small type tag, compared against
/// `0x26`), `aux` (passed to the setup helper), `target` (pointer to a
/// three-float aim point), `origin` (pointer to a three-float start point),
/// `extra` (stored into the slot, may be null), `flags` (only the low byte
/// is read, stored into the slot), `seed_obj` (object stored into the slot,
/// may be null; when null a fresh one is built through two helpers).
/// Returns 1 with a slot claimed and filled, or 0 with nothing stored.
///
/// Behaviour in order. Three global gates (a state word must differ from 1,
/// two epoch words must be equal, a mode word must differ from `0x12`) each
/// return 0 when tripped. A lookup helper resolves `kind` to an object (null
/// returns 0); a word at `+0xA0` of it selects the direction computation
/// (values 0-4, anything else returns 0) and a following helper must answer
/// nonzero. When the flags byte is zero, three small helpers run and their
/// answers (with `seed_obj`/`extra`) gate a parameter-block call. A setup
/// helper then runs against a scratch vector. The direction cases: 0/1/2
/// share differenced-vector code (case 1 adds a lookup result to the base),
/// case 3 runs a three-call vector pipeline or a two-call fallback depending
/// on a word at `+0xA4` of the lookup object, case 4 differences against the
/// aim point with a biased base. The tail scans the 32-entry slot table
/// (entries of `0xD0` bytes) for the first entry whose first word is zero (a
/// full table returns 0), stores the object, links the owner, fills the
/// slot's numeric fields from globals and computed vectors, runs the
/// per-type finish calls, allocates an optional child object, and finishes
/// through two virtual calls.
///
/// Calling convention notes. Outgoing calls use the intercepted-callee
/// macros with the original's conventions; the two stack-cookie checks use a
/// preserving callee id. Several callees take pointers to this function's
/// scratch vectors; those are real locals so the stub-written words land in
/// them. Two frame words the original never writes (an unused vector fourth
/// component and a scratch word feeding one helper's object register) read as
/// zero under the contract's zero stack fill. The type-3 finish block reads
/// a slot word that is scripted nonzero (its zero path would run the tail
/// with a shifted frame and fail the cookie check, which cannot complete).
/// On the route where the `+0xA4` word is zero the finish block dereferences
/// a 16-bit helper answer and faults identically on both sides (fault
/// parity); arranged trials with a mapped pointer there run the block to
/// completion.
///
/// Original: 0x009BE040 (cdecl, eight stack words). Returns its flag in AL.
lf_checker_rt::export!(cdecl, rw_009BE040(owner: u32, kind: u32, aux: u32, target: u32, origin: u32, extra: u32, flags: u32, seed_obj: u32) -> u32 {
    unsafe {
        const SLOT_BASE: u32 = 0x0129_2460;
        const SLOT_END: u32 = 0x0129_3E60;
        const SLOT_STRIDE: u32 = 0xD0;
        const SLOT_COUNT: u32 = 0x20;
        const KIND_SPECIAL: u32 = 0x26;
        const STATE_GATE: u32 = 0x011F_7060;
        const EPOCH_A: u32 = 0x0120_88B4;
        const EPOCH_B: u32 = 0x00F1_C040;
        const MODE_GATE: u32 = 0x0103_7720;
        const MODE_TRIP: u32 = 0x12;
        const ADDR_BASE: u32 = 0x0117_35B4;
        const VARIANT_GATE: u32 = 0x011D_6FD4;
        const THIS_CONST_A: u32 = 0x018E_CFB0;
        const THIS_CONST_B: u32 = 0x0128_31E4;
        // Absolute immediates the worker relocates (like all other absolute
        // operands); the rewrite must use the relocated values.
        let this_a = lf_checker_rt::relocated(THIS_CONST_A);
        let this_b = lf_checker_rt::relocated(THIS_CONST_B);
        let t3_const_a = lf_checker_rt::relocated(TYPE3_CONST_A);
        let t3_const_c = lf_checker_rt::relocated(TYPE3_CONST_C);
        let t3_const_c18 = lf_checker_rt::relocated(TYPE3_CONST_C18);
        let t3_const_d = lf_checker_rt::relocated(TYPE3_CONST_D);
        const MIN_LEN_BITS: u32 = 0xFE8BAC;
        const ADD_CONST_BITS: u32 = 0xFE86B4;
        const K_SCALE_BITS: u32 = 0xFE8DC8;
        const AXIS_X: u32 = 0x0110_DB70;
        const AXIS_Y: u32 = 0x0110_DB74;
        const AXIS_Z: u32 = 0x0110_DB78;
        const QUAD_BASE: u32 = 0x01B4_B2A0;
        const TYPE3_CONST_A: u32 = 0x00E9_4C0C;
        const TYPE3_CONST_C: u32 = 0x00E9_4C48;
        const TYPE3_CONST_C18: u32 = 0x00E9_4C18;
        const TYPE3_CONST_D: u32 = 0x00E9_4C70;
        const INDIRECT_ARG_BITS: u32 = 0x3CA3_D70A;
        const OBJ_FLAG_A: u32 = 0x40;
        const OBJ_FLAG_B: u32 = 0x1000;
        const OBJ_FLAG_C: u32 = 0x10;
        const OBJ_FLAG_D: u32 = 0x1000_0000;
        const OBJ_PARAM_BITS: u32 = 0x3F8C_CCCD;
        const NEW_OBJ_SIZE: u32 = 0x70;

        const C_LOOKUP: u32 = 0;
        const C_GATE2: u32 = 1;
        const C_SMALL: u32 = 2;
        const C_PARAMS: u32 = 3;
        const C_SETUP: u32 = 4;
        const C_AIM: u32 = 5;
        const C_NORM: u32 = 6;
        const C_LOOKUP2: u32 = 7;
        const C_VECT: u32 = 8;
        const C_CHECK: u32 = 9;
        const C_ACCUM: u32 = 10;
        const C_SCAN: u32 = 11;
        const C_LINK: u32 = 12;
        const C_ATTACH: u32 = 13;
        const C_BUILD_A: u32 = 14;
        const C_BUILD_B: u32 = 15;
        const C_REF: u32 = 16;
        const C_RESOLVE: u32 = 17;
        const C_FINISH0: u32 = 18;
        const C_T3_A: u32 = 19;
        const C_T3_B: u32 = 20;
        const C_T3_FILL: u32 = 21;
        const C_T3_HELPER: u32 = 22;
        const C_T3_APPLY: u32 = 23;
        const C_T3_BLOCK: u32 = 24;
        const C_T3_BUILD: u32 = 25;
        const C_T3_C1: u32 = 26;
        const C_T3_C2: u32 = 27;
        const C_T3_C3: u32 = 28;
        const C_T3_DONE: u32 = 29;
        const C_REG_A: u32 = 30;
        const C_REG_B: u32 = 31;
        const C_APPLY: u32 = 32;
        const C_RELEASE: u32 = 33;
        const C_STORE_VEC: u32 = 34;
        const C_STORE_DIR: u32 = 35;
        const C_SUBOBJ: u32 = 36;
        const C_SUBFIN: u32 = 37;
        const C_NEW: u32 = 38;
        const C_CTOR: u32 = 39;
        const C_INIT: u32 = 40;
        const C_HOOK: u32 = 41;
        const C_COOKIE: u32 = 42;
        const C_V_START: u32 = 43;
        const C_V_RUN: u32 = 44;
        const C_V_TUNE: u32 = 45;
        const C_V_END: u32 = 46;

        #[inline(always)]
        unsafe fn rd32(p: u32) -> u32 {
            unsafe { (p as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(p: u32) -> f32 {
            unsafe { f32::from_bits(rd32(p)) }
        }
        #[inline(always)]
        unsafe fn rd8(p: u32) -> u8 {
            unsafe { (p as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(p: u32, v: u32) {
            unsafe { (p as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wrf(p: u32, v: f32) {
            unsafe { wr32(p, v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn wr8(p: u32, v: u8) {
            unsafe { (p as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits(g32(va)) }
        }
        #[inline(always)]
        fn mul(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(y)
        }
        #[inline(always)]
        fn add(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) + core::hint::black_box(y)
        }
        #[inline(always)]
        fn sub(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) - core::hint::black_box(y)
        }
        #[inline(always)]
        unsafe fn cookie() {
            unsafe { lf_checker_rt::callee_cdecl!(C_COOKIE, u32,) };
        }

        // Scratch vectors (addressable: stubs fill through these pointers).
        let mut v10 = [0u32; 4];
        let mut v30 = [0u32; 4];
        let mut p70 = [0u32; 4];
        let mut m80 = [0u32; 4];
        let mut m90 = [0u32; 3];
        let mut m98 = [0u32; 2];
        let mut ma0 = [0u32; 4];
        let mut tb4 = [0u32; 8];
        let mut t100 = [0u32; 8];
        let mut e64scratch = [0u32; 1];
        // Frame words the original never writes (contract: zero stack fill).
        let unw_w: f32 = 0.0;
        let unw_this: u32 = 0;
        // Gates.
        if g32(STATE_GATE) == 1 {
            cookie();
            return 0;
        }
        if g32(EPOCH_A) != g32(EPOCH_B) {
            cookie();
            return 0;
        }
        if g32(MODE_GATE) == MODE_TRIP {
            cookie();
            return 0;
        }
        let obj = lf_checker_rt::callee_cdecl!(C_LOOKUP, u32, kind);
        if obj == 0 {
            cookie();
            return 0;
        }
        let sel = rd32(obj.wrapping_add(0xA0));
        let gate_ans: u32 = lf_checker_rt::callee_cdecl!(C_GATE2, u32,);
        let flags_b: u8 = (flags & 0xFF) as u8;
        let mut e40: u32 = 0;
        let mut e58: u32 = 0;
        let mut e6c: u32 = extra;
        if (gate_ans & 0xFF) != 0 && flags_b == 0 {
            let x0 = lf_checker_rt::callee_cdecl!(C_SMALL, u32, owner) & 0xFFFF;
            let x1 = lf_checker_rt::callee_cdecl!(C_SMALL, u32, extra) & 0xFFFF;
            let x2 = lf_checker_rt::callee_cdecl!(C_SMALL, u32, seed_obj) & 0xFFFF;
            e40 = x1;
            let mut run = true;
            if sel == 0 && seed_obj != 0 && x2 == 0 {
                run = false;
            }
            if owner != 0 && x2 == 0 {
                run = false;
            }
            if extra != 0 && x1 == 0 {
                run = false;
            }
            if run {
                p70[0] = rd32(origin);
                p70[1] = rd32(origin.wrapping_add(4));
                p70[2] = rd32(origin.wrapping_add(8));
                lf_checker_rt::callee_thiscall!(C_PARAMS, u32, this_a, x0, kind, target,
                    p70.as_mut_ptr() as u32, x1, x2);
            }
        }
        v30 = [0, 0, 0, 0];
        lf_checker_rt::callee_thiscall!(C_SETUP, u32, m80.as_mut_ptr() as u32, aux);
        if sel > 4 {
            cookie();
            return 0;
        }
        let mut e44: u32 = 0;
        if sel == 1 {
            let t = lf_checker_rt::callee_cdecl!(C_LOOKUP2, u32, owner, kind);
            e44 = g32(ADDR_BASE).wrapping_add(t);
        } else if sel == 0 {
            e44 = g32(ADDR_BASE).wrapping_add(0x4E20);
        } else if sel == 2 {
            e44 = g32(ADDR_BASE).wrapping_add(0x1F40);
        }
        if sel <= 2 {
            let dx = sub(rdf(origin), rdf(target));
            let dy = sub(rdf(origin.wrapping_add(4)), rdf(target.wrapping_add(4)));
            let dz = sub(rdf(origin.wrapping_add(8)), rdf(target.wrapping_add(8)));
            v10[0] = dx.to_bits();
            v10[1] = dy.to_bits();
            v10[2] = dz.to_bits();
            v10[3] = unw_w.to_bits();
            if owner != 0 {
                let q: u32 = lf_checker_rt::callee_thiscall!(C_AIM, u32, owner);
                v30[0] = rd32(q);
                v30[1] = rd32(q.wrapping_add(4));
                v30[2] = rd32(q.wrapping_add(8));
                v30[3] = rd32(q.wrapping_add(12));
                lf_checker_rt::callee_thiscall!(C_NORM, u32, v30.as_mut_ptr() as u32);
                let fx = f32::from_bits(v30[0]);
                let fy = f32::from_bits(v30[1]);
                let fz = f32::from_bits(v30[2]);
                let len = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz)).sqrt();
                let cap = gf(MIN_LEN_BITS);
                let m = if len < cap { len } else { cap };
                v30[0] = mul(fx, m).to_bits();
                v30[1] = mul(fy, m).to_bits();
                v30[2] = mul(fz, m).to_bits();
            }
        } else if sel == 3 {
            e44 = g32(ADDR_BASE).wrapping_add(0xFA0);
            let w_a4 = rd32(obj.wrapping_add(0xA4));
            if w_a4 == 0 {
                let dx = sub(rdf(origin), rdf(target));
                let dy = sub(rdf(origin.wrapping_add(4)), rdf(target.wrapping_add(4)));
                let dz = sub(rdf(origin.wrapping_add(8)), rdf(target.wrapping_add(8)));
                v10[0] = dx.to_bits();
                v10[1] = dy.to_bits();
                v10[2] = dz.to_bits();
                v10[3] = unw_w.to_bits();
                lf_checker_rt::callee_thiscall!(C_VECT, u32, v10.as_mut_ptr() as u32);
                let (e10, e14, e18, e1c) = (
                    f32::from_bits(v10[0]),
                    f32::from_bits(v10[1]),
                    f32::from_bits(v10[2]),
                    f32::from_bits(v10[3]),
                );
                m98[0] = e18.to_bits();
                m98[1] = e1c.to_bits();
                let zero = 0.0f32;
                let t1 = mul(e18, zero);
                m90[0] = e10.to_bits();
                m90[1] = e14.to_bits();
                m90[2] = 0;
                m80[0] = sub(e14, t1).to_bits();
                m80[1] = sub(t1, e10).to_bits();
                let u1 = mul(e10, zero);
                let u0 = mul(e14, zero);
                ma0[0] = 0;
                ma0[1] = 0;
                ma0[2] = 0x3F80_0000;
                m80[2] = sub(u1, u0).to_bits();
                lf_checker_rt::callee_thiscall!(C_VECT, u32, m80.as_mut_ptr() as u32);
                let (f80, f84, f88) = (
                    f32::from_bits(m80[0]),
                    f32::from_bits(m80[1]),
                    f32::from_bits(m80[2]),
                );
                let (g90, g94, g98) = (
                    f32::from_bits(m90[0]),
                    f32::from_bits(m90[1]),
                    f32::from_bits(m98[0]),
                );
                let t0 = mul(f88, g94);
                let t4 = mul(f88, g90);
                let t1b = mul(f84, g98);
                let t6 = mul(f84, g90);
                let r1 = sub(t1b, t0);
                let t0b = mul(f80, g98);
                let t2 = mul(f80, g94);
                let r4 = sub(t4, t0b);
                let r2 = sub(t2, t6);
                ma0[0] = r1.to_bits();
                ma0[1] = r4.to_bits();
                ma0[2] = r2.to_bits();
                lf_checker_rt::callee_thiscall!(C_VECT, u32, ma0.as_mut_ptr() as u32);
                let s = rdf(obj.wrapping_add(0x18));
                v10[0] = mul(f32::from_bits(v10[0]), s).to_bits();
                v10[1] = mul(f32::from_bits(v10[1]), s).to_bits();
                v10[2] = mul(f32::from_bits(v10[2]), s).to_bits();
            } else {
                v10[0] = 0;
                v10[1] = 0;
                v10[2] = 0;
                e40 = 0;
                e58 = 0;
                let f: u32 = lf_checker_rt::callee_thiscall!(C_CHECK, u32, owner);
                if (f & 0xFF) != 0 {
                    let q: u32 = lf_checker_rt::callee_thiscall!(C_ACCUM, u32, owner,
                        p70.as_mut_ptr() as u32, target, 1, 0);
                    v10[0] = add(f32::from_bits(v10[0]), rdf(q)).to_bits();
                    let s1 = add(rdf(q.wrapping_add(4)), f32::from_bits(v10[1]));
                    e40 = s1.to_bits();
                    v10[1] = s1.to_bits();
                    e58 = add(rdf(q.wrapping_add(8)), f32::from_bits(v10[2])).to_bits();
                }
                let k = gf(K_SCALE_BITS);
                v10[0] = add(mul(gf(AXIS_X), k), f32::from_bits(v10[0])).to_bits();
                v10[2] = add(mul(gf(AXIS_Z), k), f32::from_bits(e58)).to_bits();
                v10[1] = add(mul(gf(AXIS_Y), k), f32::from_bits(e40)).to_bits();
            }
        } else {
            lf_checker_rt::callee_cdecl!(C_SCAN, u32, owner);
            let dx = sub(rdf(origin), rdf(target));
            let dy = sub(rdf(origin.wrapping_add(4)), rdf(target.wrapping_add(4)));
            let dz = sub(rdf(origin.wrapping_add(8)), rdf(target.wrapping_add(8)));
            v10[0] = dx.to_bits();
            v10[1] = dy.to_bits();
            v10[2] = dz.to_bits();
            v10[3] = unw_w.to_bits();
            e44 = rd32(obj.wrapping_add(0xA4)).wrapping_add(g32(ADDR_BASE));
        }
        // Tail: scan the slot table for the first free entry.
        let base = lf_checker_rt::relocated(SLOT_BASE);
        let mut idx = 0u32;
        if rd32(base) != 0 {
            let end = lf_checker_rt::relocated(SLOT_END);
            let mut p = base;
            loop {
                if p >= end {
                    break;
                }
                p = p.wrapping_add(SLOT_STRIDE);
                idx = idx.wrapping_add(1);
                if rd32(p) == 0 {
                    break;
                }
            }
        }
        if idx == SLOT_COUNT {
            cookie();
            return 0;
        }
        let slot = base.wrapping_add(idx.wrapping_mul(SLOT_STRIDE));
        // E+0x4c: the scan index, later overwritten by the type-3 block.
        let mut e4c: u32 = idx;
        if seed_obj != 0 {
            wr32(slot, seed_obj);
            if owner != 0
                && rd32(owner.wrapping_add(0x28)) & 0x3C0 == 0xC0
                && rd32(owner.wrapping_add(0x2CC)) == seed_obj
            {
                lf_checker_rt::callee_thiscall!(C_LINK, u32, owner.wrapping_add(0x2B0));
            }
            let so = rd32(slot);
            if rd8(so.wrapping_add(0x22A)) == 3 {
                wr8(so.wrapping_add(0x22A), 6);
            }
            lf_checker_rt::callee_cdecl!(C_ATTACH, u32, seed_obj, 0);
        } else {
            let t: u32 = lf_checker_rt::callee_cdecl!(C_BUILD_A, u32, 6, 1, 1, 1, 0xFFFF_FFFF);
            let o2: u32 = lf_checker_rt::callee_cdecl!(C_BUILD_B, u32, t);
            wr32(slot, o2);
        }
        let s0 = rd32(slot);
        if s0 == 0 {
            cookie();
            return 0;
        }
        wr32(s0.wrapping_add(0x214), rd32(s0.wrapping_add(0x214)) | OBJ_FLAG_A);
        wr32(s0.wrapping_add(0x120), OBJ_PARAM_BITS);
        if sel != 4 {
            wr32(s0.wrapping_add(0x118), rd32(s0.wrapping_add(0x118)) | OBJ_FLAG_B);
        }
        lf_checker_rt::callee_thiscall!(C_REF, u32, s0, slot);
        wr8(s0.wrapping_add(0x63), rd8(owner.wrapping_add(0x63)));
        {
            let vt: u32 = rd32(s0);
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(vt.wrapping_add(0x30)) as usize);
            f(s0, 1);
        }
        if rd32(owner.wrapping_add(0x28)) & 0x3C0 == 0xC0 {
            if rd32(owner.wrapping_add(0x26C)) & 4 != 0 {
                let t = rd32(owner.wrapping_add(0xB30));
                if t != 0 {
                    lf_checker_rt::callee_thiscall!(C_FINISH0, u32, s0, t, 0x20);
                } else if g32(VARIANT_GATE) == 2
                    && rd8(owner.wrapping_add(0x1E2)) & 0xF >= 2
                    && (rd32(owner.wrapping_add(0x24)) >> 5) & 1 == 0
                {
                    let t2: u32 = lf_checker_rt::callee_cdecl!(C_RESOLVE, u32, 0x20);
                    lf_checker_rt::callee_thiscall!(C_FINISH0, u32, s0, t2, 0x20);
                } else {
                    lf_checker_rt::callee_thiscall!(C_FINISH0, u32, s0, owner, 0x40);
                }
            } else if g32(VARIANT_GATE) == 2
                && rd8(owner.wrapping_add(0x1E2)) & 0xF >= 2
                && (rd32(owner.wrapping_add(0x24)) >> 5) & 1 == 0
            {
                let t2: u32 = lf_checker_rt::callee_cdecl!(C_RESOLVE, u32, 0x20);
                lf_checker_rt::callee_thiscall!(C_FINISH0, u32, s0, t2, 0x20);
            } else {
                lf_checker_rt::callee_thiscall!(C_FINISH0, u32, s0, owner, 0x40);
            }
        } else if kind == KIND_SPECIAL {
            lf_checker_rt::callee_thiscall!(C_FINISH0, u32, s0, owner, 0x20);
        }
        if sel == 3 {
            // Type-3 finish block.
            p70[0] = 0;
            p70[1] = 0x3F80_0000;
            p70[2] = 0;
            let s20 = slot.wrapping_add(0x20);
            lf_checker_rt::callee_thiscall!(C_T3_A, u32, s20, t3_const_a,
                p70.as_mut_ptr() as u32);
            wr32(slot.wrapping_add(0x68), rd32(rd32(slot).wrapping_add(0x20)));
            lf_checker_rt::callee_thiscall!(C_T3_B, u32, tb4.as_mut_ptr() as u32);
            let s74 = slot.wrapping_add(0x74);
            // The original spills s74 into the v30[1] slot when setting up
            // this call; v30 is untouched on this route otherwise.
            v30[1] = s74;
            let fill_const = if kind == KIND_SPECIAL { t3_const_c18 } else { t3_const_c };
            lf_checker_rt::callee_thiscall!(C_T3_FILL, u32, this_b, fill_const,
                s74, tb4.as_mut_ptr() as u32, 0xFFFF_FFFF, 0, 0);
            if rd32(s74) == 0 {
                // Zero path: excluded by the contract (scripted nonzero).
                // The original would continue with a shifted frame here and
                // fail its cookie check; no trial takes this path.
                cookie();
                return 0;
            }
            let h: f32 = lf_checker_rt::callee_thiscall!(C_T3_HELPER, f32, unw_this, 0);
            e4c = h.to_bits();
            lf_checker_rt::callee_thiscall!(C_T3_APPLY, u32, rd32(s74), h.to_bits());
            lf_checker_rt::callee_thiscall!(C_T3_BLOCK, u32, t100.as_mut_ptr() as u32, 0xAB);
            e6c = 0xAB;
            let b: u32 = lf_checker_rt::callee_cdecl!(C_T3_BUILD, u32,
                t3_const_d, 0, 0, 1, 0, tb4.as_mut_ptr() as u32,
                e64scratch.as_mut_ptr() as u32, rd32(slot), 0xFFFF_FFFF);
            let c1: u32 = lf_checker_rt::callee_cdecl!(C_T3_C1, u32, b);
            // NOTE: the original discards the C2 answer; the +0xA8 store
            // takes the C3 call's return value (eax after that call).
            lf_checker_rt::callee_stdcall!(C_T3_C2, u32, c1,
                t100.as_mut_ptr() as u32, rd32(slot));
            let c3: u32 = lf_checker_rt::callee_cdecl!(C_T3_C3, u32, c1);
            // Faulting-or-arranged dereference of the head/route value.
            let hobj = rd32(e40);
            wr32(hobj.wrapping_add(0xA4), c1);
            wr32(hobj.wrapping_add(0xA8), c3);
            wr32(hobj.wrapping_add(0xAC), 0);
            lf_checker_rt::callee_thiscall!(C_T3_DONE, u32, hobj, 0, 0, 0);
        }
        // Tail2: register the slot.
        let s70 = slot.wrapping_add(0x70);
        lf_checker_rt::callee_cdecl!(C_REG_A, u32, kind, rd32(slot), s70, 0);
        lf_checker_rt::callee_cdecl!(C_REG_B, u32, rd32(slot), kind, e4c, 0, 0);
        wr32(slot.wrapping_add(4), kind);
        wr32(slot.wrapping_add(8), sel);
        let s0c = slot.wrapping_add(0xC);
        wr32(s0c, owner);
        lf_checker_rt::callee_thiscall!(C_REF, u32, owner, s0c);
        tb4[1] = add(f32::from_bits(tb4[1]), gf(ADD_CONST_BITS)).to_bits();
        {
            let vt: u32 = rd32(s0);
            let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(vt.wrapping_add(4)) as usize);
            f(s0, m80.as_mut_ptr() as u32, 0, 1);
        }
        wr32(slot.wrapping_add(0x14), e44);
        wr8(slot.wrapping_add(0xB8), flags_b);
        wr32(slot.wrapping_add(0xB9), 0);
        let qb = lf_checker_rt::relocated(QUAD_BASE);
        wr32(slot.wrapping_add(0xA0), rd32(qb));
        wr32(slot.wrapping_add(0xA4), rd32(qb.wrapping_add(4)));
        wr32(slot.wrapping_add(0xA8), rd32(qb.wrapping_add(8)));
        wr32(slot.wrapping_add(0xAC), rd32(qb.wrapping_add(12)));
        wr32(slot.wrapping_add(0xB4), 0);
        wr32(s0.wrapping_add(0x118), rd32(s0.wrapping_add(0x118)) | OBJ_FLAG_C);
        wr32(s0.wrapping_add(0x210), rd32(s0.wrapping_add(0x210)) | OBJ_FLAG_D);
        lf_checker_rt::callee_thiscall!(C_APPLY, u32, s0, owner, 0);
        lf_checker_rt::callee_cdecl!(C_RELEASE, u32, rd32(slot), 0);
        lf_checker_rt::callee_thiscall!(C_STORE_VEC, u32, s0, v10.as_mut_ptr() as u32);
        lf_checker_rt::callee_thiscall!(C_STORE_DIR, u32, s0, v30.as_mut_ptr() as u32);
        if sel == 3 {
            if rd32(s0.wrapping_add(0x38)) != 0 {
                let t: u32 = lf_checker_rt::callee_thiscall!(C_SUBOBJ, u32, s0);
                let vtd: u32 = rd32(t);
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(vtd.wrapping_add(0x44)) as usize);
                f(t, INDIRECT_ARG_BITS);
                lf_checker_rt::callee_thiscall!(C_SUBFIN, u32, s0);
            }
            wr32(slot.wrapping_add(0x18), rd32(obj.wrapping_add(0xA4)).wrapping_add(g32(ADDR_BASE)));
        }
        if rd32(s0.wrapping_add(0x25C)) == 0 {
            let n: u32 = lf_checker_rt::callee_cdecl!(C_NEW, u32, NEW_OBJ_SIZE);
            if n != 0 {
                lf_checker_rt::callee_thiscall!(C_CTOR, u32, n);
            }
            // NOTE: the original zeroes eax after the conditional constructor
            // call, so the init call's object register is always null here.
            lf_checker_rt::callee_thiscall!(C_INIT, u32, 0, kind, 0, 1);
            // EXPERIMENT: orig pushes 0 here on every observed trial; testing.
            lf_checker_rt::callee_thiscall!(C_HOOK, u32, s0, 0);
        }
        wr32(slot.wrapping_add(0x10), e6c);
        if e6c != 0 {
            let s10 = slot.wrapping_add(0x10);
            lf_checker_rt::callee_thiscall!(C_REF, u32, e6c, s10);
        }
        let s20v = rd32(s0.wrapping_add(0x20));
        let cp: u32 = if s20v != 0 { s20v.wrapping_add(0x30) } else { s0.wrapping_add(0x10) };
        wr32(slot.wrapping_sub(0x10), rd32(cp));
        wrf(slot.wrapping_sub(0xC), rdf(cp.wrapping_add(4)));
        wrf(slot.wrapping_sub(8), rdf(cp.wrapping_add(8)));
        wr32(slot.wrapping_sub(4), rd32(cp.wrapping_add(12)));
        let ear: u32 = {
            let vt: u32 = rd32(s0);
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(vt.wrapping_add(0xEC)) as usize);
            f(s0, p70.as_mut_ptr() as u32)
        };
        wr32(slot.wrapping_add(0xA0), rd32(ear));
        wrf(slot.wrapping_add(0xA4), rdf(ear.wrapping_add(4)));
        wrf(slot.wrapping_add(0xA8), rdf(ear.wrapping_add(8)));
        wr32(slot.wrapping_add(0xAC), rd32(ear.wrapping_add(12)));
        cookie();
        1
    }
});
