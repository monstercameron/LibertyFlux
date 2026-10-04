// original: 0x00B5CCC0 unnamed
/// Shape one positional audio update: resolve range parameters through
/// callee 1, run the source-to-listener difference triple through two gated
/// scaling stages (callees 2-4) and a cross-product cascade (callees 4),
/// then accumulate two table-driven corrections into the output triple
/// `out`. Returns the table index (low 8 bits of a chopped conversion).
///
/// Cond: `src` (or null) points to the source object, `a1` to 3 readable
/// floats, `out` to 3 readable/writable floats.
export!(stdcall, rw_b73_f3(s0: u32, s1: u32, s2: u32) -> u32 {
    unsafe {
        // Callee 1 fills three scalar slots (zeroed: snapshotted pre-write).
        let mut p1 = [0u32; 1];
        let mut p2 = [0u32; 1];
        let mut p3 = [0u32; 1];
        let _: u32 = callee_stdcall!(
            1,
            u32,
            s0,
            p3.as_mut_ptr() as u32,
            p2.as_mut_ptr() as u32,
            p1.as_mut_ptr() as u32
        );
        let fa = s1 as *const f32;
        let fo = s2 as *mut f32;
        let dx = *fo.offset(0) - *fa.offset(0);
        let dy = *fo.offset(1) - *fa.offset(1);
        let dz = *fo.offset(2) - *fa.offset(2);
        let len2 = addss_exact(
            addss_exact(mulss_exact(dx, dx), mulss_exact(dy, dy)),
            mulss_exact(dz, dz),
        );
        let slen = len2.sqrt();
        let p3v = f32::from_bits(p3[0]);
        let mut s14 = mulss_exact(mulss_exact(slen, *global::<f32>(0xFE8734)), p3v);
        if s0 != 0
            && (*(s0.wrapping_add(0x28) as *const u32) & 0x3C0) == 0xC0
            && *(s0.wrapping_add(0x219) as *const u8) != 0
        {
            let a2: u32 = callee_thiscall!(2, u32, s0);
            let flag = u32::from(
                s0.wrapping_add(0x2B0) != 0
                    && *(s0.wrapping_add(0x398) as *const u32) != 0,
            );
            if a2 == 0 || *(a2.wrapping_add(0x328D) as *const u8) == 0 {
                let c1: u32 = callee_cdecl!(3, u32,);
                if (c1 & 0xFF) != 0 && flag == 0 {
                    s14 = mulss_exact(s14, *global::<f32>(0xFE879C));
                } else {
                    let c2: u32 = callee_cdecl!(3, u32,);
                    if (c2 & 0xFF) != 0 && flag != 0 {
                        s14 = mulss_exact(s14, *global::<f32>(0xFE8864));
                    }
                }
            } else {
                s14 = mulss_exact(s14, *global::<f32>(0xFE879C));
            }
        }
        // Normalize the triple past a 0.1 length gate.
        let (mut dx, mut dy, mut dz) = (dx, dy, dz);
        if slen > *global::<f32>(0xFE879C) {
            let k = *global::<f32>(0xFE88E8) / slen;
            dx = mulss_exact(dx, k);
            dy = mulss_exact(dy, k);
            dz = mulss_exact(dz, k);
        }
        // Cross-product cascade in the original's exact order.
        let z = 0.0f32;
        let a = mulss_exact(dz, z);
        let b = dy - a;
        let c = a - dx;
        let d = mulss_exact(dy, z);
        let e = mulss_exact(dx, z);
        let f = e - d;
        let g = mulss_exact(b, b);
        let h = mulss_exact(c, c);
        let i = addss_exact(h, g);
        let j = mulss_exact(f, f);
        let k = addss_exact(i, j);
        let r5: f32 = callee_cdecl!(4, f32, k.to_bits());
        let f1 = mulss_exact(f, r5);
        let c1 = mulss_exact(c, r5);
        let b1 = mulss_exact(b, r5);
        let g1 = mulss_exact(c1, dz);
        let h1 = mulss_exact(f1, dy);
        let g2 = g1 - h1;
        let i1 = mulss_exact(f1, dx);
        let j1 = mulss_exact(b1, dz);
        let k1 = mulss_exact(b1, dy);
        let i2 = i1 - j1;
        let l1 = mulss_exact(c1, dx);
        let k2 = k1 - l1;
        let m1 = mulss_exact(g2, g2);
        let n1 = mulss_exact(i2, i2);
        let o1 = addss_exact(n1, m1);
        let p1v = mulss_exact(k2, k2);
        let q1 = addss_exact(o1, p1v);
        let r6: f32 = callee_cdecl!(4, f32, q1.to_bits());
        let g3 = mulss_exact(g2, r6);
        let i3 = mulss_exact(i2, r6);
        let k3 = mulss_exact(k2, r6);
        // Late gate: rescale s14 behind the mode global and source flags.
        if *global::<u32>(0x11D6FD4) == 2 {
            let a3: u32 = callee_cdecl!(3, u32,);
            if (a3 & 0xFF) != 0
                && s0 != 0
                && (*(s0.wrapping_add(0x28) as *const u32) & 0x3C0) == 0xC0
                && *(s0.wrapping_add(0x218) as *const u8) == 0
                && *(s0.wrapping_add(0x219) as *const u8) != 0
            {
                s14 = mulss_exact(*global::<f32>(0x1046A7C), s14);
            }
        }
        let rng: u32 = callee_cdecl!(5, u32,);
        let k0 = mulss_exact(
            mulss_exact(
                mulss_exact(rng as i32 as f32, *global::<f32>(0xFE8684)),
                *global::<f32>(0xFE8AEC),
            ),
            *global::<f32>(0xEB1174),
        );
        let vb2 = mulss_exact(b1, s14);
        let vc2 = mulss_exact(c1, s14);
        let vf2 = mulss_exact(f1, s14);
        let vg2 = mulss_exact(g3, s14);
        let vi2 = mulss_exact(i3, s14);
        let vk2 = mulss_exact(k3, s14);
        let index = ((fistpq_chop(k0) as u64) & 0xFF) as u32;
        let t1 = *(relocated(0x16CF3C8).wrapping_add(index.wrapping_mul(4)) as *const f32);
        let wb = mulss_exact(vb2, t1);
        let wc = mulss_exact(vc2, t1);
        let o0a = addss_exact(wb, *fo.offset(0));
        let wf = mulss_exact(vf2, t1);
        let o1a = addss_exact(wc, *fo.offset(1));
        let o2a = addss_exact(wf, *fo.offset(2));
        *fo.offset(0) = o0a;
        *fo.offset(1) = o1a;
        *fo.offset(2) = o2a;
        let t2 = *(relocated(0x16CF7C8).wrapping_add(index.wrapping_mul(4)) as *const f32);
        let zg = mulss_exact(vg2, t2);
        let zi = mulss_exact(vi2, t2);
        let zk = mulss_exact(vk2, t2);
        *fo.offset(0) = addss_exact(o0a, zg);
        *fo.offset(1) = addss_exact(o1a, zi);
        *fo.offset(2) = addss_exact(o2a, zk);
        index
    }
});
// Exact-operation helpers used above (shared across this lane's rewrites).
fn addss_exact(dest: f32, src: f32) -> f32 {
    let a = dest.to_bits();
    if a & 0x7F800000 == 0x7F800000 && a & 0x007FFFFF != 0 {
        return f32::from_bits(a | 0x00400000);
    }
    let b = src.to_bits();
    if b & 0x7F800000 == 0x7F800000 && b & 0x007FFFFF != 0 {
        return f32::from_bits(b | 0x00400000);
    }
    dest + src
}
fn mulss_exact(dest: f32, src: f32) -> f32 {
    let a = dest.to_bits();
    if a & 0x7F800000 == 0x7F800000 && a & 0x007FFFFF != 0 {
        return f32::from_bits(a | 0x00400000);
    }
    let b = src.to_bits();
    if b & 0x7F800000 == 0x7F800000 && b & 0x007FFFFF != 0 {
        return f32::from_bits(b | 0x00400000);
    }
    dest * src
}
fn fistpq_chop(x: f32) -> i64 {
    if x.is_nan() || x >= 9223372036854775808.0 || x < -9223372036854775808.0 {
        i64::MIN
    } else {
        x as i64
    }
}
