// original: 0x00ace640 audio_voice_render (proposed)

/// Render one audio voice for the current frame: gate on audibility, resolve
/// the voice's output target, and emit two 11-word mix-fill calls plus two
/// engine notifications.
///
/// `thiscall`: `this` is the voice object, `arg0` the listener/target object,
/// `arg1`/`arg3` are floats, only the low byte of `arg2` is read. Returns
/// nothing meaningful (`ret: none` in the contract: every exit path leaves
/// whatever the last callee or gate left in eax).
///
/// Behaviour in order: measure the voice (`cdecl/1` returning a double,
/// converted to float) and exit when it exceeds `arg3 * 900`; resolve the
/// voice kind through the manager at the fabricated global (`thiscall/1`,
/// six indirect sites sharing vtable slot `+0x14`) and exit on the release
/// bit or bit 27 of `this+0xD8`; derive a 0..3 code from whether the timer at
/// `this+0x160` is zero and whether `arg1 >= 0.6`; dispatch on the `arg2`
/// byte (direct engine query or two more kind resolutions feeding the query);
/// exit unless the energy at `this+0x130` beats the queried level; pick a
/// blend factor (timer path, kind-match path, or measured path with a
/// 0..1 clamp); exit unless the orientation dot product beats the global
/// limit; stamp the listener when the range test fails; normalise the
/// direction vector; emit fill 1 (11 words: listener, two frame structs,
/// two computed floats, codes) and engine notify 1; run the table-driven
/// tail gates (listener word, flag bits 12/11, runtime table byte) and emit
/// fill 2 plus notify 2 with the adjusted vector.
///
/// Float comparison notes (all derived from the jumps, NaN included):
/// `ja` after `comiss` is Rust `>`; `jbe` is `!(>)`; `jb` is `<`; `setae`
/// and `cmovae` are `>=`; the `ucomiss`+`lahf`+`(an instruction of the original)`+`jp` idiom is
/// Rust `!=` against `+0.0`. Arithmetic uses explicit-operand-order helpers
/// matching the SSE NaN rule (signalling NaN wins, else quiet NaN
/// destination-first, else the hardware op), because LLVM reorders operands
/// at opt-level 3 and two-NaN payload forwarding is positional.
///
/// Two frame words are read without ever being written (`B+0x20`, feeding
/// the fill struct, and `B+0x2C`, inside the notify struct): the contract
/// fills uninitialised stack with zero, so the rewrite hard-codes `0.0`/`0`
/// for them. In the game these words hold caller garbage.
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

#[inline(always)]
fn is_nan_bits(b: u32) -> bool {
    (b & 0x7F80_0000) == 0x7F80_0000 && (b & 0x007F_FFFF) != 0
}
#[inline(always)]
fn is_snan_bits(b: u32) -> bool {
    (b & 0x7F80_0000) == 0x7F80_0000
        && (b & 0x007F_FFFF) != 0
        && (b & 0x0040_0000) == 0
}
#[inline(always)]
fn quiet(b: u32) -> u32 {
    b | 0x0040_0000
}
#[inline(always)]
// Scalar-SSE NaN rule, measured on the stock worker (two trials of
// 0x00acce10 pin it from opposite sides): a signalling NaN wins over
// everything (destination sNaN, then source sNaN), else a quiet NaN wins
// destination-first, else the hardware op. Trial 22: addss of canonical
// destination NaN and payload source sNaN forwards the source quieted.
// Trial 0: an upstream op of two quiet NaNs forwards the destination.
fn add_ss(d: f32, s: f32) -> f32 {
    let (db, sb) = (d.to_bits(), s.to_bits());
    if is_snan_bits(db) {
        return f32::from_bits(quiet(db));
    }
    if is_snan_bits(sb) {
        return f32::from_bits(quiet(sb));
    }
    if is_nan_bits(db) {
        return f32::from_bits(quiet(db));
    }
    if is_nan_bits(sb) {
        return f32::from_bits(quiet(sb));
    }
    d + s
}
#[inline(always)]
fn sub_ss(d: f32, s: f32) -> f32 {
    let (db, sb) = (d.to_bits(), s.to_bits());
    if is_snan_bits(db) {
        return f32::from_bits(quiet(db));
    }
    if is_snan_bits(sb) {
        return f32::from_bits(quiet(sb));
    }
    if is_nan_bits(db) {
        return f32::from_bits(quiet(db));
    }
    if is_nan_bits(sb) {
        return f32::from_bits(quiet(sb));
    }
    d - s
}
#[inline(always)]
fn mul_ss(d: f32, s: f32) -> f32 {
    let (db, sb) = (d.to_bits(), s.to_bits());
    if is_snan_bits(db) {
        return f32::from_bits(quiet(db));
    }
    if is_snan_bits(sb) {
        return f32::from_bits(quiet(sb));
    }
    if is_nan_bits(db) {
        return f32::from_bits(quiet(db));
    }
    if is_nan_bits(sb) {
        return f32::from_bits(quiet(sb));
    }
    d * s
}
#[inline(always)]
fn div_ss(d: f32, s: f32) -> f32 {
    let (db, sb) = (d.to_bits(), s.to_bits());
    if is_snan_bits(db) {
        return f32::from_bits(quiet(db));
    }
    if is_snan_bits(sb) {
        return f32::from_bits(quiet(sb));
    }
    if is_nan_bits(db) {
        return f32::from_bits(quiet(db));
    }
    if is_nan_bits(sb) {
        return f32::from_bits(quiet(sb));
    }
    d / s
}

unsafe fn render(
    this: u32,
    arg0: u32,
    arg1: f32,
    arg2: u32,
    arg3: f32,
    stamp: bool,
) -> u32 {
        const MGR_GLOBAL: u32 = 0x018B8968;
        // The original loads these engine pointers as immediates, which the
        // loader relocates; derive them through the relocated base.
        let engine_this: u32 = relocated(0x016D9F58);
        let notify_this: u32 = relocated(0x0139C280);
        const KIND_SLOT: u32 = 0x14;
        const ABS_MASK: u32 = 0x00FE8F80;

        let absmask = *global::<u32>(ABS_MASK);
        // Call 1: voice measure, double answer converted to float.
        let measured: f64 = callee_cdecl!(2, f64, this.wrapping_add(0xA0));
        let got = measured as f32;
        // Gate A: exit when the measure beats arg3 * 900.
        let prod = mul_ss(arg3, *global::<f32>(0x00E9CAE0));
        if got > prod {
            return 0;
        }
        // Indirect kind resolution #1 through the manager vtable.
        let mgr = *global::<u32>(MGR_GLOBAL);
        let vt = *(mgr as *const u32);
        let kind_fn: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*((vt.wrapping_add(KIND_SLOT)) as *const u32) as usize);
        let d8 = *((this.wrapping_add(0xD8)) as *const u32);
        let ans1 = kind_fn(mgr, d8);
        if (((*((ans1.wrapping_add(0x38)) as *const u32)) >> 1) & 1) != 0 {
            return 0;
        }
        // Gate B: bit 27 of this+0xD8 (reached via the saved frame slot).
        if ((d8 >> 27) & 1) != 0 {
            return 0;
        }
        // lahf idiom #1: code from timer-zeroness and arg1 >= 0.6.
        let timer = *((this.wrapping_add(0x160)) as *const f32);
        let k06 = *global::<f32>(0x00ECA7F4);
        let code: u32 = if timer != 0.0 {
            if arg1 >= k06 { 2 } else { 0 }
        } else if arg1 >= k06 {
            3
        } else {
            1
        };
        // Engine query, direct or via two more kind resolutions.
        let edi = code;
        let (qans, arg5): (u32, u32) = if (arg2 as u8) == 0 {
            let w1 = *((kind_fn(mgr, d8).wrapping_add(0x20)) as *const u16) as u32;
            let w2 = *((kind_fn(mgr, d8).wrapping_add(0x20)) as *const u16) as u32;
            (callee_thiscall!(3, u32, engine_this, edi, w2), w1)
        } else {
            (callee_thiscall!(3, u32, engine_this, edi, 0x22), 0x22)
        };
        let flags = *((this.wrapping_add(0x164)) as *const u32);
        let bit4 = ((flags >> 4) & 1) != 0;
        let bit5 = ((flags >> 5) & 1) != 0;
        let mut x1 = *(qans as *const f32);
        // Timer override + flag byte (0 or 1; upper bytes stay 0).
        let flag: u32 = if (bit4 || bit5) && timer > 0.0 {
            x1 = *global::<f32>(0x00FE879C);
            1
        } else {
            0
        };
        // Energy gate: |this+0x130| must beat the level.
        let a130 = f32::from_bits(*((this.wrapping_add(0x130)) as *const u32) & absmask);
        if !(a130 > x1) {
            return 0;
        }
        let one = *global::<f32>(0x00FE88E8);
        if bit4 || bit5 {
            x1 = *((this.wrapping_add(0x168)) as *const f32);
            if !(one > x1) {
                x1 = one;
            }
            if !(x1 > 0.0) {
                x1 = 0.0;
            }
        } else {
            // Kind-match chain: 8, 6, 0x17, else the measured path.
            let a4 = kind_fn(mgr, d8);
            if *((a4.wrapping_add(0x20)) as *const u16) == 8 {
                x1 = one;
            } else {
                let a5 = kind_fn(mgr, d8);
                if *((a5.wrapping_add(0x20)) as *const u16) == 6 {
                    x1 = one;
                } else {
                    let a6 = kind_fn(mgr, d8);
                    if *((a6.wrapping_add(0x20)) as *const u16) == 0x17 {
                        x1 = one;
                    } else {
                        let t = sub_ss(a130, *(qans as *const f32));
                        x1 = mul_ss(t, *global::<f32>(0x00FE87E4));
                        if !(one > x1) {
                            x1 = one;
                        }
                        if !(x1 > 0.0) {
                            x1 = 0.0;
                        }
                    }
                }
            }
        }
        // Orientation dot product against the global limit.
        let dx = *((arg0.wrapping_add(0x20)) as *const u32);
        let t0 = mul_ss(
            *((dx.wrapping_add(0x20)) as *const f32),
            *((this.wrapping_add(0xC0)) as *const f32),
        );
        let mut dot = mul_ss(
            *((dx.wrapping_add(0x24)) as *const f32),
            *((this.wrapping_add(0xC4)) as *const f32),
        );
        dot = add_ss(dot, t0);
        let t2 = mul_ss(
            *((dx.wrapping_add(0x28)) as *const f32),
            *((this.wrapping_add(0xC8)) as *const f32),
        );
        dot = add_ss(dot, t2);
        if *global::<f32>(0x0103F3D0) > dot {
            return 0;
        }
        // Hemisphere stamp + square for the fill below.
        let half = *global::<f32>(0x00FE8830);
        let s158 = *((this.wrapping_add(0x158)) as *const f32);
        let dot2 = mul_ss(dot, dot);
        if stamp && !(half < s158) {
            *((arg0.wrapping_add(0x12FC)) as *mut u8) = 1;
        }
        // Direction normalisation factor (lahf idiom #2).
        let v110 = *((this.wrapping_add(0x110)) as *const f32);
        let v114 = *((this.wrapping_add(0x114)) as *const f32);
        let v118 = *((this.wrapping_add(0x118)) as *const f32);
        let mut n5 = mul_ss(v110, v110);
        n5 = add_ss(n5, mul_ss(v114, v114));
        n5 = add_ss(n5, mul_ss(v118, v118));
        let k: f32 = if n5 != 0.0 { div_ss(one, n5.sqrt()) } else { 0.0 };
        let w0 = mul_ss(v114, k);
        let w7 = mul_ss(v110, k);
        let w5 = mul_ss(v118, k);
        let mut x2 = *((dx.wrapping_add(0x14)) as *const f32);
        let w7 = mul_ss(w7, *((dx.wrapping_add(0x10)) as *const f32));
        x2 = mul_ss(x2, w0);
        let t = mul_ss(*((dx.wrapping_add(0x18)) as *const f32), w5);
        x2 = add_ss(x2, w7);
        x2 = add_ss(x2, t);
        // lahf idiom #3 selects the response curve.
        x2 = f32::from_bits(x2.to_bits() & absmask);
        if timer != 0.0 {
            x2 = add_ss(mul_ss(x2, half), half);
        } else {
            x2 = add_ss(
                mul_ss(mul_ss(x2, x2), *global::<f32>(0x00FE88C4)),
                *global::<f32>(0x00FE876C),
            );
        }
        // Fill structs (zeroed arrays stand in for the zero-filled frame).
        let mut vec = [0u32; 4];
        vec[0] = *((this.wrapping_add(0xA0)) as *const u32);
        vec[1] = *((this.wrapping_add(0xA4)) as *const u32);
        vec[2] = *((this.wrapping_add(0xA8)) as *const u32);
        vec[3] = *((this.wrapping_add(0xAC)) as *const u32);
        let mut big = [0u32; 15];
        // big[3] mirrors frame word B+0x20, never written: stack_fill 0.
        big[4] = *((this.wrapping_add(0xC0)) as *const u32);
        big[5] = *((this.wrapping_add(0xC4)) as *const u32);
        big[6] = *((this.wrapping_add(0xC8)) as *const u32);
        big[7] = *((this.wrapping_add(0xCC)) as *const u32);
        let f0 = mul_ss(
            mul_ss(
                add_ss(
                    *((this.wrapping_add(0x10)) as *const f32),
                    *global::<f32>(0x00ECA7F8),
                ),
                x2,
            ),
            dot2,
        );
        big[8] = f0.to_bits();
        big[9] = x1.to_bits();
        big[10] = flag;
        big[11] = qans;
        let byte0 = *(this as *const u8) as u32;
        let worddc = *((this.wrapping_add(0xDC)) as *const u16) as u32;
        let d0 = *((this.wrapping_add(0xD0)) as *const u32);
        let dc = *((this.wrapping_add(0xDC)) as *const u32);
        let vecp = vec.as_mut_ptr() as u32;
        let bigp = (big.as_mut_ptr() as u32).wrapping_add(16);
        callee_cdecl!(
            4, u32, arg0, vecp, bigp, f0.to_bits(), x1.to_bits(),
            arg5, edi, worddc, byte0, 0, flag
        );
        // Notify 1.
        let mut aa = [0u32; 5];
        aa[0] = this;
        aa[2] = 0; // frame word B+0x2C, never written: stack_fill 0.
        aa[3] = d0;
        aa[4] = dc;
        callee_thiscall!(5, u32, notify_this, aa.as_mut_ptr() as u32, 0);
        // Tail gates.
        if *((arg0.wrapping_add(0x1304)) as *const u32) == 1 {
            return 0;
        }
        if ((flags >> 12) & 1) == 0 {
            return 0;
        }
        let tidx = *((arg0.wrapping_add(0x2E)) as *const i16) as i32 as u32;
        let tab = relocated(0x01295CD8).wrapping_add(tidx.wrapping_mul(4));
        let entry = *(tab as *const u32);
        let obj = *((entry.wrapping_add(0xCC)) as *const u32);
        if *((obj.wrapping_add(0x204)) as *const u8) == 0 {
            return 0;
        }
        aa[0] = this.wrapping_add(1);
        // Vector adjust from the reloaded (stub-written) fill outputs.
        let s10 = *((this.wrapping_add(0x10)) as *const f32);
        let ea = *((arg0.wrapping_add(0x20)) as *const u32);
        let mut m2 = mul_ss(*((ea.wrapping_add(4)) as *const f32), s10);
        let mut m3 = mul_ss(*((ea.wrapping_add(8)) as *const f32), s10);
        let k110 = *global::<f32>(0x00FE8914);
        let mut v0 = f32::from_bits(vec[0]);
        let mut v1 = f32::from_bits(vec[1]);
        let mut v2 = f32::from_bits(vec[2]);
        if ((flags >> 11) & 1) != 0 {
            let m1 = mul_ss(mul_ss(s10, *((ea) as *const f32)), k110);
            m2 = mul_ss(m2, k110);
            m3 = mul_ss(m3, k110);
            v0 = add_ss(v0, m1);
            v1 = add_ss(v1, m2);
            v2 = add_ss(v2, m3);
        } else {
            let m1 = mul_ss(mul_ss(*((ea) as *const f32), s10), k110);
            m2 = mul_ss(m2, k110);
            m3 = mul_ss(m3, k110);
            v0 = sub_ss(v0, m1);
            v1 = sub_ss(v1, m2);
            v2 = sub_ss(v2, m3);
        }
        vec[0] = v0.to_bits();
        vec[1] = v1.to_bits();
        vec[2] = v2.to_bits();
        // Fill 2 reuses the adjusted vector and the reloaded scalars.
        let f0b = big[8];
        let f1b = big[9];
        callee_cdecl!(
            6, u32, arg0, vecp, bigp, f0b, f1b,
            arg5, edi, worddc, byte0, 1, flag
        );
        callee_thiscall!(5, u32, notify_this, aa.as_mut_ptr() as u32, 0);
        0
}

export!(thiscall, rw_00ace640(this: u32, arg0: u32, arg1: f32, arg2: u32, arg3: f32) -> u32 {
    unsafe { render(this, arg0, arg1, arg2, arg3, true) }
});
