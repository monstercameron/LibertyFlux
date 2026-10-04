// original: 0x00DBCE60 path_smooth (proposed)
/// Fit and smooth a path polyline through the recorded points.
///
/// Takes the object's stored point list (`count` at offset `0x1c`), publishes
/// the first point, then walks the list: each step derives the segment
/// direction, asks the segment fitter to project the next point (accepted
/// points are published with a zero tag, rejected ones publish the raw
/// sample), and stops after the last sample or after 30 accepted points. The
/// tail point is published from the sample past the end, the object count is
/// reset, and a second pass drops points whose turn is sharper than the angle
/// threshold, keeping significant ones. Returns the last tag read, or the
/// accepted count when the smoothing pass is skipped.
///
/// The threshold comes from a helper called with a constant angle in XMM0; a
/// Rust rewrite cannot pass or read that register, so the helper's exact
/// answer for that constant (measured once with the checker worker itself,
/// `0x3FEFE0D3B3DD3F14`) is used directly after making the same call. Zero
/// length normalizes to a zero direction, and `NaN` lengths take the
/// reciprocal-square-root path, matching the original's flag tests.
lf_checker_rt::export!(thiscall, rw_dbce60(this_ptr: u32, obj: u32, _b: u32, _c: u32, _d: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x017A_6900;
        const PTS: u32 = 0x017A_6700;
        const TAG: u32 = 0x017A_6908;
        const TAGB: u32 = 0x017A_6988;
        const THRESH: u32 = 0x017A_6A08;
        const COS_5DEG: f64 = f64::from_bits(0x3FEF_E0D3_B3DD_3F14);
        let g32 = |va: u32| (global::<u32>(va)).read();
        let g32w = |va: u32, v: u32| (global::<u32>(va)).write(v);
        let g8 = |va: u32| (global::<u8>(va)).read();
        let g8w = |va: u32, v: u8| (global::<u8>(va)).write(v);
        let g16w = |va: u32, v: u16| (global::<u16>(va)).write(v);
        let h32 = |p: u32| ((p as *const u32)).read();
        let h32w = |p: u32, v: u32| ((p as *mut u32)).write(v);
        let hf = |p: u32| ((p as *const f32)).read();
        let hfw = |p: u32, v: f32| ((p as *mut f32)).write(v);
        let gf = |va: u32| (global::<f32>(va)).read();
        let gfw = |va: u32, v: f32| (global::<f32>(va)).write(v);
        let inv_len = |len2: f32| if len2 == 0.0 { 0.0 } else { 1.0 / len2.sqrt() };

        let mut flag = g32(FLAG);
        if flag & 1 == 0 {
            flag |= 1;
            g32w(FLAG, flag);
        }
        gfw(PTS, hf(obj.wrapping_add(0x90)));
        gfw(PTS.wrapping_add(4), hf(obj.wrapping_add(0x94)));
        gfw(PTS.wrapping_add(8), hf(obj.wrapping_add(0x98)));
        gfw(PTS.wrapping_add(12), hf(obj.wrapping_add(0x9C)));
        g32w(TAG, h32(obj.wrapping_add(0x1E4)));
        g32w(TAGB, h32(obj.wrapping_add(0x1A0)));
        let n = h32(obj.wrapping_add(0x1C));
        let mut edi: u32 = 1;
        // First point, reloaded from the published arrays except when the
        // main pass is skipped (single sample), where the object's own
        // values are written back.
        let (mut p0, mut p1, mut p2);
        if n.wrapping_sub(1) == 0 {
            p0 = hf(obj.wrapping_add(0x90));
            p1 = hf(obj.wrapping_add(0x94));
            p2 = hf(obj.wrapping_add(0x98));
        } else {
            let mut cptr = obj.wrapping_add(0x1E8);
            let mut dptr = obj.wrapping_add(0x94);
            let mut k: u32 = 0;
            let mut w8: u32 = 0;
            let mut out = [0u32; 4];
            let mut dir = [0f32; 3];
            let mut done = false;
            while !done {
                let v0 = h32(cptr.wrapping_sub(4));
                let v1 = h32(cptr);
                let dx = hf(dptr.wrapping_add(0xC)) - hf(dptr.wrapping_sub(4));
                let dy = hf(dptr.wrapping_add(0x10)) - hf(dptr);
                let dz = hf(dptr.wrapping_add(0x14)) - hf(dptr.wrapping_add(4));
                let pa = dptr.wrapping_add(0xC);
                let pb = dptr.wrapping_sub(4);
                let len2 = dy * dy + dx * dx + dz * dz;
                let inv = inv_len(len2);
                dir[0] = dx * inv;
                dir[1] = dy * inv;
                dir[2] = dz * inv;
                callee_thiscall!(1, u32, this_ptr);
                let mut al = callee_stdcall!(2, u32, pb, pa, v1, v0,
                    dir.as_mut_ptr() as u32, &mut w8 as *mut u32 as u32,
                    out.as_mut_ptr() as u32);
                if (al as u8) != 0 {
                    let mut e4 = edi.wrapping_mul(4);
                    loop {
                        let echo = w8;
                        let o0 = f32::from_bits(out[0]);
                        let o1 = f32::from_bits(out[1]);
                        let o2 = f32::from_bits(out[2]);
                        let o3 = f32::from_bits(out[3]);
                        g32w(TAG.wrapping_add(e4), echo);
                        g16w(TAGB.wrapping_add(e4), 0);
                        g8w(TAGB.wrapping_add(e4).wrapping_add(2), 0);
                        let base = PTS.wrapping_add(edi.wrapping_mul(16));
                        gfw(base.wrapping_add(8).wrapping_sub(8), o0);
                        gfw(base.wrapping_add(8).wrapping_sub(4), o1);
                        gfw(base.wrapping_add(8), o2);
                        gfw(base.wrapping_add(12), o3);
                        edi = edi.wrapping_add(1);
                        e4 = edi.wrapping_mul(4);
                        if edi == 0x1F {
                            done = true;
                            break;
                        }
                        hfw(pb, o0);
                        hfw(dptr, o1);
                        hfw(dptr.wrapping_add(8), o3);
                        hfw(dptr.wrapping_add(4), o2);
                        let ddx = hf(dptr.wrapping_add(0xC)) - o0;
                        let ddy = hf(dptr.wrapping_add(0x10)) - o1;
                        let ddz = hf(dptr.wrapping_add(0x14)) - o2;
                        let len_b = ddy * ddy + ddx * ddx + ddz * ddz;
                        let inv_b = inv_len(len_b);
                        dir[0] = ddx * inv_b;
                        dir[1] = ddy * inv_b;
                        dir[2] = ddz * inv_b;
                        al = callee_stdcall!(2, u32, pb, dptr.wrapping_add(0xC), v1, echo,
                            dir.as_mut_ptr() as u32, &mut w8 as *mut u32 as u32,
                            out.as_mut_ptr() as u32);
                        if (al as u8) == 0 {
                            break;
                        }
                    }
                    if done {
                        break;
                    }
                }
                let pa2 = pa;
                let dp = dptr;
                let base = PTS.wrapping_add(edi.wrapping_mul(16));
                g32w(base, h32(pa2));
                gfw(base.wrapping_add(4), hf(pa2.wrapping_add(4)));
                gfw(base.wrapping_add(8), hf(pa2.wrapping_add(8)));
                g32w(base.wrapping_add(12), h32(dp.wrapping_add(0x18)));
                // The original increments the counter before the tag stores
                // (but after computing the point base above), so the point
                // lands in the old slot and the tags in the next one.
                edi = edi.wrapping_add(1);
                g32w(TAG.wrapping_sub(4).wrapping_add(edi.wrapping_mul(4)), v1);
                g32w(TAGB.wrapping_sub(4).wrapping_add(edi.wrapping_mul(4)), h32(cptr.wrapping_sub(0x44)));
                if edi == 0x1F {
                    break;
                }
                k = k.wrapping_add(1);
                dptr = dptr.wrapping_add(0x10);
                cptr = cptr.wrapping_add(4);
                // The original decrements the count before comparing: the
                // pass runs `n - 1` times, and one sample never enters it.
                if k >= n.wrapping_sub(1) {
                    break;
                }
            }
            p0 = gf(PTS);
            p1 = gf(PTS.wrapping_add(4));
            p2 = gf(PTS.wrapping_add(8));
        }
        hfw(obj.wrapping_add(0x90), p0);
        hfw(obj.wrapping_add(0x94), p1);
        hfw(obj.wrapping_add(0x98), p2);
        hfw(obj.wrapping_add(0x9C), gf(PTS.wrapping_add(12)));
        let nn = h32(obj.wrapping_add(0x1C));
        h32w(obj.wrapping_add(0x1E4), g32(TAG));
        h32w(obj.wrapping_add(0x1A0), g32(TAGB));
        let tail = obj.wrapping_add(nn.wrapping_add(8).wrapping_mul(16));
        let base = PTS.wrapping_add(edi.wrapping_mul(16));
        g32w(base, h32(tail));
        gfw(base.wrapping_add(4), hf(tail.wrapping_add(4)));
        gfw(base.wrapping_add(8), hf(tail.wrapping_add(8)));
        g32w(base.wrapping_add(12), h32(tail.wrapping_add(0xC)));
        g32w(TAG.wrapping_add(edi.wrapping_mul(4)), h32(obj.wrapping_add(nn.wrapping_mul(4)).wrapping_add(0x1E0)));
        g32w(TAGB.wrapping_add(edi.wrapping_mul(4)), h32(obj.wrapping_add(nn.wrapping_mul(4)).wrapping_add(0x19C)));
        h32w(obj.wrapping_add(0x1C), 1);
        let mut eax = flag;
        flag = g32(FLAG);
        if flag & 2 == 0 {
            flag |= 2;
            g32w(FLAG, flag);
            callee_cdecl!(3, u32,);
            gfw(THRESH, COS_5DEG as f32);
        }
        eax = edi;
        if edi <= 1 {
            return eax;
        }
        let cnt = edi;
        let mut esi: u32 = 1;
        let mut dp = PTS.wrapping_add(8);
        loop {
            let curr = gf(dp.wrapping_add(8));
            let prev = gf(dp.wrapping_sub(8));
            let dx0 = gf(dp.wrapping_add(0x10)) - gf(dp);
            let dy0 = curr - prev;
            let dz0 = gf(dp.wrapping_add(0xC)) - gf(dp.wrapping_sub(4));
            let dx1 = gf(dp.wrapping_add(0x18)) - curr;
            let dy1 = gf(dp.wrapping_add(0x1C)) - gf(dp.wrapping_add(0xC));
            let dz1 = gf(dp.wrapping_add(0x20)) - gf(dp.wrapping_add(0x10));
            let len_a = dz0 * dz0 + dy0 * dy0 + dx0 * dx0;
            let inv_a = inv_len(len_a);
            let xxd = dx0 * inv_a;
            let xyd = dy0 * inv_a;
            let xzd = dz0 * inv_a;
            let len_b = dy1 * dy1 + dx1 * dx1 + dz1 * dz1;
            let inv_b = inv_len(len_b);
            let xnd = dx1 * inv_b;
            let ynd = dy1 * inv_b;
            let znd = dz1 * inv_b;
            let dot = ynd * xzd + xnd * xyd + znd * xxd;
            let thresh = gf(THRESH);
            // The original ends the compare with `jbe`, which is taken on
            // unordered too (NaN sets CF and ZF), so NaN dots keep. `<=`
            // would be false there; only the ordered `>` matches.
            let keep = if dot > thresh {
                let w0 = (g32(TAGB.wrapping_sub(4).wrapping_add(esi.wrapping_mul(4))) & 0xFFFF) as u16;
                let w1 = (g32(TAGB.wrapping_add(esi.wrapping_mul(4))) & 0xFFFF) as u16;
                eax = (g32(TAGB.wrapping_add(4).wrapping_add(esi.wrapping_mul(4))) & 0xFFFF) as u32;
                ((w0 == w1) as u32) != eax
            } else {
                true
            };
            if keep {
                let slot = h32(obj.wrapping_add(0x1C));
                let dst = obj.wrapping_add(slot.wrapping_add(9).wrapping_mul(16));
                hfw(dst, curr);
                hfw(dst.wrapping_add(4), gf(dp.wrapping_add(0xC)));
                hfw(dst.wrapping_add(8), gf(dp.wrapping_add(0x10)));
                h32w(dst.wrapping_add(0xC), g32(dp.wrapping_add(0x14)));
                let slot2 = h32(obj.wrapping_add(0x1C));
                eax = g32(TAG.wrapping_add(esi.wrapping_mul(4)));
                h32w(obj.wrapping_add(slot2.wrapping_mul(4)).wrapping_add(0x1E4), eax);
                let slot3 = h32(obj.wrapping_add(0x1C));
                eax = g32(TAGB.wrapping_add(esi.wrapping_mul(4)));
                h32w(obj.wrapping_add(slot3.wrapping_mul(4)).wrapping_add(0x1A0), eax);
                h32w(obj.wrapping_add(0x1C), slot3.wrapping_add(1));
            }
            if h32(obj.wrapping_add(0x1C)) == 0xF {
                break;
            }
            esi = esi.wrapping_add(1);
            dp = dp.wrapping_add(0x10);
            if esi >= cnt {
                break;
            }
        }
        eax
    }
});
