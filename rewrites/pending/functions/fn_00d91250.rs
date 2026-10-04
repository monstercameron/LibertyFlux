// original: 0x00d91250 audio_segment_crossing
/// Classify two segments and interpolate the crossing point.
///
/// Takes two 2D points (`a`, `b`), two 3D points (`c`, `d`) and an output
/// vector `e`. Normalizes both 2D directions, runs twelve matrix transforms
/// that each consume one input quad from the working frame, combines the
/// resulting quads into six dot terms, and signs four of them. When the
/// signs disagree in both pairs, writes the interpolated crossing point
/// (plus a trailing quad lane) to `e` and returns 2; otherwise returns 1.
/// The working frame `fr` mirrors the original stack layout exactly, so the
/// per-call input snapshots verify every transform input bit for bit.
export!(cdecl, rw_00d91250(a: *const f32, b: *const f32, c: *const f32,
                           d: *const f32, e: *mut f32) -> u32 {
    let mut fr = [0.0f32; 26];
    let a0 = unsafe { *a };
    let a1 = unsafe { *a.add(1) };
    let b0 = unsafe { *b };
    let b1 = unsafe { *b.add(1) };
    fr[0x2c / 4] = a1;
    fr[0x38 / 4] = b0;
    fr[0x3c / 4] = b1;
    let dx1 = b0 - a0;
    let dy1 = b1 - a1;
    fr[0x28 / 4] = a0;
    let c0 = unsafe { *c };
    let c1 = unsafe { *c.add(1) };
    let c2 = unsafe { *c.add(2) };
    let d0 = unsafe { *d };
    let d1v = unsafe { *d.add(1) };
    let d2v = unsafe { *d.add(2) };
    fr[0x30 / 4] = c0;
    fr[0x34 / 4] = c1;
    fr[0x40 / 4] = d0;
    fr[0x44 / 4] = d1v;
    // Zero-length directions normalize to zero, not NaN (flag-tested branch).
    let inv1 = {
        let len2 = fadd(dy1 * dy1, dx1 * dx1);
        if len2 == 0.0 { 0.0 } else { 1.0 / len2.sqrt() }
    };
    let dx2 = d0 - c0;
    let dy2 = d1v - c1;
    let nx = fmul(inv1, dx1);
    let ny = fmul(inv1, dy1);
    fr[0x0c / 4] = nx;
    let inv2 = {
        let len2 = fadd(dy2 * dy2, dx2 * dx2);
        if len2 == 0.0 { 0.0 } else { 1.0 / len2.sqrt() }
    };
    let qx = fmul(inv2, dx2);
    let qy = fmul(inv2, dy2);
    fr[0x24 / 4] = -nx;
    fr[0x20 / 4] = ny;
    fr[0x1c / 4] = -qx;
    fr[0x18 / 4] = qy;
    quad(&mut fr, 1, 0x58, 0x28);
    quad(&mut fr, 2, 0x48, 0x20);
    let d1 = -(fadd(fadd(fmul(fr[0x48 / 4], fr[0x58 / 4]),
                          fmul(fr[0x4c / 4], fr[0x5c / 4])),
                     fmul(fr[0x50 / 4], fr[0x60 / 4])));
    fr[0x0c / 4] = d1;
    quad(&mut fr, 3, 0x48, 0x30);
    quad(&mut fr, 4, 0x58, 0x18);
    let d2 = -(fadd(fadd(fmul(fr[0x58 / 4], fr[0x48 / 4]),
                          fmul(fr[0x5c / 4], fr[0x4c / 4])),
                     fmul(fr[0x60 / 4], fr[0x50 / 4])));
    fr[0x08 / 4] = d2;
    quad(&mut fr, 5, 0x48, 0x18);
    quad(&mut fr, 6, 0x58, 0x28);
    let d3 = fadd(fadd(fadd(fmul(fr[0x58 / 4], fr[0x48 / 4]),
                                 fmul(fr[0x5c / 4], fr[0x4c / 4])),
                            fmul(fr[0x60 / 4], fr[0x50 / 4])),
                       fr[0x08 / 4]);
    fr[0x10 / 4] = d3;
    quad(&mut fr, 7, 0x48, 0x18);
    quad(&mut fr, 8, 0x58, 0x38);
    let d4 = fadd(fadd(fadd(fmul(fr[0x58 / 4], fr[0x48 / 4]),
                                 fmul(fr[0x5c / 4], fr[0x4c / 4])),
                            fmul(fr[0x60 / 4], fr[0x50 / 4])),
                       fr[0x08 / 4]);
    fr[0x08 / 4] = d4;
    quad(&mut fr, 9, 0x48, 0x20);
    quad(&mut fr, 10, 0x58, 0x30);
    let d5 = fadd(fadd(fadd(fmul(fr[0x58 / 4], fr[0x48 / 4]),
                                 fmul(fr[0x5c / 4], fr[0x4c / 4])),
                            fmul(fr[0x60 / 4], fr[0x50 / 4])),
                       fr[0x0c / 4]);
    fr[0x14 / 4] = d5;
    quad(&mut fr, 11, 0x48, 0x20);
    quad(&mut fr, 12, 0x58, 0x40);
    let d6 = fadd(fadd(fadd(fmul(fr[0x58 / 4], fr[0x48 / 4]),
                                 fmul(fr[0x5c / 4], fr[0x4c / 4])),
                            fmul(fr[0x60 / 4], fr[0x50 / 4])),
                       fr[0x0c / 4]);
    let s1 = if !(d3 < 0.0) { 1i32 } else { -1 };
    let mut s2 = if !(d4 < 0.0) { 1i32 } else { -1 };
    let s3 = if !(d5 < 0.0) { 1i32 } else { -1 };
    let s4 = if !(d6 < 0.0) { 1i32 } else { -1 };
    // The original's zero-fallback moves never fire: every sign is ±1.
    if s1 == s2 && (d3.abs() < 0.05 || d4.abs() < 0.05) {
        s2 = -s1;
    }
    if s1 == s2 || s3 == s4 {
        return 1;
    }
    let t = d5 / (d5 - d6);
    unsafe {
        *e = fadd(fmul(d0 - c0, t), c0);
        *e.add(1) = fadd(fmul(d1v - c1, t), c1);
        *e.add(2) = fadd(fmul(d2v - c2, t), c2);
        *e.add(3) = fr[0x64 / 4];
    }
    2
});
