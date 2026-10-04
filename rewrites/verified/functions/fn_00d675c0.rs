// original: 0x00d675c0 emit_indexed_record
//
// Fans out through a sibling routine once the counter reaches its
// threshold, then classifies the input vector: a zero vector takes the
// projection call, a nonzero vector takes two solver calls whose accepted
// answers are emitted. Either way the frame basis, the source triple and
// the quad block are stored into the record selected by the counter, and
// the counter is incremented.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, relocated};

// Callee ids (see contract).
const S1_FANOUT: u32 = 11; // 0xD66CE0 cdecl/5: gated fan-out (sibling fn1)
const S2_SOLVE_A: u32 = 12; // 0x9BF120 cdecl/4, first site: (key, vec, aux, out)
const S3_SOLVE_B: u32 = 13; // 0x9BF120 cdecl/4, second site: (vec, tag, aux, out)
const S4_PROJECT: u32 = 14; // 0x4181B0 thiscall/0: zero-path projection
const S5_EMIT: u32 = 15; // 0x4E8880 thiscall/2: emit (basis, aux, value)

const C_ONE: u32 = 0xFE88E8; // float constant +1.0
const C_NEG_ONE: u32 = 0xFE8D94; // float constant -1.0
const FANOUT_MIN: u32 = 0x71C; // counter threshold for the fan-out call

#[inline(always)]
unsafe fn rd32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr32(addr: u32, v: u32) {
    unsafe { (addr as *mut u32).write_unaligned(v) }
}

/// Rewrite of the original function at 0x00D675C0
/// (cdecl/9 -> u32, returns the fourth word of the `quad` block).
///
/// Fans out through a sibling routine once the counter reaches its
/// threshold, then classifies the input vector: a zero vector takes the
/// projection call, a nonzero vector takes two solver calls whose accepted
/// answers are emitted. Either way the frame basis, the source triple and
/// the quad block are stored into the record selected by the counter, and
/// the counter is incremented.
export!(cdecl, rw_d675c0(
    triple: u32,
    tag: u32,
    vec: u32,
    quad: u32,
    rec: u32,
    ctr_obj: u32,
    w6: u32,
    w7: u32,
    mode: u32,
) -> u32 {
    unsafe {
        let ctr = rd32(ctr_obj);
        if ctr >= FANOUT_MIN {
            callee_cdecl!(S1_FANOUT, u32, rec, ctr_obj, w6, w7, mode);
        }
        let mut fr = [0u32; 28];
        // Basis block: ones on the diagonal slots, zeros elsewhere.
        fr[0x30 / 4] = 0x3F80_0000;
        fr[0x44 / 4] = 0x3F80_0000;
        fr[0x58 / 4] = 0x3F80_0000;
        let base = fr.as_mut_ptr() as u32;
        let f0 = (vec as *const f32).read_unaligned();
        let f1 = ((vec + 4) as *const f32).read_unaligned();
        // NaN-aware nonzero test, matching ucomiss+test/jp: NaN counts as
        // nonzero, both signed zeros count as zero.
        if f0 != 0.0 || f1 != 0.0 {
            let key_mid = rd32(relocated(if mode == 1 { C_NEG_ONE } else { C_ONE }));
            // First solver call: key record (0, key, 0), out-slot beside it.
            fr[0x10 / 4] = 0;
            fr[0x14 / 4] = key_mid;
            fr[0x18 / 4] = 0;
            let a1 = callee_cdecl!(S2_SOLVE_A, u32, base + 0x10, vec, base + 0x20, base + 0x0C);
            if (a1 & 0xFF) != 0 {
                let v = fr[0x0C / 4];
                callee_thiscall!(S5_EMIT, u32, base + 0x30, base + 0x20, v);
            }
            // Second solver call: same out-slot, vector and tag instead.
            let a2 = callee_cdecl!(S3_SOLVE_B, u32, vec, tag, base + 0x20, base + 0x0C);
            if (a2 & 0xFF) != 0 {
                let v = fr[0x0C / 4];
                callee_thiscall!(S5_EMIT, u32, base + 0x30, base + 0x20, v);
            }
        } else {
            callee_thiscall!(S4_PROJECT, u32, base + 0x30);
        }
        // Store the record: basis words, source triple, quad dwords.
        let dst = rec.wrapping_add(ctr.wrapping_mul(5).wrapping_shl(4));
        wr32(dst, fr[0x30 / 4]);
        wr32(dst + 4, fr[0x34 / 4]);
        wr32(dst + 8, fr[0x38 / 4]);
        wr32(dst + 0x10, fr[0x40 / 4]);
        wr32(dst + 0x14, fr[0x44 / 4]);
        wr32(dst + 0x18, fr[0x48 / 4]);
        wr32(dst + 0x20, fr[0x50 / 4]);
        wr32(dst + 0x24, fr[0x54 / 4]);
        wr32(dst + 0x28, fr[0x58 / 4]);
        wr32(dst + 0x30, rd32(triple));
        wr32(dst + 0x34, rd32(triple + 4));
        wr32(dst + 0x38, rd32(triple + 8));
        let qd = rec.wrapping_add(ctr.wrapping_mul(5).wrapping_mul(2).wrapping_mul(8));
        wr32(qd + 0x40, rd32(quad));
        wr32(qd + 0x44, rd32(quad + 4));
        let qret = rd32(quad + 8);
        wr32(qd + 0x48, qret);
        let out = rd32(quad + 12);
        wr32(qd + 0x4C, out);
        wr32(ctr_obj, ctr.wrapping_add(1));
        out
    }
});
