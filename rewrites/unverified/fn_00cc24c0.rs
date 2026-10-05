// original: 0x00cc24c0 ped_task_range_update (proposed)

/// Refresh a task's ranged state from its member object, with cached limits.
///
/// `this` points to the task state, `arg` to a parameter block carrying bit
/// flags (`BITS` at +0x378). The task holds a member pointer (`MEMBER` at
/// +0x24), a reference point (`PX/PY` at +0x98/+0x9c), a height reading
/// (`PH` at +0xa4), flag bits (`FLAGS` at +0x50) and a sign word (`SIGN` at
/// +0xaa). A global caches a squared range limit, computed once from two
/// global floats (`CACHE_VAL`, valid bit in `CACHE_OK`).
///
/// What it does, in order: return 0 with the sign cleared unless a global
/// switch is set, the parameter's bit 11 is set and the member's
/// sub-object flag bit 1 is clear; fold a global mode byte with the task's
/// height against a constant (clearing it on ordered greater when flag bit
/// 13 is set); load or compute-and-cache the range limit; measure the
/// squared distance from the member's anchor point (+0x20) to the reference
/// point and, when it exceeds the limit with the mode byte allowing, pick a
/// sign (+1/-1/0) from the height error against a global tolerance (exact
/// zero and in-tolerance both yield 0), store it, set flag bit 0 of +0xa8
/// from the mode and copy the anchor into the reference fields; then,
/// unless gated out, call the member's virtual slot +0xec with a scratch
/// slot (pointer result, three floats), blend those with the anchor into
/// three sums, run two helper calls (a sync call and a six-argument query
/// with two out slots whose float and dword answers drive a second sign
/// pick), and finally refresh field +0x78 from a global when the sign is
/// nonzero. The returned dword is the last value the original left in EAX
/// on the taken path (0 on the early exits, a shifted flag word, a counter
/// difference or the picked sign); every path's value is input-derived and
/// is reproduced exactly.
///
/// Comparison semantics: `comiss`+`jbe` is taken for unordered operands
/// too, so its taken side is written `!(a > b)`; `comiss`+`jb` is
/// `!(a >= b)`. Any float field may be NaN or infinite and takes the same
/// side as the original in every case. The 16-byte xor mask at its global
/// is all sign bits, so the lane-xor is a plain negation.
///
/// Original: 0x00cc24c0 (thiscall, one stack argument, dword result in EAX;
/// 3 outgoing calls, one through the member's table; reads 10 globals,
/// writes 4 global words/bytes).
/// Honest narrowings (see the contract): three frame-pointer call
/// arguments are skipped (their contents snapshotted where observed, the
/// query's out values verified through the branches they drive).
lf_checker_rt::export!(thiscall, rw_00cc24c0(this: u32, arg: u32) -> u32 {
    unsafe {
        // Object layouts.
        const MEMBER: u32 = 0x24;
        const FLAGS: u32 = 0x50;
        const FLAG_BIT13: u32 = 0x2000;
        const H8: u32 = 0x08;
        const PX: u32 = 0x98;
        const PY: u32 = 0x9c;
        const PZ: u32 = 0xa0;
        const PH: u32 = 0xa4;
        const FLAGW: u32 = 0xa8;
        const SIGN: u32 = 0xaa;
        const SEED78: u32 = 0x78;
        const M_BASE: u32 = 0x20;
        const M_MODE: u32 = 0x14f;
        const M_SUB: u32 = 0xa80;
        const M_H: u32 = 0xb10;
        const SUB_BITS: u32 = 0x50;
        const B_X: u32 = 0x30;
        const B_Y: u32 = 0x34;
        const B_Z: u32 = 0x38;
        const VIRT_SLOT: u32 = 0xec;
        const BITS: u32 = 0x378;
        const PARAM_BIT11: u32 = 0x800;
        const SIGN_MASK: u32 = 0x8000_0000;
        // Globals (file VAs).
        const G_SWITCH: u32 = 0x0105_1456;
        const G_SWITCH2: u32 = 0x0105_1457;
        const G_REF2: u32 = 0x00FE_8A24;
        const G_TOL: u32 = 0x0105_14A0;
        const G_TOL_K: u32 = 0x00FE_8AD8;
        const G_HALF: u32 = 0x00FE_8830;
        const G_ONE: u32 = 0x00FE_88E8;
        const G_FOUR: u32 = 0x00FE_8AB8;
        const G_COUNT: u32 = 0x0117_35B4;
        const G_COUNT_SEEN: u32 = 0x0171_BFAC;
        const G_SEED: u32 = 0x0105_149C;
        const G_MODE: u32 = 0x0171_BFA8;
        const G_MODE2: u32 = 0x0171_BF95;
        const G_CACHE_VAL: u32 = 0x0171_C0E0;
        const G_CACHE_OK: u32 = 0x0171_C0E4;
        // Callees.
        const CALLEE_VIRT: u32 = 1;
        const CALLEE_SYNC: u32 = 2;
        const CALLEE_QUERY: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
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
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits((lf_checker_rt::relocated(va) as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn gd(va: u32) -> u32 {
            unsafe { (lf_checker_rt::relocated(va) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn gb(va: u32) -> u8 {
            unsafe { (lf_checker_rt::relocated(va) as *const u8).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wb(va: u32, v: u8) {
            unsafe { (lf_checker_rt::relocated(va) as *mut u8).write_unaligned(v) }
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
        fn neg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ SIGN_MASK)
        }
        #[inline(always)]
        unsafe fn tail(this: u32, r: u32) -> u32 {
            unsafe {
                if rd16(this + SIGN) != 0 {
                    wrf(this + SEED78, gf(G_SEED));
                }
                r
            }
        }

        let member = rd32(this + MEMBER);
        if gb(G_SWITCH) == 0 {
            wr16(this + SIGN, 0);
            return 0;
        }
        if rd32(arg + BITS) & PARAM_BIT11 == 0 {
            wr16(this + SIGN, 0);
            return 0;
        }
        if (rd32(rd32(member + M_SUB) + SUB_BITS) >> 1) & 1 != 0 {
            wr16(this + SIGN, 0);
            return 0;
        }

        // Mode fold.
        let xref = gf(G_REF2);
        let s50 = rd32(this + FLAGS);
        let cl: u8;
        if s50 & FLAG_BIT13 != 0 {
            let mut m = gb(G_MODE);
            if xref > rdf(this + H8) {
                m = 0;
            }
            wb(G_MODE, m);
            cl = m;
        } else {
            cl = gb(G_MODE);
        }

        // Cached range limit.
        let tol = gf(G_TOL);
        let cok = gd(G_CACHE_OK);
        let limit: f32;
        if cok & 1 != 0 {
            limit = gf(G_CACHE_VAL);
        } else {
            let mut l = mul(tol, gf(G_TOL_K));
            wr32(lf_checker_rt::relocated(G_CACHE_OK), cok | 1);
            l = mul(l, l);
            wrf(lf_checker_rt::relocated(G_CACHE_VAL), l);
            limit = l;
        }

        // Distance gate; skipped paths rejoin below with ret = s1.
        let mut dl: u8 = 0;
        let mut ret: u32;
        let base = rd32(member + M_BASE);
        let s1 = s50 >> 13;
        let gated = (s50 & FLAG_BIT13 != 0 && cl != 0) || {
            let dx = sub(rdf(base + B_X), rdf(this + PX));
            let dy = sub(rdf(base + B_Y), rdf(this + PY));
            let dist2 = add(mul(dy, dy), mul(dx, dx));
            !(dist2 > limit)
        };
        if gated {
            ret = s1;
        } else {
            // Sign pick from the height error.
            let do_pick: bool;
            if gb(G_MODE2) != 0 {
                dl = 1;
                do_pick = true;
            } else {
                dl = ((member + M_MODE) as *const u8).read_unaligned() & 1;
                do_pick = dl != 0;
            }
            let sign: i32 = if !do_pick {
                0
            } else if rd16(this + FLAGW) & 1 == 0 {
                0
            } else {
                let diff = sub(rdf(member + M_H), rdf(this + PH));
                if diff >= tol {
                    1
                } else if neg(tol) >= diff {
                    -1
                } else {
                    0
                }
            };
            wr16(this + SIGN, sign as u16);
            wr16(this + FLAGW, (rd16(this + FLAGW) & 0xFFFE) | (dl as u16 & 1));
            wr32(this + PX, rd32(base + B_X));
            wr32(this + PY, rd32(base + B_Y));
            wr32(this + PZ, rd32(base + B_Z));
            ret = rd32(member + M_H);
            wr32(this + PH, ret);
        }

        // Second gate chain.
        if gb(G_SWITCH2) == 0 {
            return tail(this, ret);
        }
        if dl != 0 {
            return tail(this, ret);
        }
        if s50 & FLAG_BIT13 == 0 {
            return tail(this, ret);
        }
        let s2 = rd32(rd32(member + M_SUB) + SUB_BITS) >> 1;
        if (s2 & 1) != 0 {
            return tail(this, s2);
        }
        if !(rdf(this + H8) >= xref) {
            return tail(this, s2);
        }
        let cnt = gd(G_COUNT);
        let s3 = cnt.wrapping_sub(gd(G_COUNT_SEEN));
        if s3 < 250 {
            return tail(this, s3);
        }
        wr32(lf_checker_rt::relocated(G_COUNT_SEEN), cnt);

        // Virtual slot +0xec of the member, pointer result.
        let mut ind_slot: u32 = 0;
        let ind_ptr = core::ptr::addr_of_mut!(ind_slot) as u32;
        let virt: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(member) + VIRT_SLOT) as usize);
        let p = virt(member, ind_ptr);
        let half = gf(G_HALF);
        let q1 = mul(rdf(p + 4), half);
        let mut q3 = mul(rdf(p), half);
        let q2 = mul(rdf(p + 8), half);
        let base2 = rd32(member + M_BASE);
        let mut q0 = rdf(base2 + B_Y);
        q3 = add(q3, rdf(base2 + B_X));
        q0 = add(q0, q1);
        let mut q1b = rdf(base2 + B_Z);
        q1b = add(q1b, q2);

        // Sync call, then the query's float argument.
        let mut sync_slot: u32 = 0;
        let sync_ptr = core::ptr::addr_of_mut!(sync_slot) as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_SYNC, u32, sync_ptr);
        let f28 = add(q1b, gf(G_ONE));
        let farg = sub(f28, gf(G_FOUR));

        // Six-argument query with two out slots.
        let mut out20 = [q3.to_bits(), q0.to_bits(), f28.to_bits()];
        let mut out40 = [0u32; 20];
        let out20_ptr = out20.as_mut_ptr() as u32;
        let out40_ptr = out40.as_mut_ptr() as u32;
        let r2: u32 = lf_checker_rt::callee_cdecl!(
            CALLEE_QUERY, u32, out20_ptr, farg.to_bits(), member, out40_ptr, 6, 0
        );
        if (r2 & 0xFF) == 0 {
            wr16(this + SIGN, 0);
            wb(G_MODE, 0);
            return tail(this, 0);
        }
        if ((out40[18] >> 24) & 1) == 0 {
            wr16(this + SIGN, 0);
            wb(G_MODE, 0);
            return tail(this, 0);
        }
        let t = sub(rdf(base2 + B_Z), gf(G_ONE));
        let f1 = sub(f32::from_bits(out40[6]), t);
        if f1 > tol {
            wr16(this + SIGN, 1);
            ret = 1;
        } else if neg(tol) > f1 {
            wr16(this + SIGN, 0xFFFF);
            ret = 0xFFFF_FFFF;
        } else {
            wr16(this + SIGN, 0);
            ret = 0;
        }
        wb(G_MODE, 1);
        tail(this, ret)
    }
});
