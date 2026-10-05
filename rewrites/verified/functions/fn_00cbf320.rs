// original: 0x00CBF320 task_aim_update (proposed)

/// Refresh a ped task's aim state from its head object and target.
///
/// `this` is the task object (head pointer at `+0x24`, outputs at `+0x8`,
/// `+0x28` and `+0x2c`) and `arg0` points at the target record. A mode word
/// on the head picks a reset (outputs zeroed, the pitch slot takes the
/// head-pitch word) or the full update: the pitch output tracks the head
/// pitch within a target-plus-one bound, the head's two filter words are
/// smoothed and their difference normalised into range, a blend factor is
/// derived through mode-selected globals (or published to a global on the
/// override path), and the lateral outputs are divided, clamped and
/// reconciled through two min/max tails. Every below-or-equal branch after a float compare takes
/// the unordered (NaN) side; every `ja` or `jb` takes only its ordered side.
///
/// Original: 0x00CBF320 (thiscall, one stack argument, no return value).
lf_checker_rt::export!(thiscall, rw_00cbf320(this: u32, arg0: u32) -> u32 {
    unsafe {
        const SMOOTH_A: u32 = 1;
        const SMOOTH_B: u32 = 2;
        const REFINE_A: u32 = 3;
        const REFINE_B: u32 = 4;
        const K_HI: u32 = 0x00fe_8aa0;
        const K_ADJ: u32 = 0x00fe_8aec;
        const K_LO: u32 = 0x00fe_8dc4;
        const K_ABS: u32 = 0x00ed_9180;
        const MASK_ABS: u32 = 0x00fe_8f80;
        const K_EGATE: u32 = 0x00fe_8894;
        const K_HALF: u32 = 0x00fe_8830;
        const K_ONE: u32 = 0x00fe_88e8;
        const K_INVTAU: u32 = 0x00fe_87bc;
        const K_H1: u32 = 0x00fe_8728;
        const K_H2: u32 = 0x00fe_8b68;
        const K_X8: u32 = 0x00fe_8afc;
        const K_LIM: u32 = 0x00fe_8d48;
        const K_DIV1: u32 = 0x00fe_870c;
        const MASK_NEG: u32 = 0x00fe_8fa0;
        const K_S8MIN: u32 = 0x00fe_8a24;
        const G_X4LO: u32 = 0x0105_14e8;
        const G_X3LO: u32 = 0x0105_14ec;
        const G_X4HI: u32 = 0x0105_14f0;
        const G_X3HI: u32 = 0x0105_14f4;
        const G_X6: u32 = 0x0105_14f8;
        const G_X7: u32 = 0x0105_14fc;
        const G_DVHI: u32 = 0x0105_1494;
        const G_DVLO: u32 = 0x0105_1498;
        const G_PUB: u32 = 0x0105_14a8;
        const G_MIX: u32 = 0x0117_35bc;
        const G_42: u32 = 0x0105_1442;
        const G_43: u32 = 0x0105_1443;
        const G_50: u32 = 0x0105_1450;
        const G_51: u32 = 0x0105_1451;
        const G_8B: u32 = 0x0171_bf8b;
        const G_90: u32 = 0x0171_bf90;
        const G_92: u32 = 0x0171_bf92;
        const G_93: u32 = 0x0171_bf93;
        const G_97: u32 = 0x0171_bf97;
        const G_E8: u32 = 0x0171_c0e8;
        const G_E9: u32 = 0x0171_c0e9;
        const G_EA: u32 = 0x0171_c0ea;
        const G_EB: u32 = 0x0171_c0eb;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn kf(va: u32) -> f32 {
            unsafe { f32::from_bits(lf_checker_rt::global::<u32>(va).read()) }
        }
        #[inline(always)]
        unsafe fn ku(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read() }
        }
        #[inline(always)]
        unsafe fn gb(va: u32) -> u8 {
            unsafe { lf_checker_rt::global::<u8>(va).read() }
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

        let esi = this;
        let head = rd32(esi.wrapping_add(0x24));
        let mode = rd32(head.wrapping_add(0x26c));
        if mode & 0x2000 != 0 || mode & 1 == 0 && rd8(head.wrapping_add(0x29c)) & 4 == 0 {
            wr32(esi.wrapping_add(4), 0);
            wr32(esi.wrapping_add(8), rd32(esi.wrapping_add(0x10)));
            wr32(esi.wrapping_add(0x28), 0);
            wr32(esi.wrapping_add(0x2c), 0);
            return 0;
        }
        let gm = kf(G_MIX);
        let edx = rd32(esi.wrapping_add(0x50));
        let bit = (edx >> 13) & 1;
        let (x4, x3) = if bit != 0 { (kf(G_X4HI), kf(G_X3HI)) } else { (kf(G_X4LO), kf(G_X3LO)) };
        let x1 = rdf(esi.wrapping_add(0x10));
        if edx & 0x20 == 0 && rd32(esi.wrapping_add(0x48)) == 0xffff_ffff {
            let x2 = rdf(esi.wrapping_add(8));
            if x1 > x2 {
                let mut x0 = add(mul(gm, x4), x2);
                wr32(esi.wrapping_add(8), x0.to_bits());
                if x0 > x1 {
                    x0 = x1;
                    wr32(esi.wrapping_add(8), x0.to_bits());
                }
                let b = add(rdf(arg0.wrapping_add(0x350)), kf(K_ONE));
                if x0 > b {
                    wr32(esi.wrapping_add(8), b.to_bits());
                }
            } else if x2 > x1 {
                let x0 = mul(gm, x3);
                let mut x2b = sub(x2, x0);
                wr32(esi.wrapping_add(8), x2b.to_bits());
                if x1 > x2b {
                    x2b = x1;
                    wr32(esi.wrapping_add(8), x2b.to_bits());
                }
                if 0.0 > rdf(esi.wrapping_add(8)) {
                    wr32(esi.wrapping_add(8), 0);
                }
            } else {
                wr32(esi.wrapping_add(8), x1.to_bits());
            }
        }
        wr32(esi.wrapping_add(0x50), edx & !0x20);
        let v1: f32 = lf_checker_rt::callee_cdecl!(SMOOTH_A, f32, rd32(head.wrapping_add(0xaa0)));
        wr32(head.wrapping_add(0xaa0), v1.to_bits());
        let v2: f32 = lf_checker_rt::callee_cdecl!(SMOOTH_B, f32, rd32(head.wrapping_add(0xaa4)));
        wr32(head.wrapping_add(0xaa4), v2.to_bits());
        let d1 = sub(rdf(head.wrapping_add(0xaa4)), rdf(head.wrapping_add(0xaa0)));
        wr32(esi.wrapping_add(0x2c), d1.to_bits());
        let e = sub(rdf(esi.wrapping_add(0x58)), rdf(head.wrapping_add(0xaa4)));
        let bit13 = (rd32(esi.wrapping_add(0x50)) >> 13) & 1;
        if gb(G_8B) != 0 && bit13 != 0 || gb(G_90) != 0 && bit13 == 0 {
            let ae = f32::from_bits(e.to_bits() & ku(MASK_ABS));
            if ae > kf(K_EGATE) {
                let r3a: f32 = lf_checker_rt::callee_cdecl!(REFINE_A, f32, d1.to_bits());
                let r3b: f32 = lf_checker_rt::callee_cdecl!(REFINE_B, f32, e.to_bits());
                if r3a == r3b {
                    wr32(esi.wrapping_add(0x2c), add(d1, e).to_bits());
                }
            }
        }
        let mut d = rdf(esi.wrapping_add(0x2c));
        if d > kf(K_HI) {
            d = sub(d, kf(K_ADJ));
        } else if kf(K_LO) > d {
            d = add(d, kf(K_ADJ));
        }
        wr32(esi.wrapping_add(0x2c), d.to_bits());
        let ad = f32::from_bits(d.to_bits() & ku(MASK_ABS));
        if kf(K_ABS) > ad {
            wr32(esi.wrapping_add(0x30), d.to_bits());
            wr32(esi.wrapping_add(0x2c), 0);
        }
        let mut x6 = kf(G_X6);
        let mut x7 = kf(G_X7);
        let al = if bit != 0 { gb(G_42) != 0 } else { gb(G_43) != 0 };
        let dl = if bit != 0 { gb(G_50) != 0 } else { gb(G_51) != 0 };
        if al {
            let mut xc = add(mul(f32::from_bits(rd32(esi.wrapping_add(0x2c)) & ku(MASK_ABS)), kf(K_INVTAU)), kf(K_HALF));
            if 0.0 > xc {
                xc = 0.0;
            }
            if xc > kf(K_ONE) {
                xc = kf(K_ONE);
            }
            let mut x3s = xc;
            if dl {
                xc = mul(xc, xc);
                if 0.0 > xc {
                    xc = 0.0;
                }
                if xc > kf(K_ONE) {
                    xc = kf(K_ONE);
                }
            }
            if gb(G_E8) == 0 || bit != 0 {
                x6 = mul(xc, x6);
                x7 = mul(xc, x7);
            } else if rdf(esi.wrapping_add(8)) > 0.0 && rdf(esi.wrapping_add(0x20)) > kf(K_ONE) {
                if gb(G_E9) != 0 {
                    x3s = add(mul(x3s, kf(K_HALF)), kf(K_HALF));
                }
                lf_checker_rt::global::<u32>(G_PUB).write(x3s.to_bits());
                lf_checker_rt::global::<u8>(G_97).write(1);
            }
        }
        let divi = if bit13 != 0 { kf(G_DVHI) } else { kf(G_DVLO) };
        let mut x1b = div(rdf(esi.wrapping_add(0x2c)), divi);
        wr32(esi.wrapping_add(0x2c), x1b.to_bits());
        let mut x0 = rdf(head.wrapping_add(0xaa8));
        let edi = rd32(head.wrapping_add(0xab0));
        if edi != 0
            && rd32(edi.wrapping_add(0x28)) & 0x3c0 == 0x80
            && (gb(G_EA) != 0 || bit == 0)
            && rd32(edi.wrapping_add(0x1304)) == 2
        {
            x0 = mul(x0, kf(K_X8));
        }
        x0 = mul(x0, kf(K_H1));
        x0 = mul(x0, kf(K_H2));
        x1b = rdf(esi.wrapping_add(0x2c));
        if x1b > x0 {
            wr32(esi.wrapping_add(0x2c), x0.to_bits());
        } else {
            let nx0 = f32::from_bits(x0.to_bits() ^ ku(MASK_NEG));
            if nx0 > x1b {
                wr32(esi.wrapping_add(0x2c), nx0.to_bits());
            }
        }
        if gb(G_93) != 0 && bit != 0 || gb(G_92) != 0 {
            if gb(G_EB) != 0 || rd32(head.wrapping_add(0x29c)) & 0x800 != 0 {
                if rdf(esi.wrapping_add(8)) >= kf(K_S8MIN) {
                    let x1c = rdf(esi.wrapping_add(0x28));
                    if x1c > 0.0 {
                        if 0.0 > rdf(esi.wrapping_add(0x2c)) {
                            wr32(esi.wrapping_add(0x28), rd32(esi.wrapping_add(0x2c)));
                        }
                    } else if 0.0 > x1c {
                        if rdf(esi.wrapping_add(0x2c)) > 0.0 {
                            wr32(esi.wrapping_add(0x28), rd32(esi.wrapping_add(0x2c)));
                        }
                    }
                }
            }
        }
        let s2c = rdf(esi.wrapping_add(0x2c));
        let s28 = rdf(esi.wrapping_add(0x28));
        if s2c > s28 && s28 > kf(K_LIM) {
            let x0d = add(mul(gm, x6), s28);
            wr32(esi.wrapping_add(0x28), x0d.to_bits());
            if x0d > s2c {
                wr32(esi.wrapping_add(0x28), s2c.to_bits());
            }
            return 0;
        }
        if s28 > s2c && kf(K_DIV1) > s28 {
            let x0d = mul(gm, x6);
            let x1d = sub(s28, x0d);
            wr32(esi.wrapping_add(0x28), x1d.to_bits());
            if s2c > x1d {
                wr32(esi.wrapping_add(0x28), s2c.to_bits());
            }
            return 0;
        }
        let x0d = mul(gm, x7);
        if s2c > s28 {
            let x0e = add(x0d, s28);
            wr32(esi.wrapping_add(0x28), x0e.to_bits());
            if x0e > s2c {
                wr32(esi.wrapping_add(0x28), s2c.to_bits());
            }
            return 0;
        }
        let x1d = sub(s28, x0d);
        wr32(esi.wrapping_add(0x28), x1d.to_bits());
        if s2c > x1d {
            wr32(esi.wrapping_add(0x28), s2c.to_bits());
        }
    }
    0
});
