// original: 0x00afcb30 proximity_grid_scan

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};
/// Read one little-endian u16 field from a heap object at `base`.
#[inline(always)]
fn rd16(base: u32, off: u32) -> u16 {
    unsafe { ((base.wrapping_add(off)) as *const u16).read_unaligned() }
}

/// Read one byte field from a heap object at `base`.
#[inline(always)]
fn rd8(base: u32, off: u32) -> u8 {
    unsafe { ((base.wrapping_add(off)) as *const u8).read() }
}

/// Proximity-grid scan (original 0x00AFCB30, cdecl/3).
///
/// Takes a 2-float probe point `a0`, a 3-float position `a1` and a float
/// range `a2`. A gate call runs first; when it passes, four clamped index
/// bounds are derived through a thiscall helper, a table of 32-byte record
/// runs is walked, records nearer than the squared range are flagged, and a
/// per-record hook fans out over a small row range. Returns the gate answer
/// on the early path, otherwise the second index bound.
macro_rules! afcb30_body {
    ($c6end:expr, $a0:ident, $a1:ident, $a2:ident) => {{
        // Gate call; nonzero low byte returns straight out.
        let g0 = callee_cdecl!(1, u32,);
        if (g0 & 0xFF) != 0 {
            return g0;
        }
        let bound = f32::from_bits(global::<u32>(0x00FE8BB0).read());
        let tag = global::<u8>(0x0103FFDB).read();
        let near: u32 = if tag == (g0 & 0xFF) as u8 {
            1
        } else {
            // Squared length of a1 in the original's accumulation order.
            let v0 = (($a1) as *const f32).read();
            let v1 = (($a1).wrapping_add(4) as *const f32).read();
            let v2 = (($a1).wrapping_add(8) as *const f32).read();
            let d = v0 * v0 + v1 * v1 + v2 * v2;
            // Original: comiss + jbe-skips, i.e. flag 1 only when d > bound.
            if d > bound { 1 } else { 0 }
        };
        // Range window around the probe point.
        let scale = f32::from_bits(global::<u32>(0x0103FF5C).read());
        let t = scale * f32::from_bits($a2);
        let t2 = t * t;
        let s = t + bound;
        let p0 = (($a0) as *const f32).read();
        let p1 = (($a0).wrapping_add(4) as *const f32).read();
        let this1 = relocated(0x01177A80);
        let b0 = callee_thiscall!(2, u32, this1, (p0 - s).to_bits());
        let b1 = callee_thiscall!(3, u32, this1, (p0 + s).to_bits());
        let b2 = callee_thiscall!(4, u32, this1, (p1 - s).to_bits());
        let b3 = callee_thiscall!(5, u32, this1, (p1 + s).to_bits());
        if (b0 as i32) > (b1 as i32) {
            return b1;
        }
        let table = relocated(0x01178284);
        let kval = f32::from_bits(global::<u32>(0x00FE87A4).read());
        let m = global::<u32>(0x01173604).read() % 10;
        let mut loopvar = b0;
        loop {
            if (b2 as i32) <= (b3 as i32) {
                let idxval = loopvar.wrapping_add(b2.wrapping_mul(8));
                let mut slot = table.wrapping_add(idxval.wrapping_mul(4));
                let count = (b3 as i32).wrapping_sub(b2 as i32).wrapping_add(1);
                let mut ci = 0u32;
                while ci != count as u32 {
                    let entry = (slot as *const u32).read();
                    if entry != 0 {
                        // Range-query helper: fills the record-index window.
                        let mut mlo = 0u32;
                        let mut mhi = 0u32;
                        callee_thiscall!(6, u32, this1, idxval,
                            (p1 - s).to_bits(), (p1 + s).to_bits(),
                            (&mut mlo as *mut u32) as u32,
                            (&mut mhi as *mut u32) as u32);
                        let mult = ((slot.wrapping_add(0x300)) as *const u32).read();
                        let aa = (m as i32).wrapping_mul(mult as i32) / 10;
                        let bb = ((m.wrapping_add(1)) as i32).wrapping_mul(mult as i32) / 10;
                        let lo = aa.max(mlo as i32);
                        let hi = bb.min(mhi as i32);
                        if lo < hi {
                            let mut k = lo;
                            while k != hi {
                                let cur = entry.wrapping_add((k as u32).wrapping_mul(32));
                                let x = (rd16(cur, 0x14) as i16 as i32) as f32;
                                let y = (rd16(cur, 0x16) as i16 as i32) as f32;
                                let dx = x * kval - p0;
                                let dy = y * kval - p1;
                                let dist = dy * dy + dx * dx;
                                if t2 > dist {
                                    let f1f = rd8(cur, 0x1F);
                                    if (f1f & 0x40) == 0 {
                                        let set = f1f | 0x40;
                                        let f0 = rd8(cur, 0x1E);
                                        ((cur.wrapping_add(0x1F)) as *mut u8).write(set);
                                        if (f0 & 0x80) == 0 && near != 0 {
                                            let r3 = callee_thiscall!(7, u32, relocated(0x016DCEB8));
                                            if (r3 & 0xFF) != 0 {
                                                let r4 = callee_cdecl!(8, u32,);
                                                let mut flag2 = 0u32;
                                                let go: bool;
                                                if (r4.wrapping_add(8) as i32) > 0 {
                                                    flag2 = 1;
                                                    go = true;
                                                } else {
                                                    go = ((set >> 5) & 1) != 0;
                                                }
                                                if go {
                                                    let r5 = callee_thiscall!(9, u32,
                                                        global::<u32>(0x012E22A4).read());
                                                    if (r5 as i32) > 12 {
                                                        let start = rd16(cur, 0x12) as i16 as i32;
                                                        let cnt = (rd8(cur, 0x1E) & 0xF) as i32;
                                                        let end = start.wrapping_add(cnt);
                                                        if start < end {
                                                            let fw = ((end as u32) & 0xFFFFFF00)
                                                                | ((flag2 == 0) as u32);
                                                            let mut edi = start;
                                                            let stop = ($c6end)(end);
                                                            while edi != stop {
                                                                callee_cdecl!(10, u32, $a0, $a2, cur,
                                                                    edi as u32, fw);
                                                                edi = edi.wrapping_add(1);
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    let f = rd8(cur, 0x1F);
                                    ((cur.wrapping_add(0x1F)) as *mut u8).write(f & 0xBF);
                                }
                                k = k.wrapping_add(1);
                            }
                        }
                    }
                    slot = slot.wrapping_add(0x20);
                    ci = ci.wrapping_add(1);
                }
            }
            loopvar = loopvar.wrapping_add(1);
            if (loopvar as i32) > (b1 as i32) {
                break;
            }
        }
        b1
    }};
}

export!(cdecl, rw_afcb30(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe { afcb30_body!(|e: i32| e, a0, a1, a2) }
});
