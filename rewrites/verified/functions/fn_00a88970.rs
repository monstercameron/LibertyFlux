// original: 0x00a88970 input_ui_smoothed_follow_update (proposed)

/// Update one smoothed-follow channel of the input/UI state object.
///
/// `this` points to the state object (working fields at `+0x1430`..`+0x1481`,
/// parameter block pointer at `+0x1468`). `a` and `b` point at two four-word
/// vectors (three floats plus a spare word); `c` and `d` are nullable object
/// pointers consulted for gating; `e` and `f` are forwarded to the refinement
/// callee; `g` is a strength byte (ignored when the word at `+0x1464` exceeds
/// 100); `h` is a scale float; `i` selects the light path (nonzero skips the
/// refinement call).
///
/// The update blends `a` toward `b` by the smoothed factor kept at `+0x1450`,
/// after refreshing that factor from two distance queries (the first callee,
/// shared with the sibling entry, and the second callee), the scale `h`, the
/// gating objects and global tuning values. The spare word of `a` receives
/// zero: the original reads it from an uninitialized stack slot, which the
/// checker fills with zero. Returns a packed status word: on the refinement
/// path the callee result's high word with its low byte duplicated, otherwise
/// the parameter block address's high word with `0x0101`.
///
/// Comparison order follows the original exactly (every ordered float test is
/// a strict greater-than, so NaN always takes the fall-through path), and all
/// arithmetic keeps the original's operand order through exact helpers.
///
/// Original: 0x00A88970 (thiscall, nine stack words).
lf_checker_rt::export!(thiscall, rw_00a88970(
    this: u32, a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, g: u32, h: u32, i: u32,
) -> u32 {
    unsafe {
        const CALLEE_DIST: u32 = 1;
        const CALLEE_DIST2: u32 = 2;
        const CALLEE_REFINE: u32 = 3;
        // Object field offsets.
        const F_VEC0: u32 = 0x1430;
        const F_VEC1: u32 = 0x1440;
        const F_FACTOR: u32 = 0x1450;
        const F_BASE: u32 = 0x1460;
        const F_STRENGTH: u32 = 0x1464;
        const F_PARAMS: u32 = 0x1468;
        const F_COUNT: u32 = 0x1470;
        const F_TIMER: u32 = 0x147c;
        const F_FLAGS: u32 = 0x1480;
        const F_MODE: u32 = 0x1481;
        // Gate object fields.
        const G_FLAGS: u32 = 0x24;
        const G_KIND: u32 = 0x28;
        const C_LIMIT: u32 = 0x130;
        const C_LO: u32 = 0x40;
        const C_HI: u32 = 0x44;
        // Global tuning values (file VAs).
        const K_EPS: u32 = 0x00fe86b4;
        const K_HALF: u32 = 0x00fe8830;
        const K_TENTH: u32 = 0x00fe879c;
        const K_ONE: u32 = 0x00fe88e8;
        const K_CLO: u32 = 0x00ea2954;
        const K_CHI: u32 = 0x00ea2958;
        const K_DLO: u32 = 0x00fe8c10;
        const K_DHI: u32 = 0x00fe8c20;
        const K_BLEND_GATE: u32 = 0x00fe87e8;
        const K_BLEND_MUL: u32 = 0x00fe87e4;
        const K_GAIN: u32 = 0x00fe8b68;
        const K_BASE_ALT: u32 = 0x00fe888c;
        const K_LEN_CAP: u32 = 0x00fe8a24;
        const K_ALT: u32 = 0x00fe8874;
        const S_ENABLE: u32 = 0x011d6fd4;
        const S_CLIMIT: u32 = 0x0103e854;
        const S_BLEND_REF: u32 = 0x0103b734;
        const S_MODE: u32 = 0x011f7060;
        const S_TOKEN_A: u32 = 0x012088b4;
        const S_TOKEN_B: u32 = 0x00f1c040;
        const S_PHASE: u32 = 0x01037720;
        const S_RATE_A: u32 = 0x011735cc;
        const S_RATE_B: u32 = 0x011735dc;
        const S_MIN: u32 = 0x0103e858;
        const S_LEN_B: u32 = 0x0103e85c;
        const S_LEN_A: u32 = 0x0103e860;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(va))) }
        }
        #[inline(always)]
        unsafe fn gu(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
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

        // Two distance queries; scaled first result feeds the timer path.
        let r1: f32 = lf_checker_rt::callee_thiscall!(CALLEE_DIST, f32, this, d, a, b, c);
        let r2: f32 = lf_checker_rt::callee_thiscall!(CALLEE_DIST2, f32, this, d, a, b, c);
        // Scaled first answer: forwarded to the refinement callee.
        let scaled = mul(r1, f32::from_bits(h));
        // Length of (a - vec0), squares summed x, y, z.
        let dx = sub(rdf(a), rdf(this.wrapping_add(F_VEC0)));
        let dy = sub(rdf(a.wrapping_add(4)), rdf(this.wrapping_add(F_VEC0 + 4)));
        let dz = sub(rdf(a.wrapping_add(8)), rdf(this.wrapping_add(F_VEC0 + 8)));
        let len_a = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz)).sqrt();
        // Length of (b - vec1), squares summed y, x, z.
        let ex = sub(rdf(b), rdf(this.wrapping_add(F_VEC1)));
        let ey = sub(rdf(b.wrapping_add(4)), rdf(this.wrapping_add(F_VEC1 + 4)));
        let ez = sub(rdf(b.wrapping_add(8)), rdf(this.wrapping_add(F_VEC1 + 8)));
        let len_b = add(add(mul(ey, ey), mul(ex, ex)), mul(ez, ez)).sqrt();
        // Strength byte, forced off when the strength word is above 100.
        let strength = (this.wrapping_add(F_STRENGTH) as *const u16).read_unaligned() as i16;
        let gval: u32 = if strength > 100 { 0 } else { g & 0xff };
        let saved_flags = (this.wrapping_add(F_FLAGS) as *const u8).read();
        let dh0: u8 = saved_flags >> 7;
        if dh0 != 0 {
            wr32(this.wrapping_add(F_TIMER), 0);
        }
        let eps = gf(K_EPS);
        let dl_len: u8 = if len_a > eps || len_b > eps { 1 } else { 0 };
        // Gate: skip the factor block only when every condition holds and the
        // high bound still clears the second value.
        let mut run_block = true;
        if gu(S_ENABLE) == 1
            && d != 0
            && rd32(d.wrapping_add(G_FLAGS)) & 0x0800_0000 != 0
            && c != 0
            && rd32(c.wrapping_add(C_LIMIT)) < gu(S_CLIMIT)
            && gf(K_CLO) > rdf(c.wrapping_add(C_LO))
            && rdf(c.wrapping_add(C_LO)) > gf(K_CHI)
            && rdf(c.wrapping_add(C_HI)) > gf(K_DLO)
            && gf(K_DHI) > rdf(c.wrapping_add(C_HI))
        {
            run_block = false;
        }
        let mut flag16: u8 = 0;
        if run_block && (gval & 0xff) != 0 && dl_len != 0 {
            if dh0 == 0 {
                wr32(
                    this.wrapping_add(F_TIMER),
                    rd32(this.wrapping_add(F_TIMER)).wrapping_add(1),
                );
            }
            flag16 = 1;
            let params = rd32(this.wrapping_add(F_PARAMS));
            let mut f = rdf(params.wrapping_add(0x10));
            if d != 0 {
                if rd32(d.wrapping_add(G_KIND)) & 0x3c0 == 0xc0
                    && rd32(d.wrapping_add(G_FLAGS)) & 0x0800_0000 != 0
                {
                    f = mul(f, gf(K_HALF));
                } else if saved_flags & 0x40 != 0 {
                    f = gf(K_TENTH);
                }
            }
            if gf(K_BLEND_GATE) > gf(S_BLEND_REF) {
                f = mul(f, gf(K_BLEND_MUL));
            }
            f = mul(f, gf(K_GAIN));
            let mut f2 = mul(f, gf(S_RATE_A));
            if gu(S_MODE) == 1 || gu(S_TOKEN_A) != gu(S_TOKEN_B) || gu(S_PHASE) == 0x12 {
                f2 = mul(f, gf(S_RATE_B));
            }
            let floor = gf(S_MIN);
            let mut o = mul(sub(gf(K_ONE), rdf(this.wrapping_add(F_FACTOR))), f2);
            o = if floor > o { o } else { floor };
            o = add(o, rdf(this.wrapping_add(F_FACTOR)));
            if o > gf(K_ONE) {
                o = gf(K_ONE);
            }
            wrf(this.wrapping_add(F_FACTOR), o);
        }
        // Remember the incoming vectors.
        wr32(this.wrapping_add(F_VEC0), rd32(a));
        wr32(this.wrapping_add(F_VEC0 + 4), rd32(a.wrapping_add(4)));
        wr32(this.wrapping_add(F_VEC0 + 8), rd32(a.wrapping_add(8)));
        wr32(this.wrapping_add(F_VEC0 + 12), rd32(a.wrapping_add(12)));
        wr32(this.wrapping_add(F_VEC1), rd32(b));
        wr32(this.wrapping_add(F_VEC1 + 4), rd32(b.wrapping_add(4)));
        wr32(this.wrapping_add(F_VEC1 + 8), rd32(b.wrapping_add(8)));
        wr32(this.wrapping_add(F_VEC1 + 12), rd32(b.wrapping_add(12)));
        let params = rd32(this.wrapping_add(F_PARAMS));
        let p14 = rd32(params.wrapping_add(0x14));
        let p18 = rd32(params.wrapping_add(0x18));
        let b0 = rdf(b);
        let b4 = rdf(b.wrapping_add(4));
        let b8 = rdf(b.wrapping_add(8));
        // Refinement call, or the light path.
        let (dl_byte, ah_val, mut x2, ret_hold): (u8, u8, f32, u32);
        let (flag16p, flag17p): (u8, u8);
        if (i & 0xff) != 0 {
            dl_byte = 0;
            ah_val = 0;
            x2 = r2;
            ret_hold = params;
            flag16p = flag16;
            flag17p = dl_len;
        } else {
            let probe = [b0.to_bits(), b4.to_bits(), b8.to_bits(), 0u32];
            let mut back: u32 = r2.to_bits();
            // The original passes a pointer to its flag bytes here; the
            // snapshot covers the answer byte, both flags and the low byte
            // of the saved second answer, so mirror that layout.
            let mut dlslot: u32 = ((r2.to_bits() & 0xff) << 24)
                | ((dl_len as u32) << 16)
                | ((flag16 as u32) << 8);
            let ret = lf_checker_rt::callee_thiscall!(
                CALLEE_REFINE,
                u32,
                this,
                core::ptr::addr_of!(probe) as u32,
                a,
                scaled.to_bits(),
                core::ptr::addr_of_mut!(back) as u32,
                r2.to_bits(),
                d,
                e,
                f,
                p14,
                p18,
                core::ptr::addr_of_mut!(dlslot) as u32
            );
            dl_byte = (dlslot & 0xff) as u8;
            ah_val = (ret & 0xff) as u8;
            x2 = f32::from_bits(back);
            ret_hold = ret;
            // The out-byte is scripted as a full word, which also replaces
            // the two flag bytes on the original side; read back the same
            // bytes the original sees after the call.
            flag16p = (dlslot >> 8 & 0xff) as u8;
            flag17p = (dlslot >> 16 & 0xff) as u8;
        }
        let x6 = r2;
        let x7 = b0;
        // Merge the refinement answer into the flags.
        let mut cl: u8 = 0;
        if flag16p != 0 && flag17p != 0 {
            let mut al = (this.wrapping_add(F_FLAGS) as *const u8).read();
            if al & 0x80 == 0 && dl_byte != 0 {
                cl = 1;
            }
            al = (al & 0x7f) | ((dl_byte & 1) << 7);
            (this.wrapping_add(F_FLAGS) as *mut u8).write(al);
        }
        // Ease the base toward its target and count the visit.
        let base = if (this.wrapping_add(F_MODE) as *const u8).read() & 1 != 0 {
            gf(K_ONE)
        } else {
            gf(K_BASE_ALT)
        };
        let old_base = rdf(this.wrapping_add(F_BASE));
        wr32(
            this.wrapping_add(F_COUNT),
            rd32(this.wrapping_add(F_COUNT)).wrapping_add(1),
        );
        let lerp = add(mul(sub(base, old_base), gf(K_TENTH)), old_base);
        wrf(this.wrapping_add(F_BASE), lerp);
        // Fold the answer back into the factor when it leads.
        let mut al = (this.wrapping_add(F_FLAGS) as *const u8).read();
        if ah_val != 0 && rdf(this.wrapping_add(F_FACTOR)) > x2 {
            let mut o = gf(K_ONE);
            if rd32(this.wrapping_add(F_COUNT)) > 0x1e
                && (len_b > gf(S_LEN_B) || len_a > gf(S_LEN_A))
            {
                o = lerp;
            }
            if len_a > gf(K_LEN_CAP) {
                o = gf(K_ONE);
            }
            al = (this.wrapping_add(F_FLAGS) as *const u8).read();
            if al & 0x40 != 0 {
                o = gf(K_ALT);
            }
            if cl != 0 && o > gf(K_HALF) {
                o = gf(K_HALF);
            }
            let t = rdf(this.wrapping_add(F_FACTOR));
            x2 = add(mul(sub(x2, t), o), t);
            wrf(this.wrapping_add(F_FACTOR), x2);
            al = (al & 0xdf) | 0x10;
        } else if al & 0x10 != 0 {
            al = (al & 0xef) | 0x20;
        } else {
            al &= 0xdf;
        }
        (this.wrapping_add(F_FLAGS) as *mut u8).write(al);
        // Light path pins the factor and the status.
        let mut ahf = ah_val;
        if (i & 0xff) != 0 {
            wrf(this.wrapping_add(F_FACTOR), x6);
            ahf = 1;
        }
        // Blend the output vector and pack the status word.
        let t = rdf(this.wrapping_add(F_FACTOR));
        let o0 = add(mul(sub(rdf(a), x7), t), x7);
        let o4 = add(mul(sub(rdf(a.wrapping_add(4)), b4), t), b4);
        let o8 = add(mul(sub(rdf(a.wrapping_add(8)), b8), t), b8);
        wrf(a, o0);
        wrf(a.wrapping_add(4), o4);
        wrf(a.wrapping_add(8), o8);
        wr32(a.wrapping_add(12), 0);
        if (i & 0xff) != 0 {
            (ret_hold & 0xffff_0000) | 0x0101
        } else {
            let lo = ret_hold & 0xff;
            (ret_hold & 0xffff_0000) | (lo << 8) | lo
        }
    }
});
