// original: 0x00969E10 sample_slot_scorer (proposed)

use lf_checker_rt::{callee_thiscall, export, global, relocated, tls_slot};

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

// Float helpers that pin the original's operand order: both operands pass
// through black_box so the compiler can neither fold nor commute them, and
// each helper emits exactly one scalar SSE instruction.
#[inline(always)]
fn fadd(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

#[inline(always)]
fn fsub(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}

#[inline(always)]
fn fmul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

#[inline(always)]
fn fdiv(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) / core::hint::black_box(b)
}

#[inline(always)]
fn fsqrt(a: f32) -> f32 {
    core::hint::black_box(a).sqrt()
}

/// Bitwise blend of two float triples under a select mask: bits from `a`
/// where the mask is set, from `b` where it is clear. Matches the
/// original's andps/andnps/orps triple exactly.
#[inline(always)]
fn blend3(a: [f32; 3], mask: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    let mut out = [0.0f32; 3];
    for i in 0..3 {
        out[i] = f32::from_bits(
            (a[i].to_bits() & mask[i].to_bits())
                | (!mask[i].to_bits() & b[i].to_bits()),
        );
    }
    out
}


// ---------------------------------------------------------------------------
// fn 0x00969E10
// ---------------------------------------------------------------------------

/// Score eight sample slots against a thread-state table row.
///
/// `this` holds float tables, `samples` points at two input floats, `out` at
/// eight output slots (null `out` returns immediately). For each slot `i` the
/// function loads a vector triple from `this + 0x1D50 + i * 16`, two memory
/// words, forms two squared distances (sample-vs-table and input-vs-memory),
/// thresholds each against three data thresholds (strictly greater, unordered
/// counts as not greater) to build select masks, blends normalized direction
/// triples against a constant vector under the masks, and calls a validator
/// helper with pointers to its scratch buffers. A zero answer stores 0; a
/// nonzero answer runs a rotation-like combination of the validator's four
/// output words with the vector triple, scales it by the second blend, calls
/// a scalar helper, and stores `slot[i] * answer`.
///
/// Returns `this + 0x1C80` on the full path. On the null-`out` path the
/// original returns its entry EAX (garbage); the rewrite returns 0 there and
/// that path is proven with no return comparison.
///
/// Original: 0x00969E10 (thiscall, two stack words).
const F3_TLS_INDEX_VA: u32 = 0x17ABA14;
const F3_TABLE_VA: u32 = 0x115DF20;
const F3_TLS_ROW_OFF: u32 = 0x70;
const F3_ROW_SHIFT: u32 = 6;
const F3_ONE_VA: u32 = 0xFE88E8;
const F3_K_VA: u32 = 0xFE8A24;
const F3_TH0_VA: u32 = 0x110DAD8;
const F3_TH1_VA: u32 = 0x110DAD4;
const F3_TH2_VA: u32 = 0x110DAD0;
const F3_SELECT_VA: u32 = 0x17AD148;
const F3_ELSE_VA: u32 = 0x110DB50;
const F3_VEC_OFF: u32 = 0x1D54;
const F3_SLOT_OFF: u32 = 0x1C60;
const F3_CALLEE2_OFF: u32 = 0x1E48;

export!(thiscall, rw_00969E10(this: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        if a2 == 0 {
            // The original falls through to the epilogue with entry EAX
            // still in place; a Rust rewrite cannot observe entry EAX, so
            // the early-out stage proves this path with no return check.
            return 0;
        }
        let tls_idx = rd32(global::<u32>(F3_TLS_INDEX_VA) as u32);
        let tls_obj = tls_slot(tls_idx as usize);
        let row = rd32(tls_obj.wrapping_add(F3_TLS_ROW_OFF));
        let table = relocated(F3_TABLE_VA).wrapping_add(row.wrapping_shl(F3_ROW_SHIFT));
        let table0 = rdf(table);
        let table1 = rdf(table.wrapping_add(4));
        let one = rdf(relocated(F3_ONE_VA));
        let kval = rdf(relocated(F3_K_VA));
        let th0 = rdf(relocated(F3_TH0_VA));
        let th1 = rdf(relocated(F3_TH1_VA));
        let th2 = rdf(relocated(F3_TH2_VA));
        let select = rdf(relocated(F3_SELECT_VA));
        let else_base = relocated(F3_ELSE_VA);
        let else0 = rdf(else_base);
        let else1 = rdf(else_base.wrapping_add(4));
        let else2 = rdf(else_base.wrapping_add(8));
        // The fourth else lane blends into a word the original never reads.
        let a0 = rdf(a1);
        let a1v = rdf(a1.wrapping_add(4));
        let zero = 0.0f32;
        let mut e1 = this.wrapping_add(F3_SLOT_OFF);
        let mut edi: u32 = 0;
        // Fixed scratch buffers reused across iterations like the original's
        // frame slots; the wbuf words persist between calls (only the callee
        // stub writes them), so they must not be re-zeroed per iteration.
        let mut wbuf = [0u32; 4];
        while edi < 8 {
            let esi = this
                .wrapping_add(F3_VEC_OFF)
                .wrapping_add(edi.wrapping_mul(16));
            let v0 = rdf(esi.wrapping_sub(4));
            let v1 = rdf(esi);
            let v2 = rdf(esi.wrapping_add(4));
            let m1 = rdf(esi.wrapping_sub(0x80));
            let m0 = rdf(esi.wrapping_sub(0x84));
            // First distance: (table[1]-m1)^2 + (table[0]-m0)^2.
            let m1d = fsub(table1, m1);
            let m0d = fsub(table0, m0);
            let a1d = fsub(a0, m0);
            let a1pd = fsub(a1v, m1);
            let d1 = fadd(fmul(m1d, m1d), fmul(m0d, m0d));
            let sel_a = if d1 > th0 { select } else { zero };
            let sel_b = if d1 > th1 { select } else { zero };
            let sel_c = if d1 > th2 { select } else { zero };
            let n1 = fdiv(one, fsqrt(d1));
            let vv1 = [fmul(n1, m0d), fmul(n1, m1d), fmul(n1, zero)];
            let b1 = blend3(vv1, [sel_c, sel_b, sel_a], [else0, else1, else2]);
            // Second distance: (a1[4]-m1)^2 + (a1[0]-m0)^2.
            let d2 = fadd(fmul(a1pd, a1pd), fmul(a1d, a1d));
            let sel_a2 = if d2 > th0 { select } else { zero };
            let sel_b2 = if d2 > th1 { select } else { zero };
            let sel_c2 = if d2 > th2 { select } else { zero };
            let n2 = fdiv(one, fsqrt(d2));
            let vv2 = [fmul(n2, a1d), fmul(n2, a1pd), fmul(n2, zero)];
            let b2 = blend3(vv2, [sel_c2, sel_b2, sel_a2], [else0, else1, else2]);
            // Validator call through scratch buffers. The stub answers AL
            // and fills four words; the scratch addresses differ per side
            // so only the contents are compared, not the pointers.
            let mut vbuf = [v0.to_bits(), v1.to_bits(), v2.to_bits()];
            let mut abuf = [b1[0].to_bits(), b1[1].to_bits(), b1[2].to_bits(), 0u32];
            let answered: u32 = callee_thiscall!(
                1,
                u32,
                wbuf.as_mut_ptr() as u32,
                abuf.as_mut_ptr() as u32,
                vbuf.as_mut_ptr() as u32
            );
            let al = (answered & 0xFF) as u8;
            if al == 0 {
                wr32(a2.wrapping_add(edi.wrapping_mul(4)), 0);
            } else {
                let w0 = f32::from_bits(wbuf[0]);
                let w1 = f32::from_bits(wbuf[1]);
                let w2 = f32::from_bits(wbuf[2]);
                let w3 = f32::from_bits(wbuf[3]);
                let dot = fadd(fadd(fmul(w0, v0), fmul(w1, v1)), fmul(w2, v2));
                let kw3 = fmul(w3, kval);
                let kdot = fmul(dot, kval);
                let c0 = fsub(fmul(fmul(w3, w3), kval), one);
                let w2k = fmul(w2, kdot);
                let w0k = fmul(w0, kdot);
                let w1k = fmul(w1, kdot);
                let r0 = fadd(fmul(v0, c0), w0k);
                let r1 = fadd(fmul(c0, v1), w1k);
                let q2 = fadd(fmul(v2, c0), w2k);
                let t2 = fsub(fmul(w1, v2), fmul(w2, v1));
                let t1 = fsub(fmul(w2, v0), fmul(w0, v2));
                let t0 = fsub(fmul(w0, v1), fmul(w1, v0));
                let r2v = fadd(fmul(t2, kw3), r0);
                let r1p = fadd(fmul(t1, kw3), r1);
                let q2p = fadd(fmul(t0, kw3), q2);
                let b0 = b2[0];
                let bb1 = b2[1];
                let bb2 = b2[2];
                let s = fadd(
                    fadd(fmul(r1p, bb1), fmul(r2v, b0)),
                    fmul(q2p, bb2),
                );
                let rbits: u32 =
                    callee_thiscall!(2, u32, this.wrapping_add(F3_CALLEE2_OFF), s.to_bits());
                let out = fmul(rdf(e1), f32::from_bits(rbits));
                wr32(a2.wrapping_add(edi.wrapping_mul(4)), out.to_bits());
            }
            e1 = e1.wrapping_add(4);
            edi = edi.wrapping_add(1);
        }
        e1
    }
});

/// Wrong version of [`rw_00969E10`]: stores the slot value without scaling
/// it by the scalar helper's answer. Must fail the contract.
/// call's result instead of adding it. Must fail the contract.
export!(thiscall, mut_00968270(this: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    unsafe {
        let sample_x = rdf(a2);
        let sample_y = rdf(a2.wrapping_add(4));
        let sample_z = rdf(a2.wrapping_add(8));
        let row_off = a1.wrapping_add(F1_INDEX_BIAS).wrapping_shl(4);
        let base = this.wrapping_add(row_off);
        let dy = fsub(sample_y, rdf(base.wrapping_add(4)));
        let dx = fsub(sample_x, rdf(base));
        let dz = fsub(sample_z, rdf(base.wrapping_add(8)));
        let d1 = fadd(fadd(fmul(dy, dy), fmul(dx, dx)), fmul(dz, dz));
        let tls_idx = rd32(global::<u32>(F1_TLS_INDEX_VA) as u32);
        let tls_obj = tls_slot(tls_idx as usize);
        let row = rd32(tls_obj.wrapping_add(F1_TLS_ROW_OFF));
        let table1 = relocated(F1_TABLE1_VA).wrapping_add(row.wrapping_shl(F1_ROW_SHIFT));
        let qy = fsub(sample_y, rdf(table1.wrapping_add(4)));
        let qx = fsub(sample_x, rdf(table1));
        let qz = fsub(sample_z, rdf(table1.wrapping_add(8)));
        let d4 = fadd(fadd(fmul(qy, qy), fmul(qx, qx)), fmul(qz, qz));
        let sqrt_d1 = fsqrt(d1);
        let slot_sum = rdf(this.wrapping_add(a1.wrapping_mul(4)).wrapping_add(F1_SLOT_SUM_OFF));
        let t = fsub(fadd(slot_sum, sqrt_d1), fsqrt(d4));
        let r1 = callee_thiscall!(1, u32, this.wrapping_add(F1_CALLEE_OBJ_OFFS[0]), t.to_bits());
        let trunc = fistp_chop_i64(f32::from_bits(r1)) as u32;
        let r2 = callee_thiscall!(1, u32, this.wrapping_add(F1_CALLEE_OBJ_OFFS[1]), sqrt_d1.to_bits());
        let t3 = rdf(this.wrapping_add(a1.wrapping_mul(4)).wrapping_add(F1_SLOT_T3_OFF));
        let r3 = callee_thiscall!(1, u32, this.wrapping_add(F1_CALLEE_OBJ_OFFS[2]), t3.to_bits());
        let r4 = callee_cdecl!(2, u32, r3);
        let sum = fsub(f32::from_bits(r4), f32::from_bits(r2)); // BUG: subtracts
        let table2 = relocated(F1_TABLE2_VA).wrapping_add(row.wrapping_shl(F1_ROW_SHIFT));
        let vx = rdf(base);
        let vy = rdf(base.wrapping_add(4));
        let vz = rdf(base.wrapping_add(8));
        let ex = fsub(vx, rdf(table2));
        let ey = fsub(vy, rdf(table2.wrapping_add(4)));
        let ez = fsub(vz, rdf(table2.wrapping_add(8)));
        let d2 = fadd(fadd(fmul(ey, ey), fmul(ex, ex)), fmul(ez, ez));
        let factor = if d2 == 0.0 || d2.is_nan() { d2 } else { fdiv(1.0, fsqrt(d2)) };
        let ox = fadd(vx, fmul(fmul(ex, factor), sqrt_d1));
        let oy = fadd(vy, fmul(fmul(ey, factor), sqrt_d1));
        let oz = fadd(vz, fmul(fmul(ez, factor), sqrt_d1));
        if a3 == 0 {
            return table2;
        }
        if a4 == 0 {
            return 0;
        }
        wr32(a3, trunc);
        wr32(a4, sum.to_bits());
        wr32(a5, ox.to_bits());
        wr32(a5.wrapping_add(4), oy.to_bits());
        wr32(a5.wrapping_add(8), oz.to_bits());
        wr32(a5.wrapping_add(12), 0);
        a5
    }
});
