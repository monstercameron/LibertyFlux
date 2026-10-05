// original: 0x008a8230 audio_voice_mix_bias10
//
// Mix three weights through an indexed coefficient row into four outputs.
//
// Arguments: `obj` (ECX) is the voice object, `out` points at four output
// floats, `weights` at three input floats, `index` selects the row as
// `obj + ((tls_base + (index + 0xA) * 4) << 6)`. Each output is
// `row[a]*w1 + row[b]*w0 + row[c]*w2 + row[d]`; outputs 0-2 are then
// scaled by `obj[0x1714]` while output 3 is stored unscaled. Returns
// `out`. No calls, no global writes; reads one global (the TLS slot
// number) and one TLS slot. Thiscall with three stack args.

use lf_checker_rt::{export, global, tls_slot};

/// File VA of the global holding the TLS slot number of the voice table.
const TLS_SLOT_INDEX: u32 = 0x17aba14;
/// Field in the thread's TLS struct holding the voice-table base.
const TLS_VOICE_BASE_OFF: u32 = 0x70;
/// Per-object scale factor applied to outputs 0, 1 and 2 (not 3).
const VOICE_SCALE_OFF: u32 = 0x1714;
/// Bytes per coefficient row selected by the biased index.
const ROW_STRIDE_BITS: u32 = 6;
/// Index bias selecting this instance's coefficient rows.
const ROW_BIAS: u32 = 0xA;

#[inline(always)]
fn fmul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}
#[inline(always)]
fn fadd(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}
#[inline(always)]
unsafe fn rf32(addr: u32) -> f32 {
    unsafe { (addr as *const f32).read_unaligned() }
}
#[inline(always)]
unsafe fn ru32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read_unaligned() }
}
#[inline(always)]
unsafe fn wf32(addr: u32, v: f32) {
    unsafe { (addr as *mut f32).write_unaligned(v) }
}

export!(thiscall, rw_008a8230(obj: u32, out: u32, weights: u32, index: u32) -> u32 {
    unsafe {
        let slot = global::<u32>(TLS_SLOT_INDEX).read();
        let tls = tls_slot(slot as usize);
        let base = ru32(tls.wrapping_add(TLS_VOICE_BASE_OFF));
        let w0 = rf32(weights);
        let w1 = rf32(weights.wrapping_add(4));
        let w2 = rf32(weights.wrapping_add(8));
        let row = obj.wrapping_add(
            base
                .wrapping_add(index.wrapping_add(ROW_BIAS).wrapping_mul(4))
                .wrapping_shl(ROW_STRIDE_BITS),
        );
        let m = |off: u32| rf32(row.wrapping_add(off));
        let scale = rf32(obj.wrapping_add(VOICE_SCALE_OFF));
        // Output 0: m10*w1 + m0*w0 + m20*w2 + m30, scaled.
        let mut o0 = fmul(m(0x10), w1);
        o0 = fadd(o0, fmul(m(0x00), w0));
        o0 = fadd(o0, fmul(m(0x20), w2));
        o0 = fadd(o0, m(0x30));
        o0 = fmul(o0, scale);
        // Output 1: m14*w1 + m4*w0 + m24*w2 + m34, scaled.
        let mut o1 = fmul(m(0x14), w1);
        o1 = fadd(o1, fmul(m(0x04), w0));
        o1 = fadd(o1, fmul(m(0x24), w2));
        o1 = fadd(o1, m(0x34));
        o1 = fmul(o1, scale);
        // Output 2: m18*w1 + m8*w0 + m28*w2 + m38, scaled.
        let mut o2 = fmul(m(0x18), w1);
        o2 = fadd(o2, fmul(m(0x08), w0));
        o2 = fadd(o2, fmul(m(0x28), w2));
        o2 = fadd(o2, m(0x38));
        o2 = fmul(o2, scale);
        // Output 3: m1c*w1 + m0c*w0 + m2c*w2 + m3c, NOT scaled.
        let mut o3 = fmul(m(0x1c), w1);
        o3 = fadd(o3, fmul(m(0x0c), w0));
        o3 = fadd(o3, fmul(m(0x2c), w2));
        o3 = fadd(o3, m(0x3c));
        wf32(out, o0);
        wf32(out.wrapping_add(4), o1);
        wf32(out.wrapping_add(8), o2);
        wf32(out.wrapping_add(12), o3);
        out
    }
});
