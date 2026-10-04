// original: 0x00d6d2e0 emit_record_composites
//
// Record-list compositor: walks the pointer array at [this+0x9C],
// dispatching each record on its type byte and threshold fields through
// ten arms of one or two halves each. Every half computes four floats
// into a frame quad, resolves two lookup keys, and emits the quad
// through the sender/commit tail calls. The frame buffer array persists
// across records exactly like the original stack frame. Returns the
// array end when at least one record runs, the threshold when the array
// is empty, and the entry EAX (0 under the contract) when null.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

// Callee ids (see contract).
const T_TIME: u32 = 1; // 0x952710 cdecl/0: threshold source
const P_SMP: u32 = 2; // 0x41F320 cdecl/0: int sample
const Q_LKP: u32 = 3; // 0xD6CEE0 thiscall/2: keyed lookup
const R_NRM: u32 = 4; // 0xD6F250 thiscall/0: normalizer
const S_SMP: u32 = 5; // 0x41F300 cdecl/0: int sample
const T_EMT: u32 = 6; // 0x8D4CB0 cdecl/2: emitter
const U_SND: u32 = 7; // 0x8D4750 cdecl/2: buffer sender (frame-pointer args)
const V_CMT: u32 = 8; // 0x8D3C20 cdecl/0: commit
const W_FLG: u32 = 9; // 0x425480 cdecl/0: flag sample (AL channel)

const G_W0: u32 = 0x105C880;
const G_W1: u32 = 0x105C87C;
const G_H0: u32 = 0x105C884;
const G_H1: u32 = 0x105C888;
const G_LO: u32 = 0x11F7028;
const G_HI: u32 = 0x11F702C;

/// Number of u32 words in the modelled frame (slots S+0x00..S+0x15F).
const FR_LEN: usize = 0x58;

#[inline(always)]
unsafe fn rd8(addr: u32) -> u8 {
    unsafe { (addr as *const u8).read() }
}

#[inline(always)]
unsafe fn rd16(addr: u32) -> u16 {
    unsafe { (addr as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn rd32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn rf32(addr: u32) -> f32 {
    unsafe { (addr as *const f32).read_unaligned() }
}

#[inline(always)]
unsafe fn rg32(va: u32) -> u32 {
    unsafe { global::<u32>(va).read() }
}

/// The `movd; cvtdq2pd; shr; addsd [bias]; cvtpd2ps` idiom: exact u32 -> f32
/// through f64 (adds 2^32 when the sign bit is set, then rounds once).
#[inline(always)]
fn u32_via_f64(u: u32) -> f32 {
    let d = (u as i32) as f64;
    let d = d + if u & 0x8000_0000 != 0 { 4294967296.0 } else { 0.0 };
    d as f32
}

/// The original's x87 `fistp qword` with a truncate control word: round
/// toward zero to int64, indefinite (0x8000000000000000) on NaN or range
/// error. Only the low 32 bits are ever used. All exceptions stay masked
/// (the saved control word comes from the worker's default state), so this
/// never faults.
#[inline(always)]
fn fistp_trunc_lo(f: f32) -> u32 {
    if f.is_nan() {
        return 0;
    }
    let t = f.trunc();
    if t >= 9223372036854775808.0 || t < -9223372036854775808.0 {
        0
    } else {
        (t as i64) as u32
    }
}

#[inline(always)]
fn slot_f(fr: &[u32; FR_LEN], byte: usize) -> f32 {
    f32::from_bits(fr[byte / 4])
}

#[inline(always)]
fn set_slot(fr: &mut [u32; FR_LEN], byte: usize, v: u32) {
    fr[byte / 4] = v;
}

#[inline(always)]
fn set_slot_f(fr: &mut [u32; FR_LEN], byte: usize, v: f32) {
    fr[byte / 4] = v.to_bits();
}

/// Shared tail: send buffers A and B, commit. Returns the commit answer
/// (dead on arrival: the loop-continue overwrites EAX right after).
#[inline(always)]
fn tail(fr: &mut [u32; FR_LEN], a_byte: usize, b_byte: usize) -> u32 {
    let a = &mut fr[a_byte / 4] as *mut u32 as u32;
    let b = &mut fr[b_byte / 4] as *mut u32 as u32;
    // Push order is A then B, so B is arg0.
    unsafe { callee_cdecl!(U_SND, u32, b, a) };
    unsafe { callee_cdecl!(V_CMT, u32,) }
}

/// One int-sample half: two lookup stages over the given keys, results into
/// the quad at `qb`, buffers sent from `tail_a`/`qb`.
#[inline(always)]
fn p_half(
    fr: &mut [u32; FR_LEN], this: u32, qb: usize, q1: u32, q2: u32,
    tail_a: usize,
) -> u32 {
    let c_c = unsafe { rf32(this.wrapping_add(0x0C)) };
    let c_t = unsafe { rf32(this.wrapping_add(0x14)) };
    let c_f10 = unsafe { rf32(this.wrapping_add(0x10)) };
    let c_f8 = unsafe { rf32(this.wrapping_add(0x08)) };
    let norm_this = this.wrapping_add(0x1C);
    set_slot(fr, qb, 0x49742400);
    set_slot(fr, qb + 12, 0x49742400);
    set_slot(fr, qb + 8, 0xC9742400);
    set_slot(fr, qb + 4, 0xC9742400);
    // Stage 1.
    let r = unsafe { callee_cdecl!(P_SMP, u32,) };
    set_slot_f(fr, qb + 4, (r as i32) as f32 * c_c);
    let r = unsafe { callee_cdecl!(P_SMP, u32,) };
    let f = (r as i32) as f32 * (c_t + c_c);
    set_slot_f(fr, qb + 12, f);
    // Push order is 0 then the key, so the key is arg0.
    let q = unsafe { callee_thiscall!(Q_LKP, u32, this, q1, 0) };
    set_slot_f(fr, 0x14, u32_via_f64(q));
    let v = unsafe { callee_thiscall!(R_NRM, u32, norm_this) };
    set_slot_f(fr, 0x14, slot_f(fr, 0x14) / u32_via_f64(v));
    let z = unsafe { callee_cdecl!(S_SMP, u32,) };
    let x1 = slot_f(fr, 0x14) * c_f10 + c_f8;
    let g = (z as i32) as f32;
    // Stage 2.
    set_slot_f(fr, qb, g * x1);
    let q = unsafe { callee_thiscall!(Q_LKP, u32, this, q2, 0) };
    set_slot_f(fr, 0x14, u32_via_f64(q));
    let v = unsafe { callee_thiscall!(R_NRM, u32, norm_this) };
    set_slot_f(fr, 0x14, slot_f(fr, 0x14) / u32_via_f64(v));
    let z = unsafe { callee_cdecl!(S_SMP, u32,) };
    let x1 = slot_f(fr, 0x14) * c_f10 + c_f8;
    let g = (z as i32) as f32;
    set_slot_f(fr, qb + 8, g * x1);
    // Push order is 1 then 0, so 0 is arg0.
    unsafe { callee_cdecl!(T_EMT, u32, 0, 1) };
    tail(fr, tail_a, qb)
}

/// One flagged-sample half: the sampler's low byte picks dimension globals,
/// truncated scaled magnitudes divide the lookup answers.
#[inline(always)]
fn v_half(
    fr: &mut [u32; FR_LEN], this: u32, qb: usize, x1: usize, x2: usize,
    q1: u32, q2: u32, tail_a: usize,
) -> u32 {
    let c_c = unsafe { rf32(this.wrapping_add(0x0C)) };
    let c_t = unsafe { rf32(this.wrapping_add(0x14)) };
    let c_f10 = unsafe { rf32(this.wrapping_add(0x10)) };
    let c_f8 = unsafe { rf32(this.wrapping_add(0x08)) };
    let c_f20 = unsafe { rf32(this.wrapping_add(0x20)) };
    set_slot(fr, qb, 0x49742400);
    set_slot(fr, qb + 12, 0x49742400);
    set_slot(fr, qb + 8, 0xC9742400);
    set_slot(fr, qb + 4, 0xC9742400);
    // Stage 1.
    let a = unsafe { callee_cdecl!(W_FLG, u32,) };
    let c = if a & 0xFF != 0 {
        unsafe { rg32(G_W1) }
    } else {
        unsafe { rg32(G_W0) }
    };
    set_slot_f(fr, qb + 4, (c as i32) as f32 * c_c);
    let a = unsafe { callee_cdecl!(W_FLG, u32,) };
    let c = if a & 0xFF != 0 {
        unsafe { rg32(G_W1) }
    } else {
        unsafe { rg32(G_W0) }
    };
    let f1 = (c_t + c_c) * (c as i32) as f32;
    set_slot_f(fr, qb + 12, f1);
    set_slot_f(fr, x1, c_f20 * 1000.0);
    let lo = fistp_trunc_lo(slot_f(fr, x1));
    let q = unsafe { callee_thiscall!(Q_LKP, u32, this, q1, 0) };
    set_slot_f(fr, 0x18, u32_via_f64(q) / u32_via_f64(lo));
    // Stage 2.
    let a = unsafe { callee_cdecl!(W_FLG, u32,) };
    let c = if a & 0xFF != 0 {
        unsafe { rg32(G_H1) }
    } else {
        unsafe { rg32(G_H0) }
    };
    let f2 = (slot_f(fr, 0x18) * c_f10 + c_f8) * (c as i32) as f32;
    set_slot_f(fr, qb, f2);
    set_slot_f(fr, x2, c_f20 * 1000.0);
    let lo = fistp_trunc_lo(slot_f(fr, x2));
    let q = unsafe { callee_thiscall!(Q_LKP, u32, this, q2, 0) };
    set_slot_f(fr, 0x18, u32_via_f64(q) / u32_via_f64(lo));
    let a = unsafe { callee_cdecl!(W_FLG, u32,) };
    let c = if a & 0xFF != 0 {
        unsafe { rg32(G_H1) }
    } else {
        unsafe { rg32(G_H0) }
    };
    let f3 = (slot_f(fr, 0x18) * c_f10 + c_f8) * (c as i32) as f32;
    set_slot_f(fr, qb + 8, f3);
    unsafe { callee_cdecl!(T_EMT, u32, 0, 1) };
    tail(fr, tail_a, qb)
}

// Record-list compositor: walks the pointer array at [this+0x9C],
// dispatching each record on its type byte and threshold fields and emitting
// composite values through the sample/lookup/emitter callees. Returns the
// array end when at least one record runs, the threshold when the array is
// empty, and the entry EAX (fixed to 0 by the contract) when the header is
// null.
export!(thiscall, rw_d6d2e0(this: u32) -> u32 {
    let mut fr = [0u32; FR_LEN];
    let mut eax: u32 = 0;
    set_slot(&mut fr, 0x2C, 0xFF4C4C4C);
    set_slot(&mut fr, 0x28, 0xFF8A8A8A);
    let thr = unsafe { callee_cdecl!(T_TIME, u32,) };
    eax = thr;
    set_slot(&mut fr, 0x24, thr);
    let mut hdr = unsafe { rd32(this.wrapping_add(0x9C)) };
    if hdr == 0 {
        return 0;
    }
    let count = unsafe { rd16(hdr.wrapping_add(4)) } as u32;
    let mut ebx = unsafe { rd32(hdr) };
    if ebx == ebx.wrapping_add(count.wrapping_mul(4)) {
        return eax;
    }
    let mut ebp = ebx.wrapping_add(4);
    loop {
        let edx = unsafe { rd32(ebx) };
        let ty = unsafe { rd8(edx.wrapping_add(1)) };
        eax = (eax & 0xFFFF_FF00) | ty as u32;
        if ty < 0x64 || ty == 0xE1 {
            // R2 dispatch.
            let cnt = unsafe { rd16(hdr.wrapping_add(4)) } as u32;
            let base = unsafe { rd32(hdr) };
            eax = base;
            if ebp == base.wrapping_add(cnt.wrapping_mul(4)) {
                let g = unsafe { rg32(G_HI) }.wrapping_sub(unsafe { rg32(G_LO) });
                eax = g;
                let c = unsafe { rd32(edx.wrapping_add(0x14)) };
                if c >= g {
                    // Skip record.
                } else if c >= thr {
                    let cur = unsafe { rd32(edx.wrapping_add(0x14)) };
                    eax = v_half(&mut fr, this, 0xF0, 0x14C, 0x118, cur, g, 0x2C);
                } else {
                    let cur = unsafe { rd32(edx.wrapping_add(0x14)) };
                    let t = fr[0x24 / 4];
                    v_half(&mut fr, this, 0xE0, 0x12C, 0x134, cur, t, 0x2C);
                    eax = v_half(&mut fr, this, 0x100, 0x110, 0x144, t, g, 0x2C);
                }
            } else {
                let c0 = unsafe { rd32(edx.wrapping_add(0x14)) };
                if c0 >= thr {
                    let cur = c0;
                    let nxt = unsafe { rd32(ebp) };
                    let nkey = unsafe { rd32(nxt.wrapping_add(0x14)) };
                    eax = v_half(&mut fr, this, 0xC0, 0x11C, 0x124, cur, nkey, 0x2C);
                } else {
                    let nxt = unsafe { rd32(ebp) };
                    eax = nxt;
                    let c1 = unsafe { rd32(nxt.wrapping_add(0x14)) };
                    if c1 >= thr {
                        let cur = unsafe { rd32(edx.wrapping_add(0x14)) };
                        let t = fr[0x24 / 4];
                        v_half(&mut fr, this, 0x80, 0x128, 0x148, cur, t, 0x2C);
                        eax = v_half(&mut fr, this, 0xA0, 0x130, 0x114, t, c1, 0x2C);
                    } else {
                        let cur = unsafe { rd32(edx.wrapping_add(0x14)) };
                        eax = v_half(&mut fr, this, 0x60, 0x120, 0x138, cur, c1, 0x2C);
                    }
                }
            }
        } else if ty == 0x64 {
            // Skip record.
        } else {
            // R1 dispatch.
            let cnt = unsafe { rd16(hdr.wrapping_add(4)) } as u32;
            let base = unsafe { rd32(hdr) };
            eax = base;
            if ebp == base.wrapping_add(cnt.wrapping_mul(4)) {
                // Last-record dispatch.
                let g = unsafe { rg32(G_HI) }.wrapping_sub(unsafe { rg32(G_LO) });
                eax = g;
                let c = unsafe { rd32(edx.wrapping_add(0x14)) };
                if c >= g {
                    // Skip record.
                } else if c >= thr {
                    let cur = c;
                    eax = p_half(&mut fr, this, 0x40, cur, g, 0x28);
                } else {
                    let cur = unsafe { rd32(edx.wrapping_add(0x14)) };
                    let t = fr[0x24 / 4];
                    p_half(&mut fr, this, 0x90, cur, t, 0x28);
                    eax = p_half(&mut fr, this, 0xD0, t, g, 0x28);
                }
            } else {
                let c0 = unsafe { rd32(edx.wrapping_add(0x14)) };
                if c0 >= thr {
                    let cur = c0;
                    let nxt = unsafe { rd32(ebp) };
                    let nkey = unsafe { rd32(nxt.wrapping_add(0x14)) };
                    eax = v_half(&mut fr, this, 0x30, 0x13C, 0x140, cur, nkey, 0x28);
                } else {
                    let nxt = unsafe { rd32(ebp) };
                    eax = nxt;
                    let c1 = unsafe { rd32(nxt.wrapping_add(0x14)) };
                    if c1 >= thr {
                        let cur = unsafe { rd32(edx.wrapping_add(0x14)) };
                        let t = fr[0x24 / 4];
                        p_half(&mut fr, this, 0xB0, cur, t, 0x28);
                        eax = p_half(&mut fr, this, 0x70, t, c1, 0x28);
                    } else {
                        let cur = unsafe { rd32(edx.wrapping_add(0x14)) };
                        eax = p_half(&mut fr, this, 0x50, cur, c1, 0x28);
                    }
                }
            }
        }
        // Loop-continue: reload header, advance cursors, recompute EAX as
        // the array end (killing any answer the arm left behind).
        hdr = unsafe { rd32(this.wrapping_add(0x9C)) };
        ebx = ebx.wrapping_add(4);
        let cnt = unsafe { rd16(hdr.wrapping_add(4)) } as u32;
        let base = unsafe { rd32(hdr) };
        ebp = ebp.wrapping_add(4);
        eax = base.wrapping_add(cnt.wrapping_mul(4));
        if ebx != eax {
            continue;
        }
        break;
    }
    eax
});
