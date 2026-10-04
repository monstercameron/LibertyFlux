// original: 0x00dbd450 bake_scaled_table_rows
//
// Bakes one row per active slot into a fixed global table: each slot reads a
// u16 key from an index array, then either expands three packed u16 samples
// (scaled, biased per channel, translated) or copies a 16-byte record as-is,
// optionally followed by a 3x4 matrix transform. Runs once per process: a
// global done-flag short-circuits later calls. Returns nothing.

use lf_checker_rt::{export, global, relocated};

/// Done flag: nonzero at entry means all rows are already baked.
const G_DONE: u32 = 0x017A337C;
/// Global float the packed samples are scaled by.
const G_SCALE: u32 = 0x00FE8674;
/// Global output table base; row i starts at OUT_BASE + i * ROW_STRIDE.
const OUT_BASE: u32 = 0x017A1DD0;
/// Bytes per baked row.
const ROW_STRIDE: u32 = 16;
/// Bits of the control word holding the row count (value >> 21) & 0xF.
const COUNT_BITS: u32 = 0x01E00000;
/// Mask of the start-slot field of the control word's second dword.
const SLOT_MASK: u32 = 0x0001FFFF;

#[inline(always)]
unsafe fn rd8(addr: u32) -> u8 {
    unsafe { (addr as *const u8).read() }
}

#[inline(always)]
unsafe fn rd16(addr: u32) -> u16 {
    // SAFETY: contract-fabricated tables; the original reads aligned u16s.
    unsafe { (addr as *const u16).read() }
}

#[inline(always)]
unsafe fn rd32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read() }
}

#[inline(always)]
unsafe fn rdf(addr: u32) -> f32 {
    unsafe { (addr as *const f32).read() }
}

#[inline(always)]
unsafe fn wrf(addr: u32, v: f32) {
    unsafe { (addr as *mut f32).write(v) }
}

#[inline(always)]
unsafe fn wr32(addr: u32, v: u32) {
    unsafe { (addr as *mut u32).write(v) }
}

/// Bake rows for `ctl` (control block) described by `obj` (parameter block).
/// `ctl[0]` carries the row count in bits 21..=24, `ctl[1]` the start slot;
/// `obj` holds the matrix, biases, table pointers and mode flags.
unsafe fn bake_rows(obj: u32, ctl: u32) {
    unsafe {
        let flag = global::<u8>(G_DONE);
        if flag.read() != 0 {
            return;
        }
        let scale = global::<f32>(G_SCALE).read();
        let w0 = rd32(ctl);
        if w0 & COUNT_BITS == 0 {
            flag.write(1);
            return;
        }
        let count = (w0 >> 21) & 0xF;
        let base = rd32(ctl.wrapping_add(4)) & SLOT_MASK;
        let mode = rd8(obj.wrapping_add(0x50));
        let key_table = rd32(obj.wrapping_add(0x60));
        let sample_table = rd32(obj.wrapping_add(0x58));
        let record_table = rd32(obj.wrapping_add(0x5C));
        let out_base = relocated(OUT_BASE);
        let mut i: u32 = 0;
        while i < count {
            let idx = base.wrapping_add(i);
            let row = out_base.wrapping_add(i.wrapping_mul(ROW_STRIDE));
            let key = rd16(key_table.wrapping_add(idx.wrapping_mul(2))) as u32;
            if mode & 1 != 0 {
                // Packed path: three u16 samples scaled, biased, translated.
                let d = key.wrapping_mul(3);
                let s = sample_table.wrapping_add(d.wrapping_mul(2));
                let mut x = rd16(s) as f32 * scale;
                let mut y = rd16(s.wrapping_add(2)) as f32 * scale;
                let mut z = rd16(s.wrapping_add(4)) as f32 * scale;
                x *= rdf(obj.wrapping_add(0x40));
                y *= rdf(obj.wrapping_add(0x44));
                z *= rdf(obj.wrapping_add(0x48));
                let t = rd32(obj.wrapping_add(0x90));
                x += rdf(t);
                y += rdf(t.wrapping_add(4));
                z += rdf(t.wrapping_add(8));
                wrf(row, x);
                wrf(row.wrapping_add(4), y);
                wrf(row.wrapping_add(8), z);
            } else {
                // Record path: copy the 16-byte record verbatim.
                let p = record_table.wrapping_add(key.wrapping_shl(4));
                wr32(row, rd32(p));
                wr32(row.wrapping_add(4), rd32(p.wrapping_add(4)));
                wr32(row.wrapping_add(8), rd32(p.wrapping_add(8)));
                wr32(row.wrapping_add(12), rd32(p.wrapping_add(12)));
            }
            if mode & 4 != 0 {
                // Matrix path: 3x4 transform of the row, in the original's
                // exact operation order. The w slot receives the function's
                // uninitialized stack word, which the contract pins to zero.
                let ox = rdf(row);
                let oy = rdf(row.wrapping_add(4));
                let oz = rdf(row.wrapping_add(8));
                let mut x0 = rdf(obj);
                let mut x6 = rdf(obj.wrapping_add(0x10));
                let mut x2 = rdf(obj.wrapping_add(0x14));
                let mut x1 = rdf(obj.wrapping_add(0x18));
                x0 *= ox;
                x6 *= oy;
                x2 *= oy;
                x6 += x0;
                x0 = rdf(obj.wrapping_add(0x20));
                x0 *= oz;
                x1 *= oy;
                x6 += x0;
                x0 = rdf(obj.wrapping_add(0x04));
                x0 *= ox;
                x6 += rdf(obj.wrapping_add(0x30));
                x2 += x0;
                x0 = rdf(obj.wrapping_add(0x24));
                x0 *= oz;
                x2 += x0;
                x0 = rdf(obj.wrapping_add(0x08));
                x0 *= ox;
                x2 += rdf(obj.wrapping_add(0x34));
                x1 += x0;
                x0 = rdf(obj.wrapping_add(0x28));
                x0 *= oz;
                x1 += x0;
                x1 += rdf(obj.wrapping_add(0x38));
                wrf(row, x6);
                wrf(row.wrapping_add(4), x2);
                wr32(row.wrapping_add(12), 0);
                wrf(row.wrapping_add(8), x1);
            }
            i = i.wrapping_add(1);
        }
        flag.write(1);
    }
}

export!(cdecl, rw_dbd450(obj: u32, ctl: u32) -> u32 {
    unsafe { bake_rows(obj, ctl) };
    0
});
