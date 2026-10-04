// original: 0x0064C550 rage::ptxSprite::vf25
//! Per-sprite particle update.
//!
//! Reads emitter state (`this`), per-effect parameters and one sprite record,
//! advances the shared random generator, interpolates three keyframe tracks,
//! scatters tint bytes, recycles sprite nodes through a pool, and finishes
//! with two conditional detail passes. Outgoing calls are intercepted by the
//! checker and answered by script; this code performs the same calls with the
//! same arguments in the same order.
//!
//! Verified: 1000/1000 trials bit-exact under checker v3 with the stock
//! worker (see lane report). NaN-propagating float operations carry
//! optimization barriers pinning the original's exact operand order,
//! which decides which NaN survives an SSE add or multiply.

use lf_checker_rt::{
    callee_cdecl, callee_stdcall, callee_thiscall, export, global, relocated,
};

// File VAs of globals this function touches.
const RNG_LO: u32 = 0x11101A0;
const RNG_HI: u32 = 0x11101A4;
const TAG_CTR: u32 = 0x18B749A;
const POOL_A: u32 = 0x1BB6674;
const POOL_B: u32 = 0x1BB6678;
const EPS0: u32 = 0x110DB40;
const EPS1: u32 = 0x110DB44;
const EPS2: u32 = 0x110DB48;
const LUT_BASE: u32 = 0x18B7588;
const C_INV_2POW23: u32 = 0xFE864C;
const C_100: u32 = 0xFE8BB0;
const C_DEG2RAD: u32 = 0xFE8728;
const C_360: u32 = 0xFE8C1C;
const C_9000: u32 = 0xFE8CAC;
const RNG_MULT: u64 = 0x5CDCFAA7;

#[inline(always)]
unsafe fn r32(a: u32) -> u32 {
    (a as *const u32).read()
}
#[inline(always)]
unsafe fn w32(a: u32, v: u32) {
    (a as *mut u32).write(v)
}
#[inline(always)]
unsafe fn rf(a: u32) -> f32 {
    (a as *const f32).read()
}
#[inline(always)]
unsafe fn wf(a: u32, v: f32) {
    (a as *mut f32).write(v)
}
#[inline(always)]
unsafe fn r8(a: u32) -> u8 {
    (a as *const u8).read()
}
#[inline(always)]
unsafe fn w8(a: u32, v: u8) {
    (a as *mut u8).write(v)
}
#[inline(always)]
unsafe fn r16(a: u32) -> u16 {
    (a as *const u16).read()
}
#[inline(always)]
unsafe fn w16(a: u32, v: u16) {
    (a as *mut u16).write(v)
}

/// One step of the shared 64-bit multiply-carry generator. Returns the new
/// low word; both words are stored back.
#[inline(always)]
unsafe fn rng_step() -> u32 {
    let lo = global::<u32>(RNG_LO).read();
    let carry = global::<u32>(RNG_HI).read();
    let full = (lo as u64)
        .wrapping_mul(RNG_MULT)
        .wrapping_add(carry as u64);
    global::<u32>(RNG_LO).write(full as u32);
    global::<u32>(RNG_HI).write((full >> 32) as u32);
    full as u32
}

/// `cdq; idiv divisor` with the dividend in `eax`. Returns (quotient, remainder).
/// All four sites are provably safe: divisors are pinned nonzero by contract
/// or guarded by a branch, quotients fit in i32.
#[inline(always)]
fn idiv32(eax: u32, divisor: u32) -> (u32, u32) {
    let full = (eax as i32) as i64;
    let d = (divisor as i32) as i64;
    ((full / d) as u32, (full % d) as u32)
}

/// `cvttss2si`: truncate toward zero, out-of-range/NaN gives `i32::MIN`.
#[inline(always)]
fn cvtt_ss2si(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}

/// Keyframe segment search: 1-based index of the first row whose key is
/// `>= key`, or `count` when every key is below it. Rows are 48 bytes.
#[inline(always)]
fn kf_search(base: u32, count: u32, key: f32) -> u32 {
    let mut idx = 1u32;
    if (count as i32) > 1 {
        let mut p = base.wrapping_add(0x30);
        loop {
            let k = unsafe { rf(p) };
            if k >= key {
                break;
            }
            idx += 1;
            p = p.wrapping_add(0x30);
            if !((idx as i32) < (count as i32)) {
                break;
            }
        }
    }
    idx
}

/// (table-pointer offset, tag-byte offset, sprite-byte offset) for the tint
/// scatter, in exact order. The twelfth slot bumps the counter only.
const TAG_SLOTS: [(u32, u32, u32); 14] = [
    (0x400, 0x404, 0x77),
    (0x2FC, 0x300, 0x7C),
    (0x22C, 0x230, 0x78),
    (0x294, 0x298, 0x79),
    (0x1C4, 0x1C8, 0x7A),
    (0x330, 0x334, 0x7E),
    (0x468, 0x46C, 0x80),
    (0x364, 0x368, 0x7B),
    (0x398, 0x39C, 0x7D),
    (0x49C, 0x4A0, 0x81),
    (0x670, 0x674, 0x83),
    (0x4D0, 0x4D4, 0x8C),
    (0x5A0, 0x5A4, 0x7F),
    (0x538, 0x53C, 0x8E),
];

/// Shared body of the rewrite.
unsafe fn inner(
    this: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5b: u32,
    a6: u32,
) -> u32 {
    let esi0 = a3.wrapping_add(0x10);
    let edi = a3.wrapping_add(0xA0);
    w32(esi0 + 0x68, 0);
    w32(esi0 + 0x6C, 0);
    w16(esi0 + 0x70, 0);
    // Random magnitude seed.
    let r0 = rng_step();
    let mut x = (((r0 & 0x7FFFFF) as i32) as f32) * global::<f32>(C_INV_2POW23).read();
    x *= global::<f32>(C_100).read();
    wf(esi0 + 0x60, x);
    w32(esi0 + 0x64, r32(a2 + 0x80));
    w8(esi0 + 0x75, 0);
    w32(edi + 0x54, 0);
    // Palette index spread over [t170..t174].
    let t170 = r32(this + 0x170);
    let range = r32(this + 0x174).wrapping_sub(t170);
    let r1 = rng_step();
    let (_, rem1) = idiv32(r1 & 0x7FFFFFFF, range.wrapping_add(1));
    let mut dl = ((rem1 & 0xFF) as u8).wrapping_add((t170 & 0xFF) as u8);
    let al = dl.wrapping_add(1);
    w8(edi + 0x5D, al);
    w8(edi + 0x5C, dl);
    let saved_dl = dl;
    if ((al as u32) as i32) > (r32(this + 0x174) as i32) {
        let f = r32(this + 0x11C);
        let b = if ((f >> 22) & 1) == 0 {
            r8(this + 0x170)
        } else {
            r8(this + 0x174)
        };
        w8(edi + 0x5D, b);
    }
    if ((r32(this + 0x11C) >> 13) & 1) != 0 {
        w8(edi + 0x5D, dl);
        let hi = r32(this + 0x178);
        let lo = r32(this + 0x170);
        if lo == hi || (saved_dl as i32) < (lo as i32) || (saved_dl as i32) > (hi as i32) {
            let r2 = rng_step();
            let d3 = hi.wrapping_sub(lo).wrapping_add(1);
            let (_, rem3) = idiv32(r2 & 0x7FFFFFFF, d3);
            dl = ((rem3 & 0xFF) as u8).wrapping_add((lo & 0xFF) as u8);
        } else {
            let r2 = rng_step();
            let d2 = hi.wrapping_sub(lo);
            let (_, rem2) = idiv32(r2 & 0x7FFFFFFF, d2);
            let e = (saved_dl as u32).wrapping_add(1);
            let d2b = d2.wrapping_add(1);
            let e2 = e.wrapping_add(rem2.wrapping_sub(lo));
            let (_, rem2b) = idiv32(e2, d2b);
            dl = ((rem2b & 0xFF) as u8).wrapping_add((lo & 0xFF) as u8);
        }
        w8(edi + 0x5D, dl);
    }
    // Stride setup call.
    let _: u32 = callee_thiscall!(1, u32, this.wrapping_add(0x188));
    // Tint scatter: tag each table with a sequence byte, copy one byte.
    for (i, &(toff, goff, soff)) in TAG_SLOTS.iter().enumerate() {
        let c = global::<u8>(TAG_CTR).read();
        global::<u8>(TAG_CTR).write(c.wrapping_add(1));
        w8(this + goff, c);
        let b = r8(r32(this + toff));
        w8(esi0 + soff, b);
        if i == 10 {
            // Counter-only slot between the 0x670 and 0x4D0 slots.
            let c2 = global::<u8>(TAG_CTR).read();
            global::<u8>(TAG_CTR).write(c2.wrapping_add(1));
            w8(esi0 + 0x82, c2);
        }
        if soff == 0x8C {
            w8(esi0 + 0x8D, b);
        }
    }
    // Activation roll: (rng % 101) <= density byte.
    w8(esi0 + 0x72, 0);
    let r3 = rng_step();
    let (_, rem4) = idiv32(r3 & 0x7FFFFFFF, 101);
    if (rem4 as i32) <= (r8(this + 0x132) as i32) {
        w8(esi0 + 0x72, 1);
    }
    // Copy the five control words.
    w32(esi0 + 0x4C, r32(a1 + 0x1A4));
    w32(esi0 + 0x50, r32(a1 + 0x1A8));
    w32(esi0 + 0x54, r32(a1 + 0x1AC));
    w32(esi0 + 0x58, r32(a1 + 0x1B0));
    w32(esi0 + 0x5C, r32(a1 + 0x1B4));
    let x3 = rf(a2 + 0x80);
    // --- keyframe track 1 (life) ---
    let c1 = r16(this + 0x528) as u32;
    let b1 = r32(this + 0x524);
    let row1 = b1
        .wrapping_sub(0x30)
        .wrapping_add(kf_search(b1, c1, x3).wrapping_mul(48));
    // Keyframe lerp: the original issues scalar (k*t)+b per lane. Pin
    // every operand with barriers (same NaN-order note as the sqrt-arg
    // sum): without them LLVM packs the lanes and may fold the factor,
    // which changes which NaN survives.
    let t1 = core::hint::black_box(x3 - rf(row1));
    let p5 = core::hint::black_box(rf(row1 + 0x20)) * core::hint::black_box(t1);
    let p4 = core::hint::black_box(rf(row1 + 0x24)) * core::hint::black_box(t1);
    let f5 = core::hint::black_box(p5) + core::hint::black_box(rf(row1 + 0x10));
    let f1a = rf(row1 + 0x28);
    let f0a = rf(row1 + 0x2C);
    let _f1a = f1a * t1 + rf(row1 + 0x18);
    let f4 = core::hint::black_box(p4) + core::hint::black_box(rf(row1 + 0x14));
    let _f0a = f0a * t1 + rf(row1 + 0x1C);
    let (mut s20, mut s3c) = (f5, f4);
    let mut buf1 = [f5.to_bits(), f4.to_bits()];
    if a4 != 0 {
        let _: u32 = callee_thiscall!(
            2, u32, a4, 3, buf1.as_mut_ptr() as u32,
            r32(this + 0x534), x3.to_bits(), esi0 + 0x4C
        );
        s3c = f32::from_bits(buf1[1]);
        s20 = f32::from_bits(buf1[0]);
    }
    // --- keyframe track 2 (size) ---
    let c2 = r16(this + 0x55C) as u32;
    let b2 = r32(this + 0x558);
    let row2 = b2
        .wrapping_sub(0x30)
        .wrapping_add(kf_search(b2, c2, x3).wrapping_mul(48));
    let t2 = core::hint::black_box(x3 - rf(row2));
    let (k5, k4);
    {
        // Lane 0 multiplies (t*k), lane 1 multiplies (k*t); both add
        // (product+base). Barriers pin the original's operand order.
        let q5 = core::hint::black_box(t2) * core::hint::black_box(rf(row2 + 0x20));
        let l5 = core::hint::black_box(q5) + core::hint::black_box(rf(row2 + 0x10));
        let q4 = core::hint::black_box(rf(row2 + 0x24)) * core::hint::black_box(t2);
        let l4 = core::hint::black_box(q4) + core::hint::black_box(rf(row2 + 0x14));
        let _l1 = rf(row2 + 0x28) * t2 + rf(row2 + 0x18);
        let _l0 = rf(row2 + 0x2C) * t2 + rf(row2 + 0x1C);
        let mut buf2 = [l5.to_bits(), l4.to_bits()];
        if a4 != 0 {
            let _: u32 = callee_thiscall!(
                2, u32, a4, 3, buf2.as_mut_ptr() as u32,
                r32(this + 0x568), x3.to_bits(), esi0 + 0x4C
            );
            k4 = f32::from_bits(buf2[1]);
            k5 = f32::from_bits(buf2[0]);
        } else {
            k4 = l4;
            k5 = l5;
        }
    }
    // Random spin and tilt from the two track outputs.
    let d1 = s3c - s20;
    let r4 = rng_step();
    let mut g0 = ((r4 & 0x7FFFFF) as i32) as f32;
    g0 *= global::<f32>(C_INV_2POW23).read();
    g0 *= d1;
    g0 += s20;
    g0 *= global::<f32>(C_DEG2RAD).read();
    wf(esi0 + 0x84, g0);
    let r5 = rng_step();
    // The track-2 delta (xmm4 -= xmm5) feeds the tilt, not the raw factor.
    let k4d = k4 - k5;
    let mut h0 = ((r5 & 0x7FFFFF) as i32) as f32;
    h0 *= global::<f32>(C_INV_2POW23).read();
    h0 *= k4d;
    h0 += k5;
    h0 *= global::<f32>(C_360).read();
    h0 *= global::<f32>(C_DEG2RAD).read();
    wf(esi0 + 0x88, h0);
    w32(edi + 0x58, 0);
    w32(esi0 + 0x68, 0);
    w32(esi0 + 0x6C, 0);
    let v5 = rf(a6) - rf(a2 + 0x40);
    let v6 = rf(a6 + 4) - rf(a2 + 0x44);
    let v7 = rf(a6 + 8) - rf(a2 + 0x48);
    // --- keyframe track 3 (drag), keyed at zero with a negated factor ---
    let c3 = r16(this + 0x2EC) as u32;
    let b3 = r32(this + 0x2E8);
    let row3 = b3
        .wrapping_sub(0x30)
        .wrapping_add(kf_search(b3, c3, 0.0).wrapping_mul(48));
    // Lane 0 multiplies (t*k), lane 1 multiplies (k*t); both add
    // (product+base). The barrier after the negation stops LLVM folding
    // it into the multiply (base-minus-product changes NaN signs).
    let t3 = core::hint::black_box(-rf(row3));
    let r4 = core::hint::black_box(t3) * core::hint::black_box(rf(row3 + 0x20));
    let l4c = core::hint::black_box(r4) + core::hint::black_box(rf(row3 + 0x10));
    let r2 = core::hint::black_box(rf(row3 + 0x24)) * core::hint::black_box(t3);
    let _l2c = core::hint::black_box(r2) + core::hint::black_box(rf(row3 + 0x14));
    let _l1c = rf(row3 + 0x28) * t3 + rf(row3 + 0x18);
    let _l0c = rf(row3 + 0x2C) * t3 + rf(row3 + 0x1C);
    // Pre-call buffer holds [l4c, l2c]; only the first word is live after.
    let mut buf3 = [l4c.to_bits(), _l2c.to_bits()];
    let k4b: f32;
    if a4 != 0 {
        let _: u32 = callee_thiscall!(
            2, u32, a4, 3, buf3.as_mut_ptr() as u32,
            r32(this + 0x2F8), 0, esi0 + 0x4C
        );
        k4b = f32::from_bits(buf3[0]);
    } else {
        k4b = l4c;
    }
    // Scale the offset by the caller factor and accumulate into the record.
    let a5f = f32::from_bits(a5b);
    let kk = k4b * a5f;
    let p5 = v5 * kk + rf(edi + 0x10);
    let p6 = v6 * kk + rf(edi + 0x14);
    let p7 = v7 * kk + rf(edi + 0x18);
    wf(edi + 0x10, p5);
    wf(edi + 0x14, p6);
    wf(edi + 0x18, p7);
    wf(edi + 0x40, p5);
    w32(edi + 0x44, r32(edi + 0x14));
    w32(edi + 0x48, r32(edi + 0x18));
    w32(edi + 0x20, r32(edi + 0x10));
    w32(edi + 0x24, r32(edi + 0x14));
    w32(edi + 0x28, r32(edi + 0x18));
    w32(esi0 + 0x18, 0);
    w32(esi0 + 0x14, 0);
    w32(esi0 + 0x10, 0);
    // Resolve the effect definition.
    let rp: u32 = callee_cdecl!(
        3, u32, r32(a2 + 0xE0), 0, relocated(0x1150B94), relocated(0x1150B24), 0
    );
    let rq = r32(rp + 0x1C);
    if r32(rq + 8) == 3 {
        // Effect-kind pass with the 9000-scaled bounds test.
        let _xmm3_arg = rf(edi + 0x50);
        let mut out1 = [0u32; 4];
        let _: u32 = callee_stdcall!(
            4, u32, out1.as_mut_ptr() as u32, this.wrapping_add(0x470), a4, esi0 + 0x4C
        );
        let o0 = f32::from_bits(out1[0]);
        let qa = rq.wrapping_add(0xB0);
        let k9000 = global::<f32>(C_9000).read();
        let mut x1 = rf(qa + 0x10);
        let mut x4 = rf(qa + 0x30);
        let mut x2 = rf(qa + 0x14);
        let mut x3 = rf(qa + 0x18);
        let mut x6 = rf(qa + 0x34);
        x1 *= k9000;
        let mut x7 = rf(qa + 0x38);
        x2 *= k9000;
        let mut x5 = x4 - x1;
        x3 *= k9000;
        let mut x0 = rf(qa + 0x34);
        x4 += x1;
        x1 = rf(qa + 0x38);
        x0 += x2;
        x6 -= x2;
        x7 -= x3;
        x4 -= x5;
        x1 += x3;
        x3 = rf(edi + 0x10);
        x0 -= x6;
        x5 -= x3;
        let s10 = x4;
        x4 = rf(edi + 0x14);
        x6 -= x4;
        x1 -= x7;
        x7 -= rf(edi + 0x18);
        let (s3c2, _s5c, s40, s20b) = (x0, x3, x6, x1);
        x1 = s10;
        x2 = x6 * x0;
        x0 = x5 * x1;
        x2 += x0;
        x0 = x7 * s20b;
        x2 += x0;
        let e0 = global::<f32>(EPS0).read();
        let e44 = global::<f32>(EPS1).read();
        let e48 = global::<f32>(EPS2).read();
        x2 = -x2;
        if !(e0 >= x2) || !(e44 >= x2) || !(e48 >= x2) {
            x0 = s3c2;
            let mut x3b = x0 * x0;
            x0 = x1 * x1;
            x1 = s20b;
            x3b += x0;
            x0 = x1 * x1;
            x3b += x0;
            x1 = x3b - x2;
            x0 = x3b - x2;
            x3b -= x2;
            x6 = s40;
            x4 = rf(edi + 0x14);
            if !(e0 >= x1) || !(e44 >= x0) || !(e48 >= x3b) {
                x0 += x2;
                x1 += x2;
                x3b += x2;
                let s40b = x0;
                x0 = x2 / x1;
                x1 = x2 / s40b;
                x2 /= x3b;
                x0 *= s10;
                x1 *= s3c2;
                x2 *= s20b;
                x5 += x0;
                x6 += x1;
                x7 += x2;
            } else {
                x5 += s10;
                x6 += s3c2;
                x7 += s20b;
            }
        }
        x2 = o0;
        x0 = rf(edi + 0x18);
        x5 += x3;
        x6 += x4;
        x7 += x0;
        x3 -= x5;
        x4 -= x6;
        x0 -= x7;
        let x1b: f32;
        if x2 != 0.0 {
            // Sum of squares feeding the sqrt call. The original issues
            // scalar mulss/addss in exactly this order, and which NaN
            // survives an SSE add depends on operand order, so LLVM must
            // not vectorize or reassociate these. Each black_box is an
            // optimization barrier only: it emits no code and changes no
            // value, it just pins the operand order of the add below it.
            // (Verified against the DLL disassembly: scalar mulss on the
            // three diffs, then addss in (x4+x3)+x0 order.)
            x4 *= x4;
            x3 *= x3;
            x0 *= x0;
            x4 = core::hint::black_box(x4) + core::hint::black_box(x3);
            x4 = core::hint::black_box(x4) + core::hint::black_box(x0);
            let sq: f32 = callee_cdecl!(5, f32, x4.to_bits());
            x0 = sq;
            x0 /= o0;
            x0 = x0.abs();
            x1b = x0.sqrt();
        } else {
            x1b = 0.0;
        }
        wf(edi + 0x30, rf(edi + 0x30) * x1b);
        wf(edi + 0x34, x1b * rf(edi + 0x34));
        wf(edi + 0x38, x1b * rf(edi + 0x38));
    }
    // Drain live sprite nodes back into the pool free list.
    let mut node = r32(esi0 + 0x20);
    let esil = esi0.wrapping_add(0x20);
    if node != 0 {
        let pool = global::<u32>(POOL_A).read();
        loop {
            let next = r32(node);
            if esil != 0 {
                w32(esil + 0x1C, r32(esil + 0x1C).wrapping_sub(1));
                let prev = r32(node + 4);
                let nx = r32(node);
                if r32(esil) == node {
                    w32(esil, nx);
                }
                if r32(node) != 0 {
                    w32(nx + 4, r32(node + 4));
                }
                if r32(node + 4) != 0 {
                    w32(prev, r32(node));
                }
                w32(node + 4, 0);
            }
            w32(node, r32(pool + 0x100));
            w32(node + 4, 0);
            w32(pool + 0x11C, r32(pool + 0x11C).wrapping_add(1));
            w32(pool + 0x100, node);
            w32(pool + 0x16C, r32(pool + 0x16C).wrapping_sub(1));
            node = next;
            if node == 0 {
                break;
            }
        }
    }
    w8(a3 + 0x85, 0);
    if (r32(this + 0x11C) as i32) < 0 {
        // Detail pass over the curve table.
        let mut out2 = [0u32; 4];
        let _xmm3_b = rf(a2 + 0x80);
        let _: u32 = callee_stdcall!(
            4, u32, out2.as_mut_ptr() as u32,
            this.wrapping_add(0x644), a4, a3.wrapping_add(0x5C)
        );
        let tx = global::<f32>(LUT_BASE + (r8(a3 + 0x93) as u32) * 4).read();
        let x2 = if r8(this + 0x675) != 0 { 1.0 - tx } else { tx };
        let o2 = f32::from_bits(out2[2]);
        let o3 = f32::from_bits(out2[3]);
        let mut x1 = o3 - o2;
        x1 *= x2;
        x1 += o2;
        let mut bound = cvtt_ss2si(x1);
        if bound < 3 {
            bound = 3;
        }
        if bound > 0 {
            // Bounded allocator loop: pop one free node per pass, at most
            // `bound` passes, stopping early when the free list empties.
            // (The taken-branch skips the bound reload, so the counter
            // counts passes, not bound+1. The bound can never be i32::MAX
            // from a float conversion, so the empty-list exit always fires.)
            let pool = global::<u32>(POOL_A).read();
            let mut ctr = 0u32;
            loop {
                let free = r32(pool + 0x100);
                if free == 0 {
                    ctr = (bound as u32).wrapping_add(1);
                } else {
                    let nx = r32(free);
                    w32(pool + 0x100, nx);
                    w32(pool + 0x11C, r32(pool + 0x11C).wrapping_sub(1));
                    w32(pool + 0x16C, r32(pool + 0x16C).wrapping_add(1));
                    let old = r32(esil);
                    if old != 0 {
                        w32(old + 4, free);
                    }
                    w32(free, old);
                    w32(free + 4, 0);
                    w32(esil + 0x1C, r32(esil + 0x1C).wrapping_add(1));
                    w32(esil, free);
                    ctr = ctr.wrapping_add(1);
                }
                if !((ctr as i32) < bound) {
                    break;
                }
            }
        }
        let mut wc = r32(esil);
        while wc != 0 {
            w32(wc + 0x10, r32(edi + 0x10));
            w32(wc + 0x14, r32(edi + 0x14));
            w32(wc + 0x18, r32(edi + 0x18));
            w8(a3 + 0x85, r8(a3 + 0x85).wrapping_add(1));
            w32(a3 + 0x50, wc);
            wc = r32(wc);
        }
    }
    // Stamp the record copy and run the conditional detail passes.
    let pool2 = global::<u32>(POOL_B).read();
    let idx = r8(pool2 + 1) as u32;
    let dest = a3
        .wrapping_add(0x100)
        .wrapping_add(idx.wrapping_mul(3).wrapping_mul(32));
    let _: u32 = callee_thiscall!(6, u32, dest, edi);
    let stride = idx.wrapping_mul(4);
    let iv = r32(a2 + stride.wrapping_add(4));
    let cnt_a = a2 + stride.wrapping_add(0x14);
    w32(cnt_a, r32(cnt_a).wrapping_add(1));
    w32(a3 + stride.wrapping_add(8), iv);
    w32(a2 + stride.wrapping_add(4), a3);
    let g = rf(edi + 0x50);
    if g > rf(this + 0x88) {
        let _: u32 = callee_thiscall!(7, u32, this, a3, 0, a1);
    }
    let g2 = rf(edi + 0x50);
    if g2 > rf(this + 0xF8) {
        let _: u32 = callee_thiscall!(7, u32, this, a3, 1, a1);
    }
    a3.wrapping_add(0x10)
}

export!(thiscall, rw_fn(this: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5b: u32, a6: u32) -> u32 {
    // SAFETY: all addresses come from the checker's fabricated objects.
    unsafe { inner(this, a1, a2, a3, a4, a5b, a6) }
});

