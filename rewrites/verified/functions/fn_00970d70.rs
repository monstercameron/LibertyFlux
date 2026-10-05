// original: 0x00970d70 audio_spatial_max_update (proposed)

/// Spread a listener-position delta over spatial audio cells, then rebuild the
/// per-cell maximum tables (thiscall, no stack arguments, returns 1).
///
/// `this` is an audio manager object. Words at `+0x580..0x598` hold the new
/// (x, y, z) listener position followed by the previous one; the entry block
/// forms the delta (dx, dy, dz) = new - old, latches the new position over
/// the old, and copies the tag int at `+0x58c` to `+0x59c`.
///
/// Phase A adds each delta component to one of two 27-cell float lists,
/// picked by a `comiss` against +0.0: the "positive" list when the delta is
/// above zero, the other list when it is zero, negative or NaN (an unordered
/// compare takes the `jbe` side). Each cell is an independent float add; the
/// operand order varies per cell (about a third are delta + cell, the rest
/// cell + delta) and is preserved exactly, since it selects the NaN payload
/// when both operands are NaN. The z lists run as two unrolled iterations of
/// 8 cells plus one cell accumulated four times.
///
/// Phases B and C are max-reduction trees over coefficient tables. All
/// selects are `comiss` + `ja`/`jbe`, i.e. max(a, b) with NaN losing to the
/// other operand (`if a > b { a } else { b }`); the integer selects are an
/// unsigned `> 3` on (counter - 2) and a signed `<= 3` on the counter.
/// Indices are small loop derivations: trunc((k + 1.0) * 0.5) mod 4 and
/// (k + 1) mod 8 in B, plus five idiv remainders mod 24 (constant divisor,
/// positive dividends, so plain `%`) in C. The two sign inputs are the
/// read-only constants +1.0 and -1.0; phase B multiplies every stored value
/// by its outer sign twice (inputs pre-scaled, outputs post-scaled).
///
/// Reads only `this` (offsets `0x580..0x1c5c`) and three read-only float
/// constants; writes the latched position, the tag, the phase-A cells and
/// the rebuilt tables at `+0x5b0..0x9b8`. No calls, no globals, no vector
/// or x87 entry state. Returns the last mod-24 remainder, always 1.
use lf_checker_rt as rt;

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
fn add_mem_first(m: f32, d: f32) -> f32 {
    core::hint::black_box(m) + core::hint::black_box(d)
}

#[inline(always)]
fn add_d_first(d: f32, m: f32) -> f32 {
    core::hint::black_box(d) + core::hint::black_box(m)
}

#[inline(always)]
fn sub_new_old(n: f32, o: f32) -> f32 {
    core::hint::black_box(n) - core::hint::black_box(o)
}

#[inline(always)]
fn mul_mem_sign(m: f32, s: f32) -> f32 {
    core::hint::black_box(m) * core::hint::black_box(s)
}

/// `comiss a, b` + `ja`/`jbe` max-select: a wins only on strict above, so NaN
/// always loses to the other operand.
#[inline(always)]
fn pick_max(a: f32, b: f32) -> f32 {
    if core::hint::black_box(a) > core::hint::black_box(b) {
        a
    } else {
        b
    }
}

#[inline(always)]
unsafe fn g1() -> f32 {
    unsafe { f32::from_bits(rd32(rt::relocated(0x00fe88e8))) }
}

#[inline(always)]
unsafe fn gneg1() -> f32 {
    unsafe { f32::from_bits(rd32(rt::relocated(0x00fe8d94))) }
}

#[inline(always)]
unsafe fn ghalf() -> f32 {
    unsafe { f32::from_bits(rd32(rt::relocated(0x00fe8830))) }
}

/// (offset, delta_first): phase-A cell lists in execution order.
const DX_POS: [(u32, bool); 27] = [
    (0x18a4, false), (0x18a8, false), (0x18ac, false), (0x18b0, false),
    (0x18b4, false), (0x18b8, false), (0x18bc, false), (0x18c0, false),
    (0x18c4, false), (0x18c8, false), (0x18cc, false), (0x1a64, false),
    (0x1a68, false), (0x1a6c, true), (0x1be4, false), (0x1a84, false),
    (0x1a88, true), (0x1a8c, false), (0x1bf4, false), (0x1aa4, false),
    (0x1aa8, true), (0x1aac, false), (0x1c04, false), (0x1ac4, false),
    (0x1ac8, true), (0x1acc, false), (0x1c14, false),
];
const DX_NEG: [(u32, bool); 27] = [
    (0x18d4, false), (0x18d8, true), (0x18dc, false), (0x18e0, true),
    (0x18e4, false), (0x18e8, true), (0x18ec, false), (0x18f0, true),
    (0x18f4, false), (0x18f8, false), (0x18fc, false), (0x1a74, false),
    (0x1a78, false), (0x1a7c, true), (0x1bec, false), (0x1a94, false),
    (0x1a98, true), (0x1a9c, false), (0x1bfc, false), (0x1ab4, false),
    (0x1ab8, true), (0x1abc, false), (0x1c0c, false), (0x1ad4, false),
    (0x1ad8, true), (0x1adc, false), (0x1c1c, false),
];
const DY_POS: [(u32, bool); 27] = [
    (0x1840, true), (0x1844, false), (0x1848, true), (0x184c, false),
    (0x1850, true), (0x1854, false), (0x188c, true), (0x1890, false),
    (0x1894, true), (0x1898, false), (0x189c, true), (0x19e0, true),
    (0x19e4, true), (0x19fc, false), (0x1ba0, true), (0x1a00, true),
    (0x1a04, false), (0x1a1c, false), (0x1bb0, true), (0x1a20, true),
    (0x1a24, false), (0x1a3c, false), (0x1bc0, true), (0x1a40, true),
    (0x1a44, false), (0x1a5c, false), (0x1bd0, true),
];
const DY_NEG: [(u32, bool); 27] = [
    (0x185c, false), (0x1860, false), (0x1864, false), (0x1868, false),
    (0x186c, false), (0x1870, false), (0x1874, false), (0x1878, false),
    (0x187c, false), (0x1880, false), (0x1884, false), (0x19ec, true),
    (0x19f0, true), (0x19f4, false), (0x1ba8, true), (0x1a0c, true),
    (0x1a10, false), (0x1a14, true), (0x1bb8, true), (0x1a2c, true),
    (0x1a30, false), (0x1a34, true), (0x1bc8, true), (0x1a4c, true),
    (0x1a50, false), (0x1a54, true), (0x1bd8, true),
];

#[inline(always)]
unsafe fn run_cells(this: u32, d: f32, cells: &[(u32, bool); 27]) {
    unsafe {
        for &(off, d_first) in cells.iter() {
            let m = rdf(this + off);
            wrf(this + off, if d_first { add_d_first(d, m) } else { add_mem_first(m, d) });
        }
    }
}

#[inline(always)]
unsafe fn cell(this: u32, base: u32, idx: u32) -> u32 {
    this + idx * 4 + base
}

unsafe fn body(this: u32) -> u32 {
    unsafe {
        let one = g1();
        let neg1 = gneg1();
        let half = ghalf();

        // Phase A entry: delta, latch, tag copy.
        let nx = rdf(this + 0x580);
        let ny = rdf(this + 0x584);
        let nz = rdf(this + 0x588);
        let ox = rdf(this + 0x590);
        let oy = rdf(this + 0x594);
        let oz = rdf(this + 0x598);
        let dx = sub_new_old(nx, ox);
        let dy = sub_new_old(ny, oy);
        let dz = sub_new_old(nz, oz);
        wrf(this + 0x590, nx);
        wrf(this + 0x594, ny);
        wrf(this + 0x598, nz);
        wr32(this + 0x59c, rd32(this + 0x58c));

        let take_pos = dx > 0.0;
        if take_pos {
            run_cells(this, dx, &DX_POS);
        } else {
            run_cells(this, dx, &DX_NEG);
        }
        if dy > 0.0 {
            run_cells(this, dy, &DY_POS);
        } else {
            run_cells(this, dy, &DY_NEG);
        }
        // Z lists: two iterations of 8 cells + one cell accumulated 4 times.
        if dz > 0.0 {
            let mut b = 0x1ae4u32;
            let mut c = 0x1c20u32;
            for _ in 0..2u32 {
                wrf(this + b - 4, add_mem_first(rdf(this + b - 4), dz));
                wrf(this + b, add_mem_first(rdf(this + b), dz));
                wrf(this + b + 4, add_mem_first(rdf(this + b + 4), dz));
                wrf(this + b + 8, add_d_first(dz, rdf(this + b + 8)));
                wrf(this + b + 12, add_mem_first(rdf(this + b + 12), dz));
                wrf(this + b + 16, add_mem_first(rdf(this + b + 16), dz));
                wrf(this + b + 20, add_mem_first(rdf(this + b + 20), dz));
                wrf(this + b + 24, add_d_first(dz, rdf(this + b + 24)));
                let mut v = add_mem_first(rdf(this + c), dz);
                v = add_mem_first(v, dz);
                v = add_mem_first(v, dz);
                v = add_mem_first(v, dz);
                wrf(this + c, v);
                b += 0x20;
                c += 0x10;
            }
        } else {
            let mut b = 0x1b24u32;
            let mut c = 0x1c40u32;
            for _ in 0..2u32 {
                wrf(this + b - 4, add_d_first(dz, rdf(this + b - 4)));
                wrf(this + b, add_mem_first(rdf(this + b), dz));
                wrf(this + b + 4, add_mem_first(rdf(this + b + 4), dz));
                wrf(this + b + 8, add_mem_first(rdf(this + b + 8), dz));
                wrf(this + b + 12, add_mem_first(rdf(this + b + 12), dz));
                wrf(this + b + 16, add_d_first(dz, rdf(this + b + 16)));
                wrf(this + b + 20, add_mem_first(rdf(this + b + 20), dz));
                wrf(this + b + 24, add_mem_first(rdf(this + b + 24), dz));
                let mut v = add_d_first(dz, rdf(this + c));
                v = add_mem_first(v, dz);
                v = add_mem_first(v, dz);
                v = add_mem_first(v, dz);
                wrf(this + c, v);
                b += 0x20;
                c += 0x10;
            }
        }

        // Phase B: two outer rows x eight inner columns of max-trees.
        for o in 0..2u32 {
            let (a_slot, b_slot) = if o == 1 { (3u32, 2u32) } else { (0u32, 1u32) };
            let (ia, ic) = if o == 1 { (3u32, 2u32) } else { (0u32, 1u32) };
            let s6 = if o == 1 { neg1 } else { one };
            let (mut p14, mut p18, mut p1c, mut p20, mut p24, mut p30) = (
                this + (ia * 8 + 0x6b8) * 4,
                this + (ic * 8 + 0x6b8) * 4,
                this + (ia * 8 + 0x678) * 4,
                this + (ic * 8 + 0x678) * 4,
                this + (ia * 8 + 0x698) * 4,
                this + (ic * 8 + 0x698) * 4,
            );
            let (i2c, i58, i28, i54, i50, i4c) = (
                ia * 8 + 0x6b8,
                ic * 8 + 0x6b8,
                ia * 8 + 0x678,
                ic * 8 + 0x678,
                ia * 8 + 0x698,
                ic * 8 + 0x698,
            );
            let mut w = this + 0x7b8 + o * 0x180;
            for b in 0..8u32 {
                let bf = b as f32;
                let q = (mul_mem_sign(add_mem_first(bf, one), half) as i32 % 4) as u32;
                let r = ((b as i32 + 1) % 8) as u32;
                let s2 = if (b as i32).wrapping_sub(2) as u32 > 3 { one } else { neg1 };
                let s4 = if (b as i32) <= 3 { one } else { neg1 };
                // Tree 0 (sign s6, base 0x1c20).
                let mut x0 = pick_max(
                    mul_mem_sign(rdf(cell(this, 0x1c20, q + 4 * a_slot)), s6),
                    mul_mem_sign(rdf(cell(this, 0x1c20, q + 4 * b_slot)), s6),
                );
                let mut x1 = pick_max(
                    mul_mem_sign(rdf(p14), s6),
                    mul_mem_sign(rdf(this + (i2c + r) * 4), s6),
                );
                x0 = pick_max(x0, x1);
                x1 = pick_max(x0, mul_mem_sign(rdf(p18), s6));
                x1 = pick_max(x1, mul_mem_sign(rdf(this + (i58 + r) * 4), s6));
                x0 = mul_mem_sign(x0, s6);
                x1 = mul_mem_sign(x1, s6);
                wrf(w - 0x200, x0);
                wrf(w, x1);
                // Tree 1 (sign s2, base 0x1ba0).
                let mut y0 = pick_max(
                    mul_mem_sign(rdf(cell(this, 0x1ba0, q + 4 * a_slot)), s2),
                    mul_mem_sign(rdf(cell(this, 0x1ba0, q + 4 * b_slot)), s2),
                );
                let mut y1 = pick_max(
                    mul_mem_sign(rdf(p1c), s2),
                    mul_mem_sign(rdf(this + (i28 + r) * 4), s2),
                );
                y0 = pick_max(y0, y1);
                y1 = pick_max(y0, mul_mem_sign(rdf(p20), s2));
                y1 = pick_max(y1, mul_mem_sign(rdf(this + (i54 + r) * 4), s2));
                y0 = mul_mem_sign(y0, s2);
                y1 = mul_mem_sign(y1, s2);
                wrf(w - 0x204, y0);
                wrf(w - 4, y1);
                // Tree 2 (sign s4, base 0x1be0).
                let mut z0 = pick_max(
                    mul_mem_sign(rdf(cell(this, 0x1be0, q + 4 * a_slot)), s4),
                    mul_mem_sign(rdf(cell(this, 0x1be0, q + 4 * b_slot)), s4),
                );
                let mut z1 = pick_max(
                    mul_mem_sign(rdf(p24), s4),
                    mul_mem_sign(rdf(this + (i50 + r) * 4), s4),
                );
                z0 = pick_max(z0, z1);
                z1 = pick_max(z0, mul_mem_sign(rdf(p30), s4));
                z1 = pick_max(z1, mul_mem_sign(rdf(this + (i4c + r) * 4), s4));
                z1 = mul_mem_sign(z1, s4);
                z0 = mul_mem_sign(z0, s4);
                wrf(w - 8, z1);
                wrf(w - 0x208, z0);
                p14 += 4;
                p18 += 4;
                p1c += 4;
                p20 += 4;
                p24 += 4;
                p30 += 4;
                w += 0x10;
            }
        }

        // Phase C: two outer rows x eight inner columns, mod-24 index trees.
        let mut ret = 0u32;
        for f in 1..3u32 {
            let (ia, ic, s) = if f == 2 { (3u32, 2u32, 2u32) } else { (0u32, 1u32, 1u32) };
            let s7 = if f == 2 { neg1 } else { one };
            let (i30, i5c, i38, i3c, i68, i60) = (
                ia * 8 + 0x6b8,
                ic * 8 + 0x6b8,
                ia * 8 + 0x678,
                ia * 8 + 0x698,
                ic * 8 + 0x698,
                ic * 8 + 0x678,
            );
            let (mut p20, mut p1c, mut p18, mut p14, mut p28, mut p2c) = (
                this + (ia * 8 + 0x6b8) * 4,
                this + (ic * 8 + 0x6b8) * 4,
                this + i38 * 4,
                this + (ic * 8 + 0x678) * 4,
                this + i3c * 4,
                this + i68 * 4,
            );
            let mut edi_r = this + 0x18a0;
            let mut wc = this + 0x7b8 + f * 0x80;
            for k in 0..8u32 {
                let edx = 2 + 3 * k as i32;
                let kf = k as f32;
                let q = (mul_mem_sign(add_mem_first(kf, one), half) as i32 % 4) as u32;
                let r = ((k as i32 + 1) % 8) as u32;
                let d0 = ((edx - 1) % 24) as u32;
                let d1 = (edx % 24) as u32;
                let d2 = ((edx + 1) % 24) as u32;
                let d3 = ((edx + 0x15) % 24) as u32;
                let d4 = ((edx + 2) % 24) as u32;
                ret = d4;
                let s2 = if (k as i32).wrapping_sub(2) as u32 > 3 { one } else { neg1 };
                let s3 = if (k as i32) <= 3 { one } else { neg1 };
                // Tree 0 (sign s7, slot s).
                let mut x0 = pick_max(
                    mul_mem_sign(rdf(p20), s7),
                    mul_mem_sign(rdf(p1c), s7),
                );
                let x1b = pick_max(
                    mul_mem_sign(rdf(this + (i30 + r) * 4), s7),
                    mul_mem_sign(rdf(this + (i5c + r) * 4), s7),
                );
                x0 = pick_max(x0, x1b);
                x0 = pick_max(x0, mul_mem_sign(rdf(cell(this, 0x1c20, q + 4 * s)), s7));
                x0 = mul_mem_sign(x0, s7);
                wrf(wc - 0x200, x0);
                wrf(wc, x0);
                // Tree 1 (sign s2, mod-24 table at 0x1840).
                let mut y0 = pick_max(
                    mul_mem_sign(rdf(p18), s2),
                    mul_mem_sign(rdf(p14), s2),
                );
                let y5m = pick_max(
                    mul_mem_sign(rdf(this + (i38 + r) * 4), s2),
                    mul_mem_sign(rdf(this + (i60 + r) * 4), s2),
                );
                let mut y1 = pick_max(
                    mul_mem_sign(rdf(edi_r - 0x60), s2),
                    mul_mem_sign(rdf(cell(this, 0x1840, d0)), s2),
                );
                y1 = pick_max(
                    y1,
                    pick_max(
                        mul_mem_sign(rdf(cell(this, 0x1840, d1)), s2),
                        mul_mem_sign(rdf(cell(this, 0x1840, d2)), s2),
                    ),
                );
                y0 = pick_max(y0, y5m);
                y0 = pick_max(y0, y1);
                y1 = pick_max(y0, mul_mem_sign(rdf(cell(this, 0x1840, d3)), s2));
                y1 = pick_max(y1, mul_mem_sign(rdf(cell(this, 0x1840, d4)), s2));
                y0 = mul_mem_sign(y0, s2);
                y1 = mul_mem_sign(y1, s2);
                wrf(wc - 0x204, y0);
                wrf(wc - 4, y1);
                // Tree 2 (sign s3, mod-24 table at 0x18a0).
                let mut z0 = pick_max(
                    mul_mem_sign(rdf(p28), s3),
                    mul_mem_sign(rdf(p2c), s3),
                );
                let z4m = pick_max(
                    mul_mem_sign(rdf(this + (i3c + r) * 4), s3),
                    mul_mem_sign(rdf(this + (i68 + r) * 4), s3),
                );
                let mut z1 = pick_max(
                    mul_mem_sign(rdf(edi_r), s3),
                    mul_mem_sign(rdf(cell(this, 0x18a0, d0)), s3),
                );
                z1 = pick_max(
                    z1,
                    pick_max(
                        mul_mem_sign(rdf(cell(this, 0x18a0, d1)), s3),
                        mul_mem_sign(rdf(cell(this, 0x18a0, d2)), s3),
                    ),
                );
                z0 = pick_max(z0, z4m);
                z0 = pick_max(z0, z1);
                z1 = pick_max(z0, mul_mem_sign(rdf(cell(this, 0x18a0, d3)), s3));
                z1 = pick_max(z1, mul_mem_sign(rdf(cell(this, 0x18a0, d4)), s3));
                z0 = mul_mem_sign(z0, s3);
                z1 = mul_mem_sign(z1, s3);
                wrf(wc - 0x208, z0);
                wrf(wc - 8, z1);
                p20 += 4;
                p1c += 4;
                p18 += 4;
                p14 += 4;
                p28 += 4;
                p2c += 4;
                edi_r += 0x0c;
                wc += 0x10;
            }
        }
        ret
    }
}

rt::export!(thiscall, rw_00970d70(this: u32) -> u32 {
    unsafe { body(this) }
});
