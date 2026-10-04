// original: 0x00983080 audio_positional_update
use lf_k2_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

/// Unaligned little-endian u32 load from the 32-bit address space.
#[inline(always)]
unsafe fn rdu(base: u32, off: u32) -> u32 {
    ((base.wrapping_add(off)) as *const u32).read_unaligned()
}

/// Unaligned f32 load.
#[inline(always)]
unsafe fn rdf(base: u32, off: u32) -> f32 {
    ((base.wrapping_add(off)) as *const f32).read_unaligned()
}

/// Unaligned u16 load.
#[inline(always)]
unsafe fn rdu16(base: u32, off: u32) -> u16 {
    ((base.wrapping_add(off)) as *const u16).read_unaligned()
}

/// Wraparound range test used for voice selection: when `hi >= lo` the value
/// must lie in `[lo, hi)`; otherwise either side of the wrap passes.
#[inline(always)]
fn in_wrap(lo: u32, hi: u32, v: u32) -> bool {
    if hi >= lo {
        v < hi && v >= lo
    } else {
        v >= lo || v < hi
    }
}

/// Final voice-gain stage of the positional audio update: for each of the 16
/// voice slots that is live, runs the distance probe, shapes the answer
/// through a fast log2 approximation and programs the mixer voice.
unsafe fn audio_gain_stage(this: u32) {
    unsafe {
        let mut slot = this.wrapping_add(0x74);
        for _ in 0..16u32 {
            let handle = *(slot as *const u32);
            if handle != 0 {
                let v0 = rdf(slot, 0u32.wrapping_sub(0x24));
                // Out-parameter slots: the probe takes the address of the
                // zero word first and of the copied voice word second.
                let mut out_zero = 0u32;
                let mut out_copy = v0.to_bits();
                let r: f32 = callee_thiscall!(
                    5,
                    f32,
                    relocated(0x1165880),
                    &mut out_copy as *mut u32 as u32,
                    &mut out_zero as *mut u32 as u32
                );
                let r2: f32 = callee_thiscall!(6, f32, relocated(0x1238810), r.to_bits());
                let o0 = f32::from_bits(out_zero);
                let x0 = *global::<f32>(0xfe88e8) - o0;
                let mut x1 = r2 * x0;
                let s18bits = x1.to_bits();
                x1 -= *global::<f32>(0xfe8670);
                // Fast log2: exponent as double plus table bias, mantissa to
                // [1, 2), then a quadratic correction polynomial.
                let e = s18bits >> 23;
                let idx = e >> 31;
                let tab = global::<f64>(0xfe8f50).byte_add((idx as usize).wrapping_mul(8));
                let mut x2 = (e as f64 + *tab) as f32;
                x2 -= *global::<f32>(0xfe8bc4);
                x2 -= *global::<f32>(0xe78580);
                let m = f32::from_bits((s18bits & 0x7fffff) | 0x3f800000);
                let gate = *global::<f32>(0xfe8628);
                let out = if !(x1 >= gate) {
                    *global::<f32>(0xfe8df8)
                } else {
                    let mut t = m * *global::<f32>(0xe78584);
                    let mut m2 = m * m;
                    t += x2;
                    m2 *= *global::<f32>(0xe7857c);
                    t -= m2;
                    t *= *global::<f32>(0xe78578);
                    t *= *global::<f32>(0xfe8b38);
                    t
                };
                let b4 = *((handle as *const u8).byte_add(4));
                let dest = if b4 == 0xff {
                    0
                } else {
                    let k = *((handle as *const u8).byte_add(0x40)) as u32;
                    let mul = *global::<u32>(0x115d968);
                    let tab = *global::<u32>(0x115d988);
                    (b4 as u32).wrapping_mul(mul).wrapping_add(rdu(
                        tab.wrapping_add(k.wrapping_mul(0x6f40)),
                        0x6f14,
                    ))
                };
                callee_thiscall!(7, u32, dest, out.to_bits());
            }
            slot = slot.wrapping_add(0x30);
        }
    }
}

/// Positional audio update for one listener position.
///
/// This matches the listener against the 16-entry zone table (2-D box test
/// plus per-zone distance and flag tests), accumulates the weights of the
/// voices selected for this tick, spends a random draw to pick one voice,
/// spawns it into a free slot with randomized position, and finally runs
/// the gain stage over all live slots. Writes voice slots, the spawn mark
/// byte and per-voice timestamps; pure selection failures write nothing.
export!(thiscall, rw_00983080(this: u32, time: u32, pos: u32, flag_a: u32, flag_b: u32) -> () {
    unsafe {
        let px = rdf(pos, 0);
        let py = rdf(pos, 4);
        let pz = rdf(pos, 8);
        let fa = (flag_a & 0xFF) != 0;
        let fb = (flag_b & 0xFF) != 0;
        // Phase A: 2-D box test of the listener against this zone.
        let r = rdf(this, 0x3c);
        let x0 = rdf(this, 0x10);
        let x1 = rdf(this, 0x20);
        let y0 = rdf(this, 0x14);
        let y1 = rdf(this, 0x24);
        let in_box = px > x0 - r && x1 + r > px && py > y0 - r && y1 + r > py;
        // Phase B: score the 16 zone entries.
        let gsel = *global::<u32>(0x1295854);
        let g = if gsel != 0xffff_ffff {
            gsel
        } else {
            *global::<u32>(0x1295848)
        };
        let mut matches = 0u32;
        let mut ent = this.wrapping_add(0x68);
        for _ in 0..16u32 {
            let n = rdu(ent, 0xc);
            if n != 0 {
                let dx = px - rdf(ent, 0u32.wrapping_sub(0x18));
                let dy = py - rdf(ent, 0u32.wrapping_sub(0x14));
                let dz = pz - rdf(ent, 0u32.wrapping_sub(0x10));
                let d2 = dx * dx + dy * dy;
                let d2 = d2 + dz * dz;
                let hi = rdu(ent, 4);
                let lo = rdu(ent, 0);
                let flo = rdf(ent, 0u32.wrapping_sub(8));
                let fhi = rdf(ent, 0u32.wrapping_sub(4));
                let f9 = *((ent as *const u8).byte_add(9));
                let f8 = *((ent as *const u8).byte_add(8));
                if !(flo > d2) && !(d2 > fhi) && in_wrap(lo, hi, g) {
                    if (f9 == 0 || !fa) && (f8 == 0 || !fb) {
                        matches = matches.wrapping_add(1);
                        ent = ent.wrapping_add(0x30);
                        continue;
                    }
                }
                callee_thiscall!(1, u32, n, 0);
            }
            ent = ent.wrapping_add(0x30);
        }
        // Phase C: gates for the spawn loop.
        if !in_box {
            return audio_gain_stage(this);
        }
        let mark = (this as *mut u8).byte_add(0x350);
        if *mark == 0 {
            *mark = 1;
        }
        let limit = rdu(this, 0x38);
        if matches >= limit {
            return audio_gain_stage(this);
        }
        let arr_count = rdu(this, 0x30);
        let arr_base = rdu(this, 0x40);
        // Phase D: outer spawn loop.
        let mut outer = matches;
        while outer < limit {
            // Inner loop 1: accumulate weights of selected voices.
            let mut sum = 0f32;
            if arr_count != 0 {
                let mut i = 0u32;
                while i < arr_count {
                    let e = *((arr_base as *const u32).wrapping_add(i as usize));
                    let b17 = *((e as *const u8).byte_add(0x17)) as u32;
                    let b16 = *((e as *const u8).byte_add(0x16)) as u32;
                    if in_wrap(b16, b17, g) {
                        let fbyte = *((e as *const u8).byte_add(5));
                        if ((fbyte & 3) != 1 || !fa) && ((fbyte & 0xc) != 4 || !fb) {
                            let t = (rdu16(e, 0x18) as u32)
                                .wrapping_mul(1000)
                                .wrapping_add(rdu(e, 0x22));
                            if time > t {
                                sum += rdf(e, 0xa);
                            }
                        }
                    }
                    i = i.wrapping_add(1);
                }
            }
            if sum > 0.0 {
                let randval: f32 = callee_cdecl!(2, f32, 0, sum.to_bits());
                // Inner loop 2: spend the draw to pick one voice.
                if arr_count != 0 {
                    let mut rem = randval;
                    let mut idx = 0u32;
                    while idx < arr_count {
                        let e = *((arr_base as *const u32).wrapping_add(idx as usize));
                        let b17 = *((e as *const u8).byte_add(0x17)) as u32;
                        let b16 = *((e as *const u8).byte_add(0x16)) as u32;
                        if in_wrap(b16, b17, g) {
                            let fbyte = *((e as *const u8).byte_add(5));
                            if ((fbyte & 3) != 1 || !fa) && ((fbyte & 0xc) != 4 || !fb) {
                                let t = (rdu16(e, 0x18) as u32)
                                    .wrapping_mul(1000)
                                    .wrapping_add(rdu(e, 0x22));
                                if time > t {
                                    rem -= rdf(e, 0xa);
                                    if rem <= 0.0 {
                                        // First free spawn slot, if any.
                                        let mut s = 0u32;
                                        let mut found = false;
                                        while s < 16 {
                                            if rdu(this, 0x74 + s.wrapping_mul(48)) == 0 {
                                                found = true;
                                                break;
                                            }
                                            s = s.wrapping_add(1);
                                        }
                                        if found {
                                            // Clamp the spawn window per axis.
                                            let r1 = rdf(e, 0xe);
                                            let r2 = rdf(e, 0x12);
                                            let mut x5 = x1;
                                            if r1 > x1 - px {
                                                x5 = px - r1;
                                            }
                                            let mut x4 = x0;
                                            if r1 > px - x0 {
                                                x4 = px + r1;
                                            }
                                            let mut x3 = y1;
                                            if r1 > y1 - py {
                                                x3 = py - r1;
                                            }
                                            let mut x2 = y0;
                                            if r1 > py - y0 {
                                                x2 = py + r1;
                                            }
                                            let t = px - r2;
                                            if !(x4 > t) {
                                                x4 = t;
                                            }
                                            if !(x4 > x0) {
                                                x4 = x0;
                                            }
                                            let t = px + r2;
                                            if !(t > x5) {
                                                x5 = t;
                                            }
                                            if !(x1 > x5) {
                                                x5 = x1;
                                            }
                                            let t = py - r2;
                                            if !(x2 > t) {
                                                x2 = t;
                                            }
                                            if !(x2 > y0) {
                                                x2 = y0;
                                            }
                                            let t = py + r2;
                                            if !(t > x3) {
                                                x3 = t;
                                            }
                                            if !(y1 > x3) {
                                                x3 = y1;
                                            }
                                            let ra: f32 =
                                                callee_cdecl!(2, f32, x4.to_bits(), x5.to_bits());
                                            let rb: f32 =
                                                callee_cdecl!(2, f32, x2.to_bits(), x3.to_bits());
                                            let s48 = s.wrapping_mul(48);
                                            let slotw = this.wrapping_add(s48);
                                            *((slotw as *mut f32).byte_add(0x50)) = ra;
                                            *((slotw as *mut f32).byte_add(0x54)) = rb;
                                            // The original copies an
                                            // uninitialized frame word here;
                                            // under the checker's defined
                                            // stack fill (0) it reads 0.
                                            *((slotw as *mut u32).byte_add(0x5c)) = 0;
                                            *((slotw as *mut f32).byte_add(0x58)) = 20.0;
                                            let ge =
                                                *((arr_base as *const u32).wrapping_add(g as usize));
                                            let rr1 = rdf(ge, 0xe);
                                            *((slotw as *mut f32).byte_add(0x60)) = rr1 * rr1;
                                            let rr2 = rdf(e, 0x12);
                                            *((slotw as *mut f32).byte_add(0x64)) = rr2 * rr2;
                                            *((slotw as *mut u32).byte_add(0x68)) =
                                                *((e as *const u8).byte_add(0x16)) as u32;
                                            *((slotw as *mut u32).byte_add(0x6c)) =
                                                *((e as *const u8).byte_add(0x17)) as u32;
                                            // Spawn only inside the annulus.
                                            let dx = ra - px;
                                            let dy = rb - py;
                                            let dz = 20.0 - pz;
                                            let d2 = dy * dy + dx * dx;
                                            let d2 = d2 + dz * dz;
                                            let r1sq = rdf(slotw, 0x60);
                                            if d2 > r1sq {
                                                let r2sq = rdf(slotw, 0x64);
                                                if r2sq > d2 {
                                                    let name = rdu(e, 0x1e);
                                                    if name != *global::<u32>(0x129243c) {
                                                        callee_thiscall!(
                                                            3,
                                                            u32,
                                                            relocated(0x115d9a0),
                                                            name
                                                        );
                                                    }
                                                    // Spawn descriptor: zero body,
                                                    // slot pointer and marker
                                                    // words, as the original
                                                    // lays them in its frame.
                                                    let mut desc = [0u32; 18];
                                                    desc[2] = slotw.wrapping_add(0x50);
                                                    desc[11] = 0xffff_ffff;
                                                    desc[12] = 0x46bab800;
                                                    desc[13] = 0xffff_ffff;
                                                    desc[14] = 0x0020_00ff;
                                                    callee_thiscall!(
                                                        4,
                                                        u32,
                                                        relocated(0x1231800),
                                                        rdu(e, 0x1a),
                                                        this.wrapping_add(0x74).wrapping_add(s48),
                                                        &mut desc as *mut u32 as u32,
                                                        0xffff_ffff,
                                                        0,
                                                        0
                                                    );
                                                    ((e as *mut u32).byte_add(0x22))
                                                        .write_unaligned(time);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        idx = idx.wrapping_add(1);
                    }
                }
            }
            outer = outer.wrapping_add(1);
        }
        audio_gain_stage(this);
    }
});
