// original: 0x00CBF840 task_steer_update (proposed)

/// Refresh a ped task's steering state from its head object.
///
/// `this` is the task object (head pointer at `+0x24`, outputs at `+0x28`
/// and `+0x2c`). The head's two filter words are passed through a smoothing
/// call each and their difference, normalised into range, seeds the lateral
/// output; a scaled head word seeds the other channels. A virtual
/// slot-0x4a check on the head then decides between two scaling regimes for
/// the channels (picked through two further polls), the lateral output is
/// divided by the regime's divisor and clamped, and the two outputs are
/// reconciled with two min/max tails. Every below-or-equal branch after a float compare takes the
/// unordered (NaN) side; every `ja` takes the ordered-greater side only.
///
/// Original: 0x00CBF840 (thiscall, no stack arguments, no return value).
lf_checker_rt::export!(thiscall, rw_00cbf840(this: u32) -> u32 {
    unsafe {
        const SMOOTH_A: u32 = 1;
        const SMOOTH_B: u32 = 2;
        const SLOT_CHECK: u32 = 3;
        const POLL_A: u32 = 4;
        const POLL_B: u32 = 5;
        const FETCH: u32 = 6;
        const K_HI: u32 = 0x00fe_8aa0;
        const K_ADJ: u32 = 0x00fe_8aec;
        const K_LO: u32 = 0x00fe_8dc4;
        const K_ABS: u32 = 0x00ed_9180;
        const MASK_ABS: u32 = 0x00fe_8f80;
        const K_H1: u32 = 0x00fe_8728;
        const K_H2: u32 = 0x00fe_8b68;
        const K_DIV0: u32 = 0x00fe_87e8;
        const K_DIV1: u32 = 0x00fe_870c;
        const K_S1: u32 = 0x00fe_8b20;
        const K_S2: u32 = 0x00fe_8b38;
        const K_S0: u32 = 0x00fe_8ab8;
        const K_LIM: u32 = 0x00fe_8d48;
        const MASK_NEG: u32 = 0x00fe_8fa0;
        const G_CH0: u32 = 0x0105_1500;
        const G_CH1: u32 = 0x0105_1504;
        const G_MIX: u32 = 0x0117_35bc;

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
        let obj = rd32(esi.wrapping_add(0x24));
        let v1: f32 = lf_checker_rt::callee_cdecl!(SMOOTH_A, f32, rd32(obj.wrapping_add(0xaa0)));
        wr32(obj.wrapping_add(0xaa0), v1.to_bits());
        let v2: f32 = lf_checker_rt::callee_cdecl!(SMOOTH_B, f32, rd32(obj.wrapping_add(0xaa4)));
        wr32(obj.wrapping_add(0xaa4), v2.to_bits());
        let k_hi = kf(K_HI);
        let k_adj = kf(K_ADJ);
        let mut d = sub(rdf(obj.wrapping_add(0xaa4)), rdf(obj.wrapping_add(0xaa0)));
        if d > k_hi {
            d = sub(d, k_adj);
        } else if kf(K_LO) > d {
            d = add(d, k_adj);
        }
        wr32(esi.wrapping_add(0x2c), d.to_bits());
        let ad = f32::from_bits(d.to_bits() & ku(MASK_ABS));
        if kf(K_ABS) > ad {
            wr32(esi.wrapping_add(0x30), d.to_bits());
            d = 0.0;
            wr32(esi.wrapping_add(0x2c), 0);
        }
        let mut h = mul(rdf(obj.wrapping_add(0xaa8)), kf(K_H1));
        h = mul(h, kf(K_H2));
        let mut x2 = kf(G_CH0);
        let mut x3 = kf(G_CH1);
        let mut divi = kf(K_DIV0);
        let gm = kf(G_MIX);
        if obj != 0 {
            let vt = rd32(obj);
            let tgt = rd32(vt.wrapping_add(0x128));
            let slot: extern "thiscall" fn(u32) -> u32 =
                unsafe { core::mem::transmute(tgt as usize) };
            if slot(obj) & 0xff != 0 {
                let r4: u32 = lf_checker_rt::callee_cdecl!(POLL_A, u32, 1u32);
                let bl = if r4 & 0xff != 0 {
                    1u32
                } else {
                    let r5: u32 = lf_checker_rt::callee_cdecl!(POLL_B, u32,);
                    (r5 & 0xff != 0) as u32
                };
                let r6: u32 = lf_checker_rt::callee_cdecl!(FETCH, u32,);
                if rd8(r6.wrapping_add(0x328d)) != 0 {
                    if bl != 0 {
                        x2 = mul(x2, kf(K_S1));
                        x3 = mul(x3, kf(K_S1));
                        h = mul(h, kf(K_S2));
                        divi = kf(K_DIV1);
                    } else {
                        let s = kf(K_S0);
                        x2 = mul(x2, s);
                        x3 = mul(x3, s);
                        h = mul(h, s);
                    }
                }
            }
        }
        let x4 = kf(K_DIV1);
        let mut x0 = div(rdf(esi.wrapping_add(0x2c)), divi);
        wr32(esi.wrapping_add(0x2c), x0.to_bits());
        if x0 > h {
            wr32(esi.wrapping_add(0x2c), h.to_bits());
        } else {
            let nh = f32::from_bits(h.to_bits() ^ ku(MASK_NEG));
            if nh > x0 {
                wr32(esi.wrapping_add(0x2c), nh.to_bits());
            }
        }
        let s2c = rdf(esi.wrapping_add(0x2c));
        let s28 = rdf(esi.wrapping_add(0x28));
        if s2c > s28 && s28 > kf(K_LIM) {
            x0 = add(mul(gm, x2), s28);
            wr32(esi.wrapping_add(0x28), x0.to_bits());
            if x0 > s2c {
                wr32(esi.wrapping_add(0x28), s2c.to_bits());
            }
            return 0;
        }
        if s28 > s2c && x4 > s28 {
            x0 = mul(gm, x2);
            let x1 = sub(s28, x0);
            wr32(esi.wrapping_add(0x28), x1.to_bits());
            if s2c > x1 {
                wr32(esi.wrapping_add(0x28), s2c.to_bits());
            }
            return 0;
        }
        let f1_taken = s2c > s28;
        x0 = mul(gm, x3);
        if f1_taken {
            x0 = add(x0, s28);
            wr32(esi.wrapping_add(0x28), x0.to_bits());
            if x0 > s2c {
                wr32(esi.wrapping_add(0x28), s2c.to_bits());
            }
            return 0;
        }
        let x1 = sub(s28, x0);
        wr32(esi.wrapping_add(0x28), x1.to_bits());
        if s2c > x1 {
            wr32(esi.wrapping_add(0x28), s2c.to_bits());
        }
    }
    0
});
