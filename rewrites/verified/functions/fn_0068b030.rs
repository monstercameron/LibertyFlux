// original: 0x0068b030 blend_pose_track (proposed)

/// Blend one animation pose track toward its target poses.
///
/// `this` points at a track header whose first word leads to a record holding
/// the element count (u16 at +0x10) and the destination element-pointer array
/// (at +0xc). `a0` likewise leads to the source element-pointer array and `a2`
/// is the per-element blend-factor array. Element `k` blends destination `s`
/// toward source `d` by `t = pa[k]`, scaled by the incoming scalar in xmm3.
///
/// Each destination carries a mode byte at +0x4: the low nibble selects the
/// blend (1 = quaternion slerp-ish blend with normalize, 0 = xyz lerp with
/// snap, 2 = x-only lerp with snap, anything else = threshold copy of x) and
/// bit 4 marks the element settled. Source elements with bit 4 set, and zero
/// blend factors, are skipped. The blended quaternion is normalized through
/// the intercepted root callee; a zero-length result normalizes to +0s.
///
/// The two scalar SSE accumulators the original threads through the loop are
/// carried explicitly (`s0`/`s1`): most paths reset them, the settled-copy
/// and threshold paths preserve them, and the x-only path leaves the stale
/// threshold behind. Float operation order matches the original exactly.
export!(thiscall, rw_0068b030(this: u32, a0: u32, _a1: u32, a2: u32) -> u32 {
    unsafe {
        const EPS: f32 = f32::from_bits(0x3a83_126f); // 0.001, lerp-entry threshold
        const SNAP: f32 = f32::from_bits(0x3f7f_be77); // 0.999, snap threshold
        const HALF: f32 = 0.5;

        #[inline(always)]
        unsafe fn cell(base: u32, off: u32) -> f32 {
            // SAFETY: checker-fabricated heap object, offset in bounds.
            unsafe { f32::from_bits(((base + off) as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn put_cell(base: u32, off: u32, v: f32) {
            // SAFETY: checker-fabricated heap object, offset in bounds.
            unsafe { ((base + off) as *mut u32).write_unaligned(v.to_bits()) }
        }
        // Ordered float arithmetic: every operand passes through black_box so
        // the compiler must emit the scalar ops in written order. Without this
        // LLVM packs the independent lanes into mulps/addps and reduces them
        // in a shuffled order, which changes NaN payloads (verified: the first
        // build failed exactly one NaN trial in 200 on the sqrt argument).
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fdiv(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn copy_word(dst: u32, src: u32, off: u32) {
            // SAFETY: checker-fabricated heap objects, offset in bounds.
            unsafe {
                let w = ((src + off) as *const u32).read_unaligned();
                ((dst + off) as *mut u32).write_unaligned(w);
            }
        }

        let h1 = (this as *const u32).read_unaligned();
        let n = ((h1 + 0x10) as *const u16).read_unaligned() as u32;
        if n == 0 {
            return 0;
        }
        let s3 = f32::from_bits(xmm_word(3, 0));
        let mut s0 = EPS;
        let mut s1 = s3;
        let pc = ((h1 + 0x0c) as *const u32).read_unaligned();
        let pb = ((a0 + 0x0c) as *const u32).read_unaligned();
        let mut pa = a2;
        let mut ib = pb;
        let mut ic = pc;
        let mut left = n;
        while left != 0 {
            left -= 1;
            let tbits = (pa as *const u32).read_unaligned();
            let t = f32::from_bits(tbits);
            pa += 4;
            let d = (ib as *const u32).read_unaligned();
            ib += 4;
            let s = (ic as *const u32).read_unaligned();
            ic += 4;
            let df = ((d + 4) as *const u8).read();
            if df & 0x10 != 0 {
                continue;
            }
            if tbits & 0x7fff_ffff == 0 {
                continue;
            }
            let cl = ((s + 4) as *const u8).read();
            let t3 = fmul(t, s1);
            match cl & 0x0f {
                1 if cl & 0x10 == 0 => {
                    let (mut sx, mut sy, mut sz, mut sw) =
                        (cell(s, 0x10), cell(s, 0x14), cell(s, 0x18), cell(s, 0x1c));
                    let (dx, dy, dz, dw) =
                        (cell(d, 0x10), cell(d, 0x14), cell(d, 0x18), cell(d, 0x1c));
                    let mut dot = fmul(sy, dy);
                    dot = fadd(dot, fmul(dx, sx));
                    dot = fadd(dot, fmul(sz, dz));
                    dot = fadd(dot, fmul(dw, sw));
                    if dot < 0.0 {
                        sx = -sx;
                        sy = -sy;
                        sz = -sz;
                        sw = -sw;
                        put_cell(s, 0x10, sx);
                        put_cell(s, 0x14, sy);
                        put_cell(s, 0x18, sz);
                        put_cell(s, 0x1c, sw);
                    }
                    let u = fsub(1.0, t3);
                    let nx = fadd(fmul(dx, t3), fmul(sx, u));
                    let ny = fadd(fmul(sy, u), fmul(t3, dy));
                    let nz = fadd(fmul(t3, dz), fmul(sz, u));
                    let nw = fadd(fmul(dw, t3), fmul(u, sw));
                    put_cell(s, 0x10, nx);
                    put_cell(s, 0x14, ny);
                    put_cell(s, 0x18, nz);
                    put_cell(s, 0x1c, nw);
                    let mut n2 = fmul(nx, nx);
                    n2 = fadd(n2, fmul(ny, ny));
                    n2 = fadd(n2, fmul(nz, nz));
                    n2 = fadd(n2, fmul(nw, nw));
                    if n2.to_bits() & 0x7fff_ffff != 0 {
                        let r: f64 = callee_cdecl!(0, f64, n2.to_bits());
                        let f = r as f32;
                        let inv = fdiv(1.0, f);
                        put_cell(s, 0x10, fmul(nx, inv));
                        put_cell(s, 0x14, fmul(ny, inv));
                        put_cell(s, 0x18, fmul(nz, inv));
                        put_cell(s, 0x1c, fmul(inv, nw));
                    } else {
                        put_cell(s, 0x10, fmul(nx, 0.0));
                        put_cell(s, 0x14, fmul(ny, 0.0));
                        put_cell(s, 0x18, fmul(nz, 0.0));
                        put_cell(s, 0x1c, fmul(nw, 0.0));
                    }
                    s0 = EPS;
                    s1 = s3;
                }
                1 => {
                    let q0 = ((d + 0x10) as *const u64).read_unaligned();
                    ((s + 0x10) as *mut u64).write_unaligned(q0);
                    let q1 = ((d + 0x18) as *const u64).read_unaligned();
                    ((s + 0x18) as *mut u64).write_unaligned(q1);
                    ((s + 4) as *mut u8).write(cl & 0xef);
                    s0 = EPS;
                }
                0 => {
                    if t3 > s0 {
                        if !(t3 < SNAP) || cl & 0x10 != 0 {
                            copy_word(s, d, 0x10);
                            copy_word(s, d, 0x14);
                            copy_word(s, d, 0x18);
                            copy_word(s, d, 0x1c);
                        } else {
                            let nx = fadd(
                                fmul(fsub(cell(d, 0x10), cell(s, 0x10)), t3),
                                cell(s, 0x10),
                            );
                            put_cell(s, 0x10, nx);
                            let ny = fadd(
                                fmul(fsub(cell(d, 0x14), cell(s, 0x14)), t3),
                                cell(s, 0x14),
                            );
                            put_cell(s, 0x14, ny);
                            let nz = fadd(
                                fmul(fsub(cell(d, 0x18), cell(s, 0x18)), t3),
                                cell(s, 0x18),
                            );
                            put_cell(s, 0x18, nz);
                        }
                        ((s + 4) as *mut u8).write(cl & 0xef);
                        s0 = EPS;
                        s1 = s3;
                    }
                }
                2 => {
                    if !(t3 < SNAP) || cl & 0x10 != 0 {
                        if t3 > s0 {
                            copy_word(s, d, 0x10);
                            ((s + 4) as *mut u8).write(if df & 0x10 != 0 {
                                cl | 0x10
                            } else {
                                cl & 0xef
                            });
                        }
                        s1 = s3;
                    } else {
                        let nx = fadd(
                            fmul(fsub(cell(d, 0x10), cell(s, 0x10)), t3),
                            cell(s, 0x10),
                        );
                        put_cell(s, 0x10, nx);
                        s0 = EPS;
                        s1 = s3;
                        ((s + 4) as *mut u8).write(if df & 0x10 != 0 {
                            cl | 0x10
                        } else {
                            cl & 0xef
                        });
                    }
                }
                _ => {
                    // jb (CF only): unordered counts as below, so NaN skips.
                    if t3 >= HALF {
                        copy_word(s, d, 0x10);
                        ((s + 4) as *mut u8).write(cl & 0xef);
                    }
                }
            }
        }
    }
    0
});
