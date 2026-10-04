// original: 0x00aebff0 timing_followup_select (proposed)

/// Follow-up selector for the timing gate: rate the row, then pick an action.
///
/// Arguments are (`obj`, `row`, `f`, `t`, `n`, `other`). `obj` carries flags
/// at `+0x24`, a link at `+0x4c`, a rate at `+0x50` and a state byte at
/// `+0x63`; `row` selects the mode through bit 3 of its word at `+0x40`;
/// `f` is a float level; the low byte of `t` arms the final store; `n` is a
/// bit index; `other` carries a bit index at `+0x900`, a factor at `+0x934`
/// and a state byte at `+0x1a`. Returns 0 when gated off, 1 when the probe
/// sequence is skipped, 2 when a probe rejects, 3 when a limit trips or the
/// final store runs. Rating is the minimum of the virtual query
/// (`[obj]+0x58`, float) plus the bias global and rate times factor, scaled
/// by the gain global when flag `0x40` is set; the scratch limit starts as
/// the level global and is overwritten only when the link is null and the
/// rating clears its floor. Probes are the virtual slot
/// `[obj]+0x40` plus the two shared probe callees; the bit dance sets one of
/// the low 24 bits of the link's words. The state byte is truncated from a
/// clamped quotient exactly like the original's float-to-int instruction
/// (out-of-range and NaN yield byte 0).
///
/// Cdecl. The original spills temporaries over its own incoming argument
/// slots; the rewrite uses locals (the values are still observed through the
/// outgoing calls). Float order is the original's.
lf_checker_rt::export!(cdecl, rw_00aebff0(obj: u32, row: u32, f: u32, t: u32, n: u32, other: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0x24;
        const LINK: u32 = 0x4c;
        const RATE: u32 = 0x50;
        const STATE: u32 = 0x63;
        const MODEW: u32 = 0x40;
        const VT_Q: u32 = 0x58;
        const VT_P: u32 = 0x40;
        const O_BIT: u32 = 0x900;
        const O_FACT: u32 = 0x934;
        const O_STATE: u32 = 0x1a;
        const L_A: u32 = 0x58;
        const L_B: u32 = 0x54;
        const L_C: u32 = 0x61;
        const G_LEVEL: u32 = 0x0103f714;
        const G_BIAS: u32 = 0x01593bbc;
        const G_FLOOR: u32 = 0x00fe8bd0;
        const G_LK1: u32 = 0x00fe877c;
        const G_LK2: u32 = 0x00fe8b08;
        const G_GAIN: u32 = 0x0103f6c0;
        const G_ONE: u32 = 0x00fe88e8;
        const G_C0: u32 = 0x00fe8830;
        const G_C1: u32 = 0x00fe8c08;
        const G_C2: u32 = 0x00fe8a24;
        const G_C3: u32 = 0x00fe8b48;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn rdg(a: u32) -> f32 {
            unsafe { (lf_checker_rt::global::<f32>(a)).read_unaligned() }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
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
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        /// The original's truncating float-to-int, low byte: out-of-range,
        /// infinite and NaN inputs yield 0x80000000, hence byte 0.
        #[inline(always)]
        fn cvt_byte(x: f32) -> u8 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                0
            } else {
                (x as i32) as u8
            }
        }
        /// Set one of the low 24 bits of `*dst`, selected by `cl`.
        #[inline(always)]
        unsafe fn bit_dance(dst: u32, cl: u8) {
            unsafe {
                let mut edx = 1u32.wrapping_shl(cl as u32);
                let eax = (dst as *const u32).read_unaligned();
                edx |= eax;
                edx ^= eax;
                edx &= 0x00ff_ffff;
                edx ^= eax;
                (dst as *mut u32).write_unaligned(edx);
            }
        }

        let flag0 = rd32(row + MODEW).wrapping_shr(3) & 0xffff_ff01;
        let flags = rd32(obj + FLAGS);
        if flags & 0x20 == 0 {
            return 0;
        }
        let rate = rdf(obj + RATE);
        let fact = rdf(other + O_FACT);
        let mut tmp_a = mul(rate, fact);
        let q: extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute(rd32(rd32(obj) + VT_Q) as usize);
        let qv = add(q(obj), rdg(G_BIAS));
        if !(qv > tmp_a) {
            tmp_a = qv;
        }
        let link = rd32(obj + LINK);
        // The scratch limit starts as the level: on entry the original stores
        // the level global over its second argument slot, which is only
        // overwritten when the link is null and the rating clears its floor.
        let mut tmp_b = rdg(G_LEVEL);
        if link == 0 {
            let mut x0 = rate;
            if !(tmp_a > x0) {
                x0 = tmp_a;
            }
            if x0 > rdg(G_FLOOR) {
                tmp_b = add(mul(x0, rdg(G_LK1)), rdg(G_LK2));
            }
        }
        if flags & 0x40 != 0 {
            tmp_a = mul(rdg(G_GAIN), tmp_a);
        }
        let level = rdg(G_LEVEL);
        let fval = f32::from_bits(f);
        let al = (flag0 & 0xff) as u8;
        let link61 = |l: u32| unsafe { ((l + L_C) as *const u8).read() };
        if al == 0 && link != 0 && link61(link) > 1 {
            let x0 = mul(rdf(obj + RATE), rdf(other + O_FACT));
            let x1 = sub(add(tmp_b, fval), level);
            if x0 > x1 {
                lf_checker_rt::callee_cdecl!(2, u32, obj, f);
                bit_dance(link + L_A, rd32(other + O_BIT) as u8);
                return 3;
            }
        }
        {
            let x0 = mul(rdf(obj + RATE), rdf(other + O_FACT));
            let x1 = sub(add(tmp_b, fval), level);
            if x0 > x1 {
                if al != 0 {
                    let p: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(rd32(rd32(obj) + VT_P) as usize);
                    p(obj);
                    let r1: u32 = lf_checker_rt::callee_cdecl!(4, u32, obj, other, 0u32);
                    if r1 & 0xff != 0 {
                        ((obj + STATE) as *mut u8).write(0xff);
                        return 2;
                    }
                    let r2: u32 = lf_checker_rt::callee_thiscall!(5, u32, obj, other);
                    if r2 & 0xff != 0 {
                        ((obj + STATE) as *mut u8).write(0xff);
                        return 2;
                    }
                    if link == 0 {
                        return 1;
                    }
                    if ((obj + STATE) as *const u8).read() == 0xff {
                        bit_dance(link + L_B, rd32(other + O_BIT) as u8);
                    }
                    if link61(link) <= 1 {
                        return 1;
                    }
                    lf_checker_rt::callee_cdecl!(2, u32, obj, f);
                    return 0;
                }
                if link != 0 {
                    bit_dance(link + L_A, rd32(other + O_BIT) as u8);
                }
            }
        }
        if flags & 0x80000 != 0 {
            return 0;
        }
        if al == 0 {
            return store_tail(obj, t, al, tmp_a, fval, level);
        }
        {
            let x0 = mul(rdf(obj + RATE), rdf(other + O_FACT));
            let x1 = sub(fval, level);
            if !(x0 > x1) {
                return store_tail(obj, t, al, tmp_a, fval, level);
            }
            let p: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(obj) + VT_P) as usize);
            p(obj);
            let r1: u32 = lf_checker_rt::callee_cdecl!(4, u32, obj, other, 0u32);
            if r1 & 0xff != 0 {
                return probe_failed(obj, other);
            }
            let r2: u32 = lf_checker_rt::callee_thiscall!(5, u32, obj, other);
            if r2 & 0xff != 0 {
                return probe_failed(obj, other);
            }
            if ((other + O_STATE) as *const u8).read() != 0 {
                let mut x1 = div(sub(add(level, tmp_b), fval), fval);
                let one = rdg(G_ONE);
                if !(one > x1) {
                    x1 = one;
                }
                if link != 0 {
                    let c0 = rdg(G_C0);
                    let c1 = rdg(G_C1);
                    if c0 > x1 {
                        x1 = mul(mul(x1, rdg(G_C2)), c1);
                    } else {
                        let mut x0 = sub(one, x1);
                        x1 = mul(one, c1);
                        x0 = mul(mul(x0, rdg(G_C2)), c1);
                        ((link + STATE) as *mut u8).write(cvt_byte(x0));
                    }
                } else {
                    x1 = mul(x1, rdg(G_C1));
                    let x0 = ((obj + STATE) as *const u8).read() as f32;
                    if !(x0 > x1) {
                        x1 = x0;
                    }
                }
                ((obj + STATE) as *mut u8).write(cvt_byte(x1));
            }
            let mask = 1u32.wrapping_shl(n);
            lf_checker_rt::callee_cdecl!(6, u32, obj, f, mask, 0u32);
            0
        }
    }
});

/// Shared tail used from inside the export: probe rejection stores 0xff and
/// returns 0 unless the peer state byte is clear, which returns 0 directly.
#[inline(always)]
unsafe fn probe_failed(obj: u32, other: u32) -> u32 {
    unsafe {
        if ((other + 0x1a) as *const u8).read() == 0 {
            return 0;
        }
        ((obj + 0x63) as *mut u8).write(0xff);
        0
    }
}

/// Shared tail: the final store path past the last two gates. The vtable
/// probe runs only when the mode flag byte is nonzero (the fall-through
/// path), never on the direct path (whose jump tested it clear).
#[inline(always)]
unsafe fn store_tail(obj: u32, t: u32, al: u8, tmp_a: f32, fval: f32, level: f32) -> u32 {
    unsafe {
        let bump = (lf_checker_rt::global::<f32>(0x00fe8b48)).read_unaligned();
        let x2 = core::hint::black_box(fval)
            - core::hint::black_box(
                core::hint::black_box(level) + core::hint::black_box(bump),
            );
        if !(tmp_a > x2) {
            return 0;
        }
        if (t & 0xff) as u8 == 0 {
            return 0;
        }
        if al != 0 {
            let vt = (obj as *const u32).read_unaligned();
            let slot = ((vt + 0x40) as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
            f(obj);
        }
        3
    }
}
