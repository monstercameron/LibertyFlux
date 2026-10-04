// original: 0x00d3cdc0 CAR_METAL
/// Synthesise one metal-impact audio event for a source object.
///
/// Reads the source kind and handles, advances the shared random generator,
/// draws a repeat count from the kind table, then per repeat builds a
/// spatial sample (position, trig-shaped spread, helper modulation) and
/// submits it through the audio graph. Returns 0 when the count is zero,
/// otherwise the exit code of the handle checks.
export!(thiscall, rw_rb44_f2(this: *mut u8, a0: u32) -> u32 {
    unsafe {
        use lf_k2_rt::global;
        const M23: u32 = 0x7FFFFF;
        let s23 = f32::from_bits(*global::<u32>(0xFE864C));
        let c025 = f32::from_bits(*global::<u32>(0xFE87E4));
        let c15 = f32::from_bits(*global::<u32>(0xFE8B20));
        let c075 = f32::from_bits(*global::<u32>(0xFE888C));
        let c20 = f32::from_bits(*global::<u32>(0xFE8B38));
        let c1 = f32::from_bits(*global::<u32>(0xFE88E8));
        let c13 = f32::from_bits(*global::<u32>(0xFE8B14));
        let c12 = f32::from_bits(*global::<u32>(0xFE8B0C));
        let c05 = f32::from_bits(*global::<u32>(0xFE8830));
        let c005 = f32::from_bits(*global::<u32>(0xFE876C));
        let c5 = f32::from_bits(*global::<u32>(0xFE8AD8));
        let c2pi = f32::from_bits(*global::<u32>(0xFE8AEC));
        let cpi = f32::from_bits(*global::<u32>(0xFE8AA0));
        let c2 = f32::from_bits(*global::<u32>(0xFE8A24));
        let c10 = f32::from_bits(*global::<u32>(0xFE8B08));
        let absmask = *global::<u32>(0xFE8F80);
        let rng_lo = global::<u32>(0x11101A0);
        let rng_hi = global::<u32>(0x11101A4);
        let kind = *((this.add(0x40)) as *const u32);
        let h48 = *((this.add(0x48)) as *const u32);
        let h4c = *((this.add(0x4C)) as *const u32);
        let mut flag = 1u32;
        if h4c != 0 && (kind == 4 || kind == 12 || kind == 5) {
            let idx = *((h4c + 0x2E) as *const i16) as i32;
            let ent = *((relocated(0x1295CD8) + (idx as u32).wrapping_mul(4)) as *const u32);
            let v = *(((r32(ent, 0xCC)) + 0xB4) as *const u32) as i32;
            if v > -1 {
                let _: u32 = callee_thiscall!(1, u32, h4c, v as u32);
                let a = *rng_lo as u64 * 0x5CDCFAA7u64 + *rng_hi as u64;
                let oa = (a as u32) & M23;
                let b = (a as u32) as u64 * 0x5CDCFAA7u64 + (a >> 32);
                let ob = (b as u32) & M23;
                *rng_lo = b as u32;
                *rng_hi = (b >> 32) as u32;
                let f0 = (oa as f32) * s23 * c15 + c20;
                let f1 = (ob as f32) * s23 * c025 + c075;
                let o = *global::<u32>(0x18B8968);
                let vt = *(o as *const u32);
                let t10 = *((vt as *const u8).add(0x10) as *const u32);
                let e10: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(t10 as usize);
                let r1 = e10(o, relocated(0xEE3A54), h48, 0, 0, 2, f0.to_bits(), f1.to_bits(), 0);
                let t20 = *((vt as *const u8).add(0x20) as *const u32);
                let e20: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(t20 as usize);
                let r2 = e20(o, r1);
                let avec = [h48, 0u32];
                let _: u32 = callee_thiscall!(4, u32, relocated(0x12E2420), 0, 0x30, avec.as_ptr() as u32, r2);
            }
            flag = 0;
        }
        let emit = if flag != 0 { 0x8Eu32 } else { 0x86u32 };
        let (tlo, thi) = step_rng(*rng_lo, *rng_hi);
        *rng_lo = tlo;
        *rng_hi = thi;
        let tp = relocated(0x12F8480).wrapping_add(kind.wrapping_mul(0x70));
        let w0 = *(tp as *const u32);
        let w1 = *((tp + 4) as *const u32);
        let div = w1.wrapping_sub(w0).wrapping_add(1) as i32;
        let n = w0 as i32 + (((tlo & 0x7FFFFFFF) as i32) % div);
        if n == 0 {
            return 0;
        }
        let (u_lo, u_hi) = step_rng(tlo, thi);
        let o1 = u_lo & M23;
        let (v_lo, v_hi) = step_rng(u_lo, u_hi);
        let o2 = v_lo & M23;
        *rng_lo = v_lo;
        *rng_hi = v_hi;
        let mut e116 = (o1 as f32) * s23 * c13 + c12;
        let mut e108 = ((o2 as f32) * s23 + c1) * c05;
        let exit_eax: u32;
        if n > 0 {
            let nn = n as u32;
            let mut i = 0u32;
            loop {
                let mut e148 = f32::from_bits(*((this.add(0x50)) as *const u32));
                let mut e144 = f32::from_bits(*((this.add(0x54)) as *const u32));
                let mut e140 = f32::from_bits(*((this.add(0x58)) as *const u32)) + c005;
                let mut e128 = 0.0f32;
                let mut e136b = 0u32;
                if i == 0 {
                    let (nlo, nhi) = step_rng(*rng_lo, *rng_hi);
                    *rng_lo = nlo;
                    let o3 = nlo & M23;
                    e108 = c1;
                    e116 = (o3 as f32) * s23 * c5 + c20;
                    *rng_hi = nhi;
                } else {
                    let f0 = f32::from_bits(*((tp + 8) as *const u32));
                    let f1 = f32::from_bits(*((tp + 12) as *const u32));
                    let (nlo, nhi) = step_rng(*rng_lo, *rng_hi);
                    *rng_lo = nlo;
                    let o4 = nlo & M23;
                    e128 = f0 + (f1 - f0) * ((o4 as f32) * s23);
                    *rng_hi = nhi;
                }
                let r5: u32 = callee_thiscall!(5, u32, this as u32);
                if r5 == 4 {
                    let ang = (i as f32) / (nn as f32) * c2pi - cpi;
                    let co: u32 = callee_cdecl!(6, u32, ang.to_bits());
                    e148 = f32::from_bits(co) * e128 + e148;
                    let si: u32 = callee_cdecl!(7, u32, ang.to_bits());
                    e144 = f32::from_bits(si) * e128 + e144;
                } else if kind == 4 && (a0 & 0xFF) == 0 && h4c != 0 {
                    let (slo, shi) = step_rng(*rng_lo, *rng_hi);
                    *rng_lo = slo;
                    *rng_hi = shi;
                    let ang2 = ((slo & M23) as f32) * s23 * c2pi - cpi;
                    let hb = *((h4c + 0x20) as *const u32);
                    let base = if hb != 0 { hb + 0x30 } else { h4c + 0x10 };
                    e148 = f32::from_bits(*(base as *const u32));
                    e144 = f32::from_bits(*((base + 4) as *const u32));
                    let mut z = f32::from_bits(*((base + 8) as *const u32));
                    e136b = *((base + 12) as *const u32);
                    if (i & 1) == 0 {
                        z = z + c1;
                    }
                    e140 = z;
                    let vt = *(h4c as *const u32);
                    let tgt = *((vt as *const u8).add(0x58) as *const u32);
                    let smp: extern "thiscall" fn(u32) -> f32 =
                        core::mem::transmute(tgt as usize);
                    let s1 = smp(h4c);
                    let m1 = s1 * e128;
                    let c: u32 = callee_cdecl!(6, u32, ang2.to_bits());
                    e148 = m1 * f32::from_bits(c) + e148;
                    let s2 = smp(h4c);
                    let m2 = s2 * e128;
                    let s: u32 = callee_cdecl!(7, u32, ang2.to_bits());
                    e144 = m2 * f32::from_bits(s) + e144;
                } else if (i as i32) > 0 {
                    let a = *rng_lo as u64 * 0x5CDCFAA7u64 + *rng_hi as u64;
                    let b = (a as u32) as u64 * 0x5CDCFAA7u64 + (a >> 32);
                    *rng_lo = b as u32;
                    *rng_hi = (b >> 32) as u32;
                    let o5 = (a as u32) & M23;
                    let o6 = (b as u32) & M23;
                    let ang3 = (o5 as f32) * s23 * c2pi - cpi;
                    let f0 = f32::from_bits(*((tp + 8) as *const u32));
                    let f1 = f32::from_bits(*((tp + 12) as *const u32));
                    let w = f0 + (f1 - f0) * ((o6 as f32) * s23);
                    let c: u32 = callee_cdecl!(6, u32, ang3.to_bits());
                    e148 = f32::from_bits(c) * w + e148;
                    let s: u32 = callee_cdecl!(7, u32, ang3.to_bits());
                    e144 = f32::from_bits(s) * w + e144;
                }
                let s0 = [e148.to_bits(), e144.to_bits(), e140.to_bits(), e136b, 0u32, e128.to_bits()];
                let vx = *global::<u32>(0x1B4B320);
                let vy = *global::<u32>(0x1B4B324);
                let vz = *global::<u32>(0x1B4B328);
                let mut s1 = [0u32; 21];
                s1[4] = vx; s1[5] = vy; s1[6] = vz;
                s1[8] = vx; s1[9] = vy; s1[10] = vz;
                s1[12] = vx; s1[13] = vy; s1[14] = vz;
                s1[19] = 0xFFFF;
                let adj = e140 - c2;
                let r9: u32 = callee_cdecl!(9, u32, s0.as_ptr() as u32, adj.to_bits(), 0, s1.as_ptr() as u32, emit, 4);
                if (r9 & 0xFF) != 0 {
                    let s1o: u32 = callee_cdecl!(10, u32, 0);
                    let mut skip = false;
                    if s1o != 0 {
                        let vt = *(s1o as *const u32);
                        let tgt = *((vt as *const u8).add(0xA0) as *const u32);
                        let q: extern "thiscall" fn(u32) -> u32 =
                            core::mem::transmute(tgt as usize);
                        let q1 = q(s1o);
                        if q1 != 0 {
                            let q2 = q(s1o);
                            let g: u32 = callee_thiscall!(12, u32, q2);
                            let arr = *(((g) + 0xD4) as *const u32);
                            let ent = *(arr as *const u32);
                            if *((ent + 0xC) as *const u8) != 0 {
                                skip = true;
                            }
                        }
                    }
                    if !skip {
                        let diff = f32::from_bits(vz) - f32::from_bits(*((this.add(0x58)) as *const u32));
                        let ad = f32::from_bits(diff.to_bits() & absmask);
                        if ad < c10 {
                            let s2o: u32 = callee_cdecl!(10, u32, 0);
                            let k = (*(((s2o) + 0x28) as *const u32) >> 6) & 0xF;
                            let w2 = if r5 == 4 { 2u32 } else { 1u32 };
                            if k == 2 || k == 4 {
                                let _: u32 = callee_thiscall!(14, u32, relocated(0x12E2420), s2o,
                                    s1.as_ptr().wrapping_add(4) as u32, s1.as_ptr().wrapping_add(8) as u32,
                                    0, h48, 0, 0, 2, e116.to_bits(), e108.to_bits(), 0);
                            } else {
                                let _: u32 = callee_thiscall!(13, u32, relocated(0x12E2420),
                                    s1.as_ptr().wrapping_add(4) as u32, w2, h48, 0, 2, e116.to_bits(), e108.to_bits(), 0);
                            }
                        }
                    }
                }
                i += 1;
                if i >= nn {
                    break;
                }
            }
            exit_eax = nn;
        } else {
            exit_eax = 0;
        }
        if h48 == 0 {
            return exit_eax;
        }
        let bits = *((h48 + 0x28) as *const u32) & 0x3C0;
        if bits != 0xC0 {
            return bits;
        }
        if *((h48 + 0x218) as *const u8) != 0 {
            return bits;
        }
        if *((h48 + 0x219) as *const u8) == 0 {
            return bits;
        }
        callee_cdecl!(15, u32, 0x1C1, 0x3F800000)
    }
});
