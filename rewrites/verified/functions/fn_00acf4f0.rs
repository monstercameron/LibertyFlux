// original: 0x00acf4f0 audio_batch_voice_render
/// Audio batch voice render into the matrix array (original 0xACF4F0).
///
/// `this` is the voice, `a0` the matrix block, `a1` the index array, `a2`
/// the entry table, `a3` its count. A negative voice index ends the call.
/// Otherwise the voice row is normalized (or zeroed when its length is
/// zero), the level shaped against the global band and flags, and the
/// projected point accumulated; flagged voices then run a clamped
/// projection update, table-search voices resolve an entry and run a
/// ratio update. The record is rotated twice by the voice angles, an
/// optional second record is stamped from it, the point is stored, and
/// flag-gated scale and ratio passes finish. Returns the table count on
/// some paths; the other exit values are incidental (negative index or
/// shifted flag words) and are reproduced exactly.
export!(thiscall, rw_b45_f4f0(this: *mut u8, a0: *const u8, a1: *const u32, a2: *const u8, a3: u32) -> u32 {
    unsafe {
        let c1 = *global::<f32>(0xFE88E8);
        let c0 = *global::<f32>(0xFE8C58);
        let ck = *global::<f32>(0xFE86B4);
        let cbig = *global::<f32>(0xFE8D94);
        let sel = *(this as *const u32);
        let idx = *(a1.add(sel as usize) as *const i32);
        if idx < 0 {
            return idx as u32;
        }
        let mbase = *(a0.add(0xC) as *const u32);
        let esi = (mbase + (idx as u32).wrapping_mul(80)) as *mut u8;
        *(esi.add(0x00) as *mut u32) = 0x3F800000;
        *(esi.add(0x04) as *mut u32) = 0;
        *(esi.add(0x08) as *mut u32) = 0;
        *(esi.add(0x10) as *mut u32) = 0;
        *(esi.add(0x14) as *mut u32) = 0x3F800000;
        *(esi.add(0x18) as *mut u32) = 0;
        *(esi.add(0x20) as *mut u32) = 0;
        *(esi.add(0x24) as *mut u32) = 0;
        *(esi.add(0x28) as *mut u32) = 0x3F800000;
        let t30 = *(this.add(0x30) as *const f32);
        let t34 = *(this.add(0x34) as *const f32);
        let t38 = *(this.add(0x38) as *const f32);
        let t40 = *(this.add(0x40) as *const f32);
        let t44 = *(this.add(0x44) as *const f32);
        let t48 = *(this.add(0x48) as *const f32);
        *(esi.add(0x28) as *mut f32) = t48;
        *(esi.add(0x24) as *mut f32) = t44;
        *(esi.add(0x20) as *mut f32) = t40;
        *(esi.add(0x2C) as *mut u32) = *(this.add(0x4C) as *const u32);
        *(esi.add(0x08) as *mut f32) = t38;
        *(esi.add(0x00) as *mut f32) = t30;
        *(esi.add(0x04) as *mut f32) = t34;
        *(esi.add(0x0C) as *mut u32) = *(this.add(0x3C) as *const u32);
        *(esi.add(0x10) as *mut f32) = t44 * t38 - t48 * t34;
        *(esi.add(0x14) as *mut f32) = t30 * t48 - t38 * t40;
        *(esi.add(0x18) as *mut f32) = t34 * t40 - t30 * t44;
        let mut len2 = t30 * t30;
        len2 += t34 * t34;
        len2 += t38 * t38;
        // ucomiss + lahf/test/jp: divide unless the length is exactly zero.
        let n = if len2 == 0.0 { 0.0 } else { c1 / len2.sqrt() };
        *(esi.add(0x00) as *mut f32) = t30 * n;
        *(esi.add(0x04) as *mut f32) = t34 * n;
        *(esi.add(0x08) as *mut f32) = t38 * n;
        let v160 = *(this.add(0x160) as *const f32);
        let mut lvl = *(this.add(0x78) as *const f32);
        if c0 > v160 {
            let base = if !(0.0 >= v160) {
                let mut w = *(this.add(0x70) as *const f32)
                    / *(this.add(0x20) as *const f32);
                w = if w > c1 { c1 } else { w };
                let k = v160 * ck;
                let mut u = (c1 - k) * w;
                u = if 0.0 > u { 0.0 } else { u };
                (*(this.add(8) as *const f32) - *(this.add(0xC) as *const f32)) * u
            } else {
                *(this.add(8) as *const f32) - *(this.add(0xC) as *const f32)
            };
            lvl -= base;
            lvl = if 0.0 > lvl { 0.0 } else { lvl };
        }
        let flags = *(this.add(0x164) as *const u32);
        if flags & 0x0C000000 != 0 {
            let c = if (flags >> 27) & 1 != 0 {
                *global::<f32>(0xFE876C)
            } else {
                *global::<f32>(0xFE8748)
            };
            lvl -= c;
            lvl = if 0.0 > lvl { 0.0 } else { lvl };
        }
        let grow = *(this.add(8) as *const f32) + lvl;
        let mut px = *(esi.add(0x20) as *const f32) * grow + *(this.add(0x60) as *const f32);
        let mut py = *(esi.add(0x24) as *const f32) * grow + *(this.add(0x64) as *const f32);
        let mut pz = *(esi.add(0x28) as *const f32) * grow + *(this.add(0x68) as *const f32);
        if flags & 0x10000 != 0 {
            lvl -= *(this.add(0x20) as *const f32);
            let mut lim = c1;
            if cbig > lvl {
                lim = cbig;
            } else if !(lvl > c1) {
                lim = lvl;
            }
            lim *= *global::<f32>(0xFE888C);
            let selc = if !(lvl >= 0.0) {
                *global::<f32>(0xFE8830)
            } else {
                *global::<f32>(0xFE87E8)
            };
            let q = selc * selc - lvl * lvl;
            let mut dd = selc;
            if q > 0.0 {
                dd = selc - q.sqrt();
            }
            let mut sp38 = dd;
            if (flags >> 11) & 1 == 0 {
                sp38 = dd * cbig;
                lim *= cbig;
            }
            let _: u32 = callee_thiscall!(1, u32, esi as u32, lim.to_bits());
            px = *global::<f32>(0x110DB00) * sp38 + px;
            py = *global::<f32>(0x110DB04) * sp38 + py;
            pz = *global::<f32>(0x110DB08) * sp38 + pz;
        } else if flags & 0x20000 != 0 && (a3 as i32) > 0 {
            let tag = *((relocated(0x103F2F4) + sel.wrapping_mul(4)) as *const u32);
            let mut entry = 0u32;
            let mut k = 0u32;
            while (k as i32) < (a3 as i32) {
                if *(a2.add(k.wrapping_mul(0x170) as usize) as *const u32) == tag {
                    entry = (a2 as u32).wrapping_add(k.wrapping_mul(0x170));
                    break;
                }
                k += 1;
            }
            if entry != 0 {
                let e = entry as *const u8;
                let e160 = *(e.add(0x160) as *const f32);
                let mut l2 = *(e.add(0x78) as *const f32);
                if c0 > e160 {
                    let mut u = c1 - e160 * ck;
                    u = if 0.0 > u { 0.0 } else { u };
                    l2 -= (*(e.add(8) as *const f32) - *(e.add(0xC) as *const f32)) * u;
                    l2 = if 0.0 > l2 { 0.0 } else { l2 };
                }
                let w = px * *global::<f32>(0xFE8DB0);
                lvl -= l2;
                lvl /= w;
                let mut lim = c1;
                if cbig > lvl {
                    lvl = cbig;
                    lim = cbig;
                } else if !(lvl > c1) {
                    lim = lvl;
                }
                let ans2 = callee_cdecl!(2, u32, lim.to_bits());
                let _: u32 = callee_thiscall!(1, u32, esi as u32, ans2);
                let g = f32::from_bits(callee_cdecl!(3, u32, ans2));
                px = g * px;
            }
        }
        let a13c = *(this.add(0x13C) as *const u32);
        let c2 = f32::from_bits(callee_cdecl!(3, u32, a13c));
        let s2 = f32::from_bits(callee_cdecl!(4, u32, a13c));
        let r0 = *(esi.add(0x00) as *const f32);
        let r4 = *(esi.add(0x04) as *const f32);
        let r8 = *(esi.add(0x08) as *const f32);
        let r10 = *(esi.add(0x10) as *const f32);
        let r14 = *(esi.add(0x14) as *const f32);
        let r18 = *(esi.add(0x18) as *const f32);
        *(esi.add(0x10) as *mut f32) = c2 * r10 - r0 * s2;
        *(esi.add(0x14) as *mut f32) = r14 * c2 - r4 * s2;
        *(esi.add(0x18) as *mut f32) = r18 * c2 - r8 * s2;
        *(esi.add(0x00) as *mut f32) = s2 * r10 + r0 * c2;
        *(esi.add(0x04) as *mut f32) = r14 * s2 + r4 * c2;
        *(esi.add(0x08) as *mut f32) = r18 * s2 + r8 * c2;
        let ib = *((relocated(0x103F2DC) + sel.wrapping_mul(4)) as *const i32);
        if ib >= 0 {
            let idx2 = *(a1.add(ib as usize) as *const i32);
            if idx2 >= 0 {
                let ecx = (mbase + (idx2 as u32).wrapping_mul(80)) as *mut u8;
                *(ecx.add(0x00) as *mut u32) = 0x3F800000;
                *(ecx.add(0x04) as *mut u32) = 0;
                *(ecx.add(0x08) as *mut u32) = 0;
                *(ecx.add(0x10) as *mut u32) = 0;
                *(ecx.add(0x14) as *mut u32) = 0x3F800000;
                *(ecx.add(0x18) as *mut u32) = 0;
                *(ecx.add(0x20) as *mut u32) = 0;
                *(ecx.add(0x24) as *mut u32) = 0;
                *(ecx.add(0x28) as *mut u32) = 0x3F800000;
                *(ecx.add(0x00) as *mut u32) = *(esi.add(0x00) as *const u32);
                *(ecx.add(0x04) as *mut u32) = *(esi.add(0x04) as *const u32);
                *(ecx.add(0x08) as *mut u32) = *(esi.add(0x08) as *const u32);
                *(ecx.add(0x10) as *mut u32) = *(esi.add(0x10) as *const u32);
                *(ecx.add(0x14) as *mut u32) = *(esi.add(0x14) as *const u32);
                *(ecx.add(0x18) as *mut u32) = *(esi.add(0x18) as *const u32);
                *(ecx.add(0x20) as *mut u32) = *(esi.add(0x20) as *const u32);
                *(ecx.add(0x24) as *mut u32) = *(esi.add(0x24) as *const u32);
                *(ecx.add(0x28) as *mut u32) = *(esi.add(0x28) as *const u32);
                *(ecx.add(0x30) as *mut f32) = px;
                *(ecx.add(0x34) as *mut f32) = py;
                *(ecx.add(0x38) as *mut f32) = pz;
                // The original spills an uninitialized frame slot here; the
                // contract defines uninitialized stack as zero.
                *(ecx.add(0x3C) as *mut f32) = 0.0;
            }
        }
        let a7c = *(this.add(0x7C) as *const u32);
        let c3 = f32::from_bits(callee_cdecl!(3, u32, a7c));
        let s3 = f32::from_bits(callee_cdecl!(4, u32, a7c));
        let q10 = *(esi.add(0x10) as *const f32);
        let q14 = *(esi.add(0x14) as *const f32);
        let q18 = *(esi.add(0x18) as *const f32);
        let q20 = *(esi.add(0x20) as *const f32);
        let q24 = *(esi.add(0x24) as *const f32);
        let q28 = *(esi.add(0x28) as *const f32);
        let n10 = s3 * q20 + q10 * c3;
        let n14 = q24 * s3 + q14 * c3;
        let n18 = q28 * s3 + q18 * c3;
        *(esi.add(0x20) as *mut f32) = c3 * q20 - s3 * q10;
        *(esi.add(0x24) as *mut f32) = q24 * c3 - q14 * s3;
        *(esi.add(0x28) as *mut f32) = q28 * c3 - q18 * s3;
        *(esi.add(0x10) as *mut f32) = n10;
        *(esi.add(0x14) as *mut f32) = n14;
        *(esi.add(0x18) as *mut f32) = n18;
        *(esi.add(0x30) as *mut f32) = px;
        *(esi.add(0x34) as *mut f32) = py;
        *(esi.add(0x38) as *mut f32) = pz;
        *(esi.add(0x3C) as *mut f32) = 0.0;
        if (flags >> 22) & 1 == 0 {
            return flags >> 22;
        }
        if (flags >> 11) & 1 == 0 {
            *(esi.add(0x00) as *mut f32) = *(esi.add(0x00) as *const f32) * cbig;
            *(esi.add(0x04) as *mut f32) = *(esi.add(0x04) as *const f32) * cbig;
            *(esi.add(0x08) as *mut f32) = *(esi.add(0x08) as *const f32) * cbig;
            *(esi.add(0x20) as *mut f32) = *(esi.add(0x20) as *const f32) * cbig;
            *(esi.add(0x24) as *mut f32) = *(esi.add(0x24) as *const f32) * cbig;
            *(esi.add(0x28) as *mut f32) = *(esi.add(0x28) as *const f32) * cbig;
        }
        if (flags >> 23) & 1 == 0 {
            return flags >> 23;
        }
        let r1 = *(this.add(0x10) as *const f32) / *(a2.add(0x10) as *const f32);
        *(esi.add(0x00) as *mut f32) = *(esi.add(0x00) as *const f32) * r1;
        *(esi.add(0x04) as *mut f32) = *(esi.add(0x04) as *const f32) * r1;
        *(esi.add(0x08) as *mut f32) = *(esi.add(0x08) as *const f32) * r1;
        let r2 = *(this.add(8) as *const f32) / *(a2.add(8) as *const f32);
        *(esi.add(0x10) as *mut f32) = *(esi.add(0x10) as *const f32) * r2;
        *(esi.add(0x14) as *mut f32) = *(esi.add(0x14) as *const f32) * r2;
        *(esi.add(0x18) as *mut f32) = *(esi.add(0x18) as *const f32) * r2;
        *(esi.add(0x20) as *mut f32) = *(esi.add(0x20) as *const f32) * r2;
        *(esi.add(0x24) as *mut f32) = *(esi.add(0x24) as *const f32) * r2;
        *(esi.add(0x28) as *mut f32) = *(esi.add(0x28) as *const f32) * r2;
        a2 as u32
    }
});
