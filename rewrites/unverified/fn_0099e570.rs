// original: 0x0099E570 LINEAR_RISE

/// Resolve one audio emitter's two output levels and its world position.
///
/// `this` is the mixer/emitter object. `out_level_a` and `out_level_b` each
/// receive one float level; `out_pos` receives four words (the emitter's
/// position at `+0x1E0` plus the word at `+0x1EC`). Returns the word at
/// `this+0x1EC`.
///
/// Object layout read here: `+0x08` child pointer (its `+0x20` word,
/// `+0xF4`/`+0x26C` flag bytes with bit 2 tested, `+0x218`/`+0x219` flag
/// bytes, `+0xB30` handle); `+0xD0`/`+0x120` filter sub-objects;
/// `+0x170`/`+0x18C` range sub-objects; `+0x1B0` helper object (stored to
/// `+0x118` and `+0x168`); `+0x1E0`/`+0x1E4`/`+0x1E8` position, `+0x1EC`
/// return word.
///
/// Stages, all gated per trial: an optional filter/range refresh (global
/// flag byte); a curve stage that maps a probe answer through two control
/// curves; a spatial stage that reads the listener position from a table
/// indexed through thread-local state, takes the emitter distance, and
/// runs two clamped interpolations plus a clamp chain; and a tail that
/// takes ordered minimums (compare-instruction order and NaN semantics:
/// an unordered compare takes the "below" branch) and evaluates two
/// output curves whose x87 results are stored to the out params.
///
/// The live-handle helper's answer is tested as an 8-bit value (only `al`
/// matters). The probe handle fed to the distance helper is an unsigned
/// word plus a constant. Two stack slots are read without being written
/// on some paths (the source vector's fourth word when the refresh runs,
/// one curve slot when the curve stage is skipped); the checker defines
/// unwritten stack as zero and this rewrite uses 0.0 there.
///
/// Original: 0x0099E570 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_0099E570(this: u32, out_level_a: u32, out_level_b: u32, out_pos: u32) -> u32 {
    unsafe {
        const FLAG_REFRESH: u32 = 0x128437A;
        const GATE_STAGES: u32 = 0x128437E;
        const TLS_INDEX_CELL: u32 = 0x17ABA14;
        const MEDIATOR: u32 = 0x11735B4;
        const POS_TABLE: u32 = 0x115E420;
        const DEFAULT_LEVEL: u32 = 0xE7CBA8;
        const RATIO_NUM: u32 = 0xFE88E8;
        const FILT_A: u32 = 0x1038D24;
        const FILT_B: u32 = 0x1038D2C;
        const FILT_C: u32 = 0x1038D30;
        const FILT_D: u32 = 0x1038D34;
        const FILT_E: u32 = 0x1038D38;
        const RANGE_A: u32 = 0x1038D3C;
        const RANGE_B: u32 = 0x1038D40;
        const LERP2_A_BITS: u32 = 0x46BAB800;
        const LERP2_B: u32 = 0x1038D28;
        const CLAMP_C1: u32 = 0x1038DBC;
        const CLAMP_C2: u32 = 0x1038DB8;
        const CLAMP_C3: u32 = 0x1038DB0;
        const CLAMP_C4: u32 = 0x1038DAC;
        const CLAMP_C5: u32 = 0x1038DC0;
        const CLAMP_C6: u32 = 0x1038DB4;
        const FILTER_NAME_1: u32 = 0xE90B58;
        const FILTER_NAME_2: u32 = 0xE90B64;
        const CURVE_OBJ_1: u32 = 0x1288728;
        const CURVE_OBJ_2: u32 = 0x1288750;
        const ATTACH_CODE: u32 = 0x4B5;
        const CHILD: u32 = 0x08;
        const CHILD_ARG: u32 = 0x20;
        const CHILD_MODE_FLAG: u32 = 0xF4;
        const CHILD_POS_FLAGS: u32 = 0x218;
        const CHILD_LIVE_FLAG: u32 = 0x26C;
        const CHILD_HANDLE: u32 = 0xB30;
        const FLAG_BIT: u8 = 4;
        const FILTER_1: u32 = 0xD0;
        const FILTER_2: u32 = 0x120;
        const RANGE_1: u32 = 0x170;
        const RANGE_2: u32 = 0x18C;
        const SLOT_A: u32 = 0x118;
        const SLOT_B: u32 = 0x168;
        const HELPER: u32 = 0x1B0;
        const PROBE_BIAS: u32 = 0x210;
        const POS_X: u32 = 0x1E0;
        const POS_Y: u32 = 0x1E4;
        const POS_Z: u32 = 0x1E8;
        const RETURN_WORD: u32 = 0x1EC;
        const TLS_SLOT: usize = 5;
        const TLS_STRUCT_INDEX: u32 = 0x70;
        const POS_RECORD_LEN: u32 = 64;
        const ONE_BITS: u32 = 0x3F800000;
        // Callee ids in this function's checker contract.
        const C_FILTER_SETUP: u32 = 1;
        const C_RANGE_SETUP: u32 = 2;
        const C_LIVE_1: u32 = 3;
        const C_LIVE_2: u32 = 4;
        const C_PROBE: u32 = 5;
        const C_CURVE: u32 = 6;
        const C_ATTACH: u32 = 7;
        const C_VOICE: u32 = 8;
        const C_LERP: u32 = 9;
        const C_BLEND: u32 = 10;
        const C_OUTPUT: u32 = 11;

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
        unsafe fn gf(addr: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(addr))) }
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
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        let one = f32::from_bits(ONE_BITS);
        let default = gf(DEFAULT_LEVEL);
        // Working slots; l20 has no initializer in the original (it keeps
        // whatever the stack held when the curve stage is skipped, which
        // the checker defines as zero).
        let mut l14 = default;
        let mut l1c = default;
        let mut l18 = 0.0f32;
        let mut l20 = 0.0f32;
        let mut l24 = 0.0f32;
        let mut l28 = 0.0f32;

        if rd8(lf_checker_rt::relocated(FLAG_REFRESH)) != 0 {
            let mut src = [0u32, ONE_BITS, 0u32, 0u32];
            lf_checker_rt::callee_thiscall!(C_FILTER_SETUP, u32,
                this.wrapping_add(FILTER_1),
                lf_checker_rt::relocated(FILTER_NAME_1),
                src.as_mut_ptr() as u32,
                rd32(lf_checker_rt::relocated(FILT_A)),
                rd32(lf_checker_rt::relocated(FILT_B)),
                rd32(lf_checker_rt::relocated(FILT_C)));
            lf_checker_rt::callee_thiscall!(C_FILTER_SETUP, u32,
                this.wrapping_add(FILTER_2),
                lf_checker_rt::relocated(FILTER_NAME_2),
                src.as_mut_ptr() as u32,
                ONE_BITS,
                rd32(lf_checker_rt::relocated(FILT_B)),
                rd32(lf_checker_rt::relocated(FILT_C)));
            let fd = rd32(lf_checker_rt::relocated(FILT_D));
            lf_checker_rt::callee_thiscall!(C_RANGE_SETUP, u32,
                this.wrapping_add(RANGE_1), fd, fd);
            let fe = rd32(lf_checker_rt::relocated(FILT_E));
            lf_checker_rt::callee_thiscall!(C_RANGE_SETUP, u32,
                this.wrapping_add(RANGE_2), fe, fe);
        }

        let mut child = rd32(this.wrapping_add(CHILD));
        let mut curve_ran = false;
        if child != 0
            && rd8(child.wrapping_add(CHILD_LIVE_FLAG)) & FLAG_BIT != 0
            && rd32(child.wrapping_add(CHILD_HANDLE)) != 0
        {
            let live: u32 = lf_checker_rt::callee_stdcall!(C_LIVE_1, u32, child);
            let enter = (live as u8) == 0
                || rd8(lf_checker_rt::relocated(GATE_STAGES)) != 0;
            if enter {
                let flag_set =
                    rd8(child.wrapping_add(CHILD_LIVE_FLAG)) & FLAG_BIT != 0;
                let base = if flag_set {
                    rd32(child.wrapping_add(CHILD_HANDLE))
                } else {
                    0
                };
                let probe_in: f32 = lf_checker_rt::callee_thiscall!(C_PROBE, f32,
                    base.wrapping_add(PROBE_BIAS));
                l20 = probe_in;
                let a1: f64 = lf_checker_rt::callee_thiscall!(C_CURVE, f64,
                    lf_checker_rt::relocated(CURVE_OBJ_1), l20.to_bits());
                l28 = a1 as f32;
                let a2: f64 = lf_checker_rt::callee_thiscall!(C_CURVE, f64,
                    lf_checker_rt::relocated(CURVE_OBJ_2), l20.to_bits());
                l24 = a2 as f32;
                curve_ran = true;
            }
        }
        if !curve_ran {
            l28 = default;
        }

        child = rd32(this.wrapping_add(CHILD));
        if child != 0 {
            let helper = this.wrapping_add(HELPER);
            if rd8(child.wrapping_add(CHILD_MODE_FLAG)) & FLAG_BIT != 0 {
                lf_checker_rt::callee_thiscall!(C_ATTACH, u32, child, helper, ATTACH_CODE);
            } else {
                lf_checker_rt::callee_thiscall!(C_VOICE, u32,
                    helper, rd32(child.wrapping_add(CHILD_ARG)));
            }
            wr32(this.wrapping_add(SLOT_A), helper);
            wr32(this.wrapping_add(SLOT_B), helper);
            let live: u32 = lf_checker_rt::callee_stdcall!(C_LIVE_2, u32, child);
            let gated_out = (live as u8) != 0
                && rd8(lf_checker_rt::relocated(GATE_STAGES)) == 0;
            let mut spatial = !gated_out;
            if spatial {
                let f0 = rd8(child.wrapping_add(CHILD_POS_FLAGS));
                let f1 = rd8(child.wrapping_add(CHILD_POS_FLAGS).wrapping_add(1));
                spatial = f0 != 0 || f1 == 0;
            }
            if !spatial {
                l18 = 0.0;
                l1c = default;
            } else {
                let tls_base = lf_checker_rt::tls_slot(TLS_SLOT);
                debug_assert_eq!(
                    rd32(lf_checker_rt::relocated(TLS_INDEX_CELL)) as usize,
                    TLS_SLOT);
                let index = rd32(tls_base.wrapping_add(TLS_STRUCT_INDEX));
                let entry = lf_checker_rt::relocated(POS_TABLE)
                    .wrapping_add(index.wrapping_mul(POS_RECORD_LEN));
                let dx = sub(rdf(entry), rdf(this.wrapping_add(POS_X)));
                let dy = sub(rdf(entry.wrapping_add(4)), rdf(this.wrapping_add(POS_Y)));
                let dz = sub(rdf(entry.wrapping_add(8)), rdf(this.wrapping_add(POS_Z)));
                let dist = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz)).sqrt();
                l14 = dist;
                let lerp1: f32 = lf_checker_rt::callee_cdecl!(C_LERP, f32,
                    0u32,
                    ONE_BITS,
                    rd32(lf_checker_rt::relocated(RANGE_A)),
                    rd32(lf_checker_rt::relocated(RANGE_B)),
                    dist.to_bits());
                l20 = lerp1;
                let b1: f32 = lf_checker_rt::callee_thiscall!(C_BLEND, f32,
                    this.wrapping_add(FILTER_1), 0u32);
                l1c = b1;
                l18 = mul(b1, l20);
                let b2: f32 = lf_checker_rt::callee_thiscall!(C_BLEND, f32,
                    this.wrapping_add(FILTER_2), 0u32);
                l1c = b2;
                let lerp2: f32 = lf_checker_rt::callee_cdecl!(C_LERP, f32,
                    LERP2_A_BITS,
                    rd32(lf_checker_rt::relocated(LERP2_B)),
                    0u32,
                    ONE_BITS,
                    mul(l1c, l20).to_bits());
                l24 = lerp2;
                let c1 = gf(CLAMP_C1);
                let c2 = gf(CLAMP_C2);
                let c3 = gf(CLAMP_C3);
                let c4 = gf(CLAMP_C4);
                let c5 = gf(CLAMP_C5);
                let r1 = gf(RATIO_NUM);
                let ratio1 = div(r1, sub(c1, c2));
                l1c = ratio1;
                let ratio2 = div(r1, sub(c5, c1));
                l20 = ratio2;
                let x4 = l14;
                let x7 = sub(x4, c1);
                let mut x6 = mul(sub(gf(CLAMP_C6), c3), mul(x7, l20));
                let mut x1 = sub(x4, c2);
                x6 = add(x6, c3);
                if !(x1 >= 0.0) {
                    l14 = c4;
                } else {
                    x1 = mul(x1, l1c);
                    let mut x3 = sub(c3, c4);
                    x3 = mul(x3, x1);
                    x3 = add(x3, c4);
                    l14 = x3;
                }
                if x7 >= 0.0 {
                    l14 = x6;
                }
                if sub(x4, c5) >= 0.0 {
                    l14 = gf(CLAMP_C6);
                }
                l1c = l24;
            }
        }

        let mediator = rd32(lf_checker_rt::relocated(MEDIATOR));
        let mut r0 = l24;
        if !(l18 > l24) {
            r0 = l18;
        }
        let s1: f32 = lf_checker_rt::callee_thiscall!(C_OUTPUT, f32,
            this.wrapping_add(RANGE_1), r0.to_bits(), mediator);
        wrf(out_level_a, s1);
        let mut r1 = l1c;
        if !(l28 > l1c) {
            r1 = l28;
        }
        if !(l14 > r1) {
            r1 = l14;
        }
        let s2: f32 = lf_checker_rt::callee_thiscall!(C_OUTPUT, f32,
            this.wrapping_add(RANGE_2), r1.to_bits(), mediator);
        wrf(out_level_b, s2);
        wr32(out_pos, rd32(this.wrapping_add(POS_X)));
        wrf(out_pos.wrapping_add(4), rdf(this.wrapping_add(POS_Y)));
        wrf(out_pos.wrapping_add(8), rdf(this.wrapping_add(POS_Z)));
        let ret = rd32(this.wrapping_add(RETURN_WORD));
        wr32(out_pos.wrapping_add(12), ret);
        ret
    }
});
