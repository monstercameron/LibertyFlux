// original: 0x00cc2230 ped_task_guard_update (proposed)

/// Refresh a task's guarded state, polling a controller object it looks up.
///
/// `this` points to the task state, `arg` to a parameter block carrying bit
/// flags (`BITS` at +0x378). The task holds a member object pointer
/// (`MEMBER` at +0x24), two distance components (`D0` at +0x0c, `D1` at
/// +0x10), a weight (`W` at +0x28), flag bits (`FLAGS` at +0x50) and scratch
/// fields. A global holds the current controller object (`CTL`).
///
/// What it does, in order: return 0 unless the parameter's bit 9 is set;
/// fetch the controller through callee 1 (returning 0 on null) and publish
/// it to `CTL`; seed fields from globals; when the controller's kind word
/// (+0x0c) is 0x37/0x38 and its squared distance is at least a global
/// limit, stamp a flag, poke the controller through callee 2 and return 0;
/// otherwise call the controller's virtual slot +8 (thiscall, float
/// result), and return 0 with a flag when the limit-minus-answer does not
/// exceed the saved +0x4c reading; then run a multi-argument update call
/// (callee 4) and, by kind (0x37/0x38, 0x39/0x3a, anything else), zero field
/// +0x04, set +0x08 to 1.0/2.0/3.0 and either probe the member (callee 5/6,
/// byte result) or check the member's sub-object flag, falling through to
/// clear +0x0c and copy the level into +0x10; then call callee 7 (thiscall,
/// magic 0x4000, an out slot, 0, 1.0) and, when it answers nonzero and the
/// controller's +0x4c reading covers the returned value, publish the scaled
/// weight, set a global byte and return 1; otherwise clear the weight, run
/// a final float call (callee 9) unless the sub-object flag vetoes it, and
/// return 1.
///
/// Comparison semantics: `comiss`+`jb` is taken for unordered operands too,
/// so its taken side is written `!(a >= b)`; `comiss`+`jbe` is `!(a > b)`.
/// Any float field may be NaN or infinite and takes the same side as the
/// original in every case.
///
/// Original: 0x00cc2230 (thiscall, one stack argument, returns its result in
/// AL; 8 outgoing calls to 7 callees, one through the controller's table;
/// reads 5 global floats/words, writes 2 global words and 1 global byte).
/// Two honest narrowings (see the contract): callee 7's out slot is the
/// caller's own argument word, which a Rust rewrite cannot name, so the
/// pointer argument is skipped and the stack check is off; the out value is
/// still verified through the branch it drives.
lf_checker_rt::export!(thiscall, rw_00cc2230(this: u32, arg: u32) -> u32 {
    unsafe {
        // Object layouts.
        const D0: u32 = 0x0c;
        const D1: u32 = 0x10;
        const LV_OUT: u32 = 0x04;
        const LV_COPY: u32 = 0x08;
        const MEMBER: u32 = 0x24;
        const W: u32 = 0x28;
        const W_OUT: u32 = 0x30;
        const SEED54: u32 = 0x54;
        const SEED1C: u32 = 0x1c;
        const SEED78: u32 = 0x78;
        const FLAGS: u32 = 0x50;
        const FLAG_EARLY: u32 = 0x2_0000;
        const FLAG_MAIN: u32 = 0x10_0000;
        const M_SUB: u32 = 0x6c;
        const M_ARG: u32 = 0x78;
        const M_FIN: u32 = 0xaa0;
        const SUB_FLAG: u32 = 0x0e;
        const CTL_KIND: u32 = 0x0c;
        const CTL_RD: u32 = 0x4c;
        const CTL_SLOT: u32 = 0x08;
        const BITS: u32 = 0x378;
        const PARAM_BIT9: u32 = 0x200;
        // Globals (file VAs).
        const G_CTL: u32 = 0x0171_BF9C;
        const G_SEED: u32 = 0x0105_149C;
        const G_SEED_OUT: u32 = 0x0171_C0D8;
        const G_LEN_LIM: u32 = 0x00FE_870C;
        const G_DIFF_LIM: u32 = 0x00FE_88E8;
        const G_UPD_K: u32 = 0x0105_1448;
        const G_W_SCALE: u32 = 0x0117_35BC;
        const G_DONE: u32 = 0x0171_BF96;
        // Callees.
        const CALLEE_FETCH: u32 = 1;
        const CALLEE_POKE: u32 = 2;
        const CALLEE_VIRT: u32 = 3;
        const CALLEE_UPDATE: u32 = 4;
        const CALLEE_PROBE: u32 = 5;
        const CALLEE_RUN: u32 = 6;
        const CALLEE_FINAL: u32 = 7;

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
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits((lf_checker_rt::relocated(va) as *const u32).read_unaligned()) }
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

        if rd32(arg + BITS) & PARAM_BIT9 == 0 {
            return 0;
        }
        let member = rd32(this + MEMBER);
        let ctl: u32 = lf_checker_rt::callee_thiscall!(CALLEE_FETCH, u32, this, rd32(member + M_ARG));
        wr32(lf_checker_rt::relocated(G_CTL), ctl);
        if ctl == 0 {
            return 0;
        }
        wr32(this + SEED54, 0x4040_0000);
        wrf(this + SEED78, gf(G_SEED));
        wr32(lf_checker_rt::relocated(G_SEED_OUT), 0x4040_0000);
        wr32(this + SEED1C, rd32(this + SEED54));

        let kind = rd32(ctl + CTL_KIND);
        if kind == 0x37 || kind == 0x38 {
            let d0 = rdf(this + D0);
            let d1 = rdf(this + D1);
            let len2 = add(mul(d0, d0), mul(d1, d1));
            if !(gf(G_LEN_LIM) >= len2) {
                // Below the limit: fall into the virtual-call path.
            } else {
                wr32(this + FLAGS, rd32(this + FLAGS) | FLAG_EARLY);
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    CALLEE_POKE, u32, ctl, 0xC080_0000u32
                );
                return 0;
            }
        }

        // Virtual slot +8 of the controller, float result.
        let saved = rdf(ctl + CTL_RD);
        let virt: extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute(rd32(rd32(ctl) + CTL_SLOT) as usize);
        let t1 = virt(ctl);
        let diff = sub(gf(G_DIFF_LIM), t1);
        if !(diff > saved) {
            wr32(this + FLAGS, rd32(this + FLAGS) | FLAG_EARLY);
            return 0;
        }
        wr32(this + FLAGS, rd32(this + FLAGS) | FLAG_MAIN);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CALLEE_UPDATE, u32, rd32(member + M_ARG), rd32(arg), kind, gf(G_UPD_K).to_bits(), rd32(arg + 4)
        );

        // Per-kind level setup.
        let kind2 = rd32(ctl + CTL_KIND);
        if kind2 == 0x37 || kind2 == 0x38 {
            wr32(this + LV_OUT, 0);
            wrf(this + LV_COPY, 1.0);
            let m = rd32(member + M_SUB);
            if m == 0 || (m as *const u8).byte_add(SUB_FLAG as usize).read_unaligned() == 0 {
                wr32(this + D0, 0);
                wrf(this + D1, 1.0);
            }
        } else if kind2 == 0x39 || kind2 == 0x3a {
            wr32(this + LV_OUT, 0);
            wrf(this + LV_COPY, 2.0);
            let r: u32 = lf_checker_rt::callee_thiscall!(CALLEE_PROBE, u32, member);
            if r & 0xFF == 0 {
                wr32(this + D0, 0);
                wrf(this + D1, 2.0);
            }
        } else {
            wr32(this + LV_OUT, 0);
            wrf(this + LV_COPY, 3.0);
            let r: u32 = lf_checker_rt::callee_thiscall!(CALLEE_PROBE, u32, member);
            if r & 0xFF == 0 {
                wr32(this + D0, 0);
                wrf(this + D1, 3.0);
            }
        }

        // Run call with an out slot, then the weight decision.
        let mut outval: u32 = 0;
        let outptr = core::ptr::addr_of_mut!(outval) as u32;
        let r: u32 = lf_checker_rt::callee_thiscall!(
            CALLEE_RUN, u32, ctl, 0x4000, outptr, 0, 0x3F80_0000u32
        );
        if r & 0xFF != 0 {
            let g4c = rdf(ctl + CTL_RD);
            let outf = f32::from_bits(outval);
            if !(g4c >= outf) {
                // Out of range: take the clear path below.
            } else {
                let w = mul(rdf(this + W), gf(G_W_SCALE));
                wrf(this + W_OUT, w);
                (lf_checker_rt::relocated(G_DONE) as *mut u8).write_unaligned(1);
                return 1;
            }
        }
        wr32(this + W, 0);
        let m = rd32(member + M_SUB);
        if m == 0 || (m as *const u8).byte_add(SUB_FLAG as usize).read_unaligned() == 0 {
            let fin = rdf(member + M_FIN);
            let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_FINAL, u32, member, fin.to_bits());
        }
        1
    }
});
