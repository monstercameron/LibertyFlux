// original: 0x009E3F10 ped_pose_blend_switch (proposed)

/// Blend a ped pose vector by kind: copy the source record, then run one of
/// thirteen float cases selected by `kind` (STAGE 1: kinds 1-10 and the
/// default path; kinds 11-13 in stage 2).
///
/// `this` carries the blend object at `+0x1bc`, whose context at `+0x20`
/// supplies the basis vectors; `src` is the source record, `dst` the
/// destination. The prologue copies nine dwords verbatim (skipping `+0x0c` and `+0x1c`)
/// plus the three floats at `+0x30`, and forms a scale from the table entry
/// selected by the signed index at `+0x2e` combined with a sign chosen by a
/// dot product (negative picks -1.0, otherwise 1.0; NaN keeps 1.0). Each
/// case reloads the output triple from the context, reads one table float,
/// and runs a fused multiply-add variant into it: kinds 1-4 add the scaled
/// basis back in, kind 5 subtracts it, kinds 6-7 omit it, kinds 8-9 subtract
/// it, kind 10 runs a matrix accumulation. Out-of-range kinds answer 1 with
/// only the prologue copies done.
///
/// The low byte answers 1 (0 only on the stage-2 call paths); the upper
/// three bytes are the last value `eax` held with the low byte replaced.
/// Float operation order is the original's, pinned through `black_box`.
///
/// Original: 0x009E3F10 (thiscall, this + three stack words).
lf_checker_rt::export!(thiscall, rw_009E3F10(this: u32, kind: u32, src: u32, dst: u32) -> u32 {
    unsafe {
        const BLEND_OFF: u32 = 0x1BC;
        const CTX_OFF: u32 = 0x20;
        const INDEX_OFF: u32 = 0x2E;
        const TABLE_ADDR: u32 = 0x01295CD8;
        const G_ONE: u32 = 0x00FE88E8;
        const G_NEG_ONE: u32 = 0x00FE8D94;
        const G_BIAS: u32 = 0x00FE8828;
        const G_HALF: u32 = 0x00FE8830;
        const G_TWO: u32 = 0x00FE8A24;
        const LOW_MASK: u32 = 0xFFFF_FF00;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { ((a as *mut u32).write_unaligned(v.to_bits())) }
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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        let g = |va: u32| -> f32 { rdf(lf_checker_rt::relocated(va)) };
        let edi = rd32(this.wrapping_add(BLEND_OFF));
        let ctx = rd32(edi.wrapping_add(CTX_OFF));
        // Dot of (src - ctx) offsets against the first basis row.
        let dy = sub(rdf(src.wrapping_add(0x34)), rdf(ctx.wrapping_add(0x34)));
        let dx = sub(rdf(src.wrapping_add(0x30)), rdf(ctx.wrapping_add(0x30)));
        let dz = sub(rdf(src.wrapping_add(0x38)), rdf(ctx.wrapping_add(0x38)));
        let mut dot = mul(rdf(ctx.wrapping_add(0x04)), dy);
        dot = add(dot, mul(dx, rdf(ctx.wrapping_add(0x00))));
        dot = add(dot, mul(rdf(ctx.wrapping_add(0x08)), dz));
        let mut four = g(G_ONE);
        if 0.0f32 > dot {
            four = g(G_NEG_ONE);
        }
        let idx = rd16(edi.wrapping_add(INDEX_OFF)) as i16 as i32 as u32;
        let table = lf_checker_rt::relocated(TABLE_ADDR);
        let entry = rd32(table.wrapping_add(idx.wrapping_mul(4)));
        let five_base = rdf(ctx.wrapping_add(0x04));
        let six_base = rdf(ctx.wrapping_add(0x08));
        let mut t = add(rdf(entry.wrapping_add(0x30)), g(G_BIAS));
        t = mul(t, four);
        four = mul(rdf(ctx.wrapping_add(0x00)), t);
        // Verbatim copies (dst+0x0c skipped).
        for off in [0u32, 4, 8, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28] {
            wr32(dst.wrapping_add(off), rd32(src.wrapping_add(off)));
        }
        let esi = dst.wrapping_add(0x30);
        wr32(esi, rd32(src.wrapping_add(0x30)));
        wr32(esi.wrapping_add(4), rd32(src.wrapping_add(0x34)));
        wr32(esi.wrapping_add(8), rd32(src.wrapping_add(0x38)));
        let five = mul(five_base, t);
        let six = mul(six_base, t);
        let eax_kind = kind.wrapping_sub(1);
        if eax_kind > 0x0C {
            return (eax_kind & LOW_MASK) | 1;
        }
        // Shared case prologue: reload the output triple from the context.
        let mut eax: u32 = 0;
        let reload = |esi: u32, dst: u32, src: u32, ctx: u32| {
            wr32(esi, rd32(ctx.wrapping_add(0x30)));
            wrf(esi.wrapping_add(4), rdf(ctx.wrapping_add(0x34)));
            wrf(esi.wrapping_add(8), rdf(ctx.wrapping_add(0x38)));
            wr32(esi.wrapping_add(0x0C), rd32(ctx.wrapping_add(0x3C)));
            wr32(dst.wrapping_add(0x38), rd32(src.wrapping_add(0x38)));
        };
        let entry_of = |edi: u32| -> u32 {
            let i = rd16(edi.wrapping_add(INDEX_OFF)) as i16 as i32 as u32;
            rd32(table.wrapping_add(i.wrapping_mul(4)))
        };
    /// Tail T1: full FMA with the scaled basis added back.
    unsafe fn tail_t1(esi: u32, ctx: u32, tt: f32, four: f32, five: f32, six: f32) {
        unsafe {
            #[inline(always)]
            fn add(a: f32, b: f32) -> f32 {
                core::hint::black_box(a) + core::hint::black_box(b)
            }
            #[inline(always)]
            fn mul(a: f32, b: f32) -> f32 {
                core::hint::black_box(a) * core::hint::black_box(b)
            }
            let rdf = |a: u32| -> f32 {
                unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
            };
            let wrf = |a: u32, v: f32| {
                unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
            };
            let mut x1 = mul(rdf(ctx.wrapping_add(0x10)), tt);
            x1 = add(x1, rdf(esi));
            let mut x2 = mul(rdf(ctx.wrapping_add(0x14)), tt);
            x1 = add(x1, four);
            let mut x3 = mul(rdf(ctx.wrapping_add(0x18)), tt);
            wrf(esi, x1);
            x2 = add(x2, rdf(esi.wrapping_add(4)));
            x2 = add(x2, five);
            wrf(esi.wrapping_add(4), x2);
            x3 = add(x3, rdf(esi.wrapping_add(8)));
            x3 = add(x3, six);
            wrf(esi.wrapping_add(8), x3);
        }
    }

    /// Tail T1 mid-point (kind 3 joins here with x1 formed).
    unsafe fn tail_t1_mid(
        esi: u32, ctx: u32, x1: f32, tt: f32, four: f32, five: f32, six: f32,
    ) {
        unsafe {
            #[inline(always)]
            fn add(a: f32, b: f32) -> f32 {
                core::hint::black_box(a) + core::hint::black_box(b)
            }
            #[inline(always)]
            fn mul(a: f32, b: f32) -> f32 {
                core::hint::black_box(a) * core::hint::black_box(b)
            }
            let rdf = |a: u32| -> f32 {
                unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
            };
            let wrf = |a: u32, v: f32| {
                unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
            };
            let mut x1 = x1;
            let mut x2 = mul(rdf(ctx.wrapping_add(0x14)), tt);
            x1 = add(x1, four);
            let mut x3 = mul(rdf(ctx.wrapping_add(0x18)), tt);
            wrf(esi, x1);
            x2 = add(x2, rdf(esi.wrapping_add(4)));
            x2 = add(x2, five);
            wrf(esi.wrapping_add(4), x2);
            x3 = add(x3, rdf(esi.wrapping_add(8)));
            x3 = add(x3, six);
            wrf(esi.wrapping_add(8), x3);
        }
    }

    /// Tail T2 mid-point (kinds 6,7 join here with x1 formed).
    unsafe fn tail_t2_mid(esi: u32, ctx: u32, x1: f32, tt: f32) {
        unsafe {
            #[inline(always)]
            fn add(a: f32, b: f32) -> f32 {
                core::hint::black_box(a) + core::hint::black_box(b)
            }
            #[inline(always)]
            fn mul(a: f32, b: f32) -> f32 {
                core::hint::black_box(a) * core::hint::black_box(b)
            }
            let rdf = |a: u32| -> f32 {
                unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
            };
            let wrf = |a: u32, v: f32| {
                unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
            };
            let mut x1 = x1;
            let mut x2 = rdf(ctx.wrapping_add(0x14));
            x1 = add(x1, rdf(esi));
            let mut x3 = rdf(ctx.wrapping_add(0x18));
            x2 = mul(x2, tt);
            x3 = mul(x3, tt);
            wrf(esi, x1);
            x2 = add(x2, rdf(esi.wrapping_add(4)));
            wrf(esi.wrapping_add(4), x2);
            x3 = add(x3, rdf(esi.wrapping_add(8)));
            wrf(esi.wrapping_add(8), x3);
        }
    }

    /// Tail T3: FMA with basis terms subtracted.
    unsafe fn tail_t3(esi: u32, ctx: u32, tt: f32, four: f32, five: f32, six: f32) {
        unsafe {
            #[inline(always)]
            fn add(a: f32, b: f32) -> f32 {
                core::hint::black_box(a) + core::hint::black_box(b)
            }
            #[inline(always)]
            fn mul(a: f32, b: f32) -> f32 {
                core::hint::black_box(a) * core::hint::black_box(b)
            }
            #[inline(always)]
            fn sub(a: f32, b: f32) -> f32 {
                core::hint::black_box(a) - core::hint::black_box(b)
            }
            let rdf = |a: u32| -> f32 {
                unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
            };
            let wrf = |a: u32, v: f32| {
                unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
            };
            let mut x1 = mul(tt, rdf(ctx.wrapping_add(0x10)));
            let mut x2 = rdf(ctx.wrapping_add(0x14));
            let mut x3 = rdf(ctx.wrapping_add(0x18));
            x1 = add(x1, rdf(esi));
            x2 = mul(x2, tt);
            x3 = mul(x3, tt);
            x1 = sub(x1, four);
            wrf(esi, x1);
            x2 = add(x2, rdf(esi.wrapping_add(4)));
            x2 = sub(x2, five);
            wrf(esi.wrapping_add(4), x2);
            x3 = add(x3, rdf(esi.wrapping_add(8)));
            x3 = sub(x3, six);
            wrf(esi.wrapping_add(8), x3);
        }
    }
        match eax_kind {
            // Kinds 1,2: table float times one half, tail T1.
            0 | 1 => {
                reload(esi, dst, src, ctx);
                let e = entry_of(edi);
                eax = e;
                let f = if eax_kind == 0 {
                    rdf(e.wrapping_add(0x24))
                } else {
                    rdf(e.wrapping_add(0x34))
                };
                let tt = mul(f, g(G_HALF));
                tail_t1(esi, ctx, tt, four, five, six);
                (eax & LOW_MASK) | 1
            }
            // Kind 3: table float times basis directly, tail T1 mid.
            2 => {
                reload(esi, dst, src, ctx);
                let e = entry_of(edi);
                eax = e;
                let tt = rdf(e.wrapping_add(0x24));
                let mut x1 = mul(tt, rdf(ctx.wrapping_add(0x10)));
                x1 = add(x1, rdf(esi));
                tail_t1_mid(esi, ctx, x1, tt, four, five, six);
                (eax & LOW_MASK) | 1
            }
            // Kind 4: table float without the half scale, tail T1.
            3 => {
                reload(esi, dst, src, ctx);
                let e = entry_of(edi);
                eax = e;
                let tt = rdf(e.wrapping_add(0x34));
                tail_t1(esi, ctx, tt, four, five, six);
                (eax & LOW_MASK) | 1
            }
            // Kind 5: subtract the scaled basis, no reload.
            4 => {
                let two = g(G_TWO);
                let f4 = mul(four, two);
                let f5 = mul(five, two);
                let f6 = mul(six, two);
                wrf(esi, sub(rdf(esi), f4));
                wrf(esi.wrapping_add(4), sub(rdf(esi.wrapping_add(4)), f5));
                wrf(esi.wrapping_add(8), sub(rdf(esi.wrapping_add(8)), f6));
                (eax_kind & LOW_MASK) | 1
            }
            // Kinds 6,7: tail T2 (no basis terms). The x1 operand order
            // differs: kind 6 forms tt*ctx10, kind 7 ctx10*tt.
            5 | 6 => {
                reload(esi, dst, src, ctx);
                let e = entry_of(edi);
                eax = e;
                let bias = g(G_BIAS);
                let c10 = rdf(ctx.wrapping_add(0x10));
                let (x1, tt) = if eax_kind == 5 {
                    let tt = add(rdf(e.wrapping_add(0x34)), bias);
                    (mul(tt, c10), tt)
                } else {
                    let tt = sub(rdf(e.wrapping_add(0x24)), bias);
                    (mul(c10, tt), tt)
                };
                tail_t2_mid(esi, ctx, x1, tt);
                (eax & LOW_MASK) | 1
            }
            // Kinds 8,9: tail T3 (basis terms subtracted).
            7 | 8 => {
                reload(esi, dst, src, ctx);
                let e = entry_of(edi);
                eax = e;
                let bias = g(G_BIAS);
                let tt = if eax_kind == 7 {
                    add(rdf(e.wrapping_add(0x34)), bias)
                } else {
                    sub(rdf(e.wrapping_add(0x24)), bias)
                };
                tail_t3(esi, ctx, tt, four, five, six);
                (eax & LOW_MASK) | 1
            }
            // Kind 10: matrix accumulation.
            9 => {
                wr32(esi, 0);
                wr32(esi.wrapping_add(4), 0);
                wr32(esi.wrapping_add(8), 0);
                let e = entry_of(edi);
                let mut tt = add(rdf(e.wrapping_add(0x38)), g(G_ONE));
                let f4 = mul(rdf(ctx.wrapping_add(0x24)), tt);
                let f5 = mul(rdf(ctx.wrapping_add(0x28)), tt);
                let f6 = mul(tt, rdf(ctx.wrapping_add(0x20)));
                wrf(esi.wrapping_add(4), f4);
                wrf(esi.wrapping_add(8), f5);
                wrf(esi, f6);
                let c = ctx;
                let mut x3 = mul(rdf(c.wrapping_add(0x10)), f4);
                x3 = add(x3, mul(rdf(c.wrapping_add(0x00)), f6));
                x3 = add(x3, mul(rdf(c.wrapping_add(0x20)), f5));
                x3 = add(x3, rdf(c.wrapping_add(0x30)));
                let mut x2 = mul(rdf(c.wrapping_add(0x14)), f4);
                x2 = add(x2, mul(rdf(c.wrapping_add(0x04)), f6));
                x2 = add(x2, mul(rdf(c.wrapping_add(0x24)), f5));
                x2 = add(x2, rdf(c.wrapping_add(0x34)));
                let mut x1 = mul(rdf(c.wrapping_add(0x18)), f4);
                x1 = add(x1, mul(rdf(c.wrapping_add(0x08)), f6));
                x1 = add(x1, mul(rdf(c.wrapping_add(0x28)), f5));
                x1 = add(x1, rdf(c.wrapping_add(0x38)));
                wrf(esi, x3);
                wrf(esi.wrapping_add(4), x2);
                wrf(esi.wrapping_add(8), x1);
                wr32(esi.wrapping_add(0x0C), 0);
                eax = ctx;
                (eax & LOW_MASK) | 1
            }
            // Kinds 11-13: stage 2 (call cases).
            _ => unreachable!("stage 2"),
        }
    }





});
