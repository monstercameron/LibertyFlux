// original: 0x0068b3c0 blend_pose_track_gated (proposed)

/// Blend one animation pose track toward its target poses, gated by a virtual call.
///
/// Same track layout as the sibling blender (`rw_0068b030`): `this` leads to
/// the element count and the destination element-pointer array, `a0` leads to
/// the source array. There is no blend-factor array here; instead each element
/// first calls virtual slot 3 of `a1` with the element's key byte (+0x5), key
/// word (+0x6) and an out-float. A false return skips the element, otherwise
/// the blend factor is the returned float times the incoming xmm3 scalar.
///
/// The per-mode blends match the sibling cell for cell (quaternion blend with
/// normalize, xyz lerp with snap, x-only lerp with snap, threshold copy of x)
/// except the float operation order, which this function schedules slightly
/// differently, and the thresholds, which are constants rather than carried
/// loop state. The third call argument carries one meaningful byte above
/// stale stack fill; its comparison is skipped in the contract.
export!(thiscall, rw_0068b3c0(this: u32, a0: u32, a1: u32, _a2: u32) -> u32 {
    unsafe {
        const EPS: f32 = f32::from_bits(0x3a83_126f); // 0.001, lerp-entry threshold
        const SNAP: f32 = f32::from_bits(0x3f7f_be77); // 0.999, snap threshold
        const HALF: f32 = 0.5;

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
        unsafe fn cell(base: u32, off: u32) -> f32 {
            // SAFETY: checker-fabricated heap object, offset in bounds.
            unsafe { f32::from_bits(((base + off) as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn put_cell(base: u32, off: u32, v: f32) {
            // SAFETY: checker-fabricated heap object, offset in bounds.
            unsafe { ((base + off) as *mut u32).write_unaligned(v.to_bits()) }
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
        let pc = ((h1 + 0x0c) as *const u32).read_unaligned();
        let pb = ((a0 + 0x0c) as *const u32).read_unaligned();
        // Virtual slot 3 of a1, planted with the recorder stub by the contract.
        let vt = (a1 as *const u32).read_unaligned();
        let slot = ((vt + 0x0c) as *const u32).read_unaligned();
        let gate: extern "thiscall" fn(u32, u32, u32, u32) -> u8 =
            core::mem::transmute(slot as usize);
        let mut left = n;
        let mut ib = pb;
        let mut ic = pc;
        while left != 0 {
            left -= 1;
            let d = (ib as *const u32).read_unaligned();
            ib += 4;
            let s = (ic as *const u32).read_unaligned();
            ic += 4;
            let df = ((d + 4) as *const u8).read();
            if df & 0x10 != 0 {
                continue;
            }
            let word = ((s + 6) as *const u16).read_unaligned() as u32;
            let key = ((s + 5) as *const u8).read() as u32;
            let mut out = 1.0f32;
            let out_ptr = core::ptr::addr_of_mut!(out) as u32;
            let ok = gate(a1, key, word, out_ptr);
            if ok == 0 {
                continue;
            }
            let t3 = fmul(out, s3);
            let cl = ((s + 4) as *const u8).read();
            match cl & 0x0f {
                1 if cl & 0x10 == 0 => {
                    let (mut sx, mut sy, mut sz, mut sw) =
                        (cell(s, 0x10), cell(s, 0x14), cell(s, 0x18), cell(s, 0x1c));
                    let (dx, dy, dz, dw) =
                        (cell(d, 0x10), cell(d, 0x14), cell(d, 0x18), cell(d, 0x1c));
                    let mut dot = fmul(sy, dy);
                    dot = fadd(dot, fmul(sx, dx));
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
                    let nx = fadd(fmul(sx, u), fmul(dx, t3));
                    let ny = fadd(fmul(sy, u), fmul(t3, dy));
                    let nz = fadd(fmul(sz, u), fmul(dz, t3));
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
                }
                1 => {
                    let q0 = ((d + 0x10) as *const u64).read_unaligned();
                    ((s + 0x10) as *mut u64).write_unaligned(q0);
                    let q1 = ((d + 0x18) as *const u64).read_unaligned();
                    ((s + 0x18) as *mut u64).write_unaligned(q1);
                    ((s + 4) as *mut u8).write(cl & 0xef);
                }
                0 => {
                    if t3 > EPS {
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
                    }
                }
                2 => {
                    if !(t3 < SNAP) || cl & 0x10 != 0 {
                        if t3 > EPS {
                            copy_word(s, d, 0x10);
                            ((s + 4) as *mut u8).write(if df & 0x10 != 0 {
                                cl | 0x10
                            } else {
                                cl & 0xef
                            });
                        }
                    } else {
                        let nx = fadd(
                            fmul(fsub(cell(d, 0x10), cell(s, 0x10)), t3),
                            cell(s, 0x10),
                        );
                        put_cell(s, 0x10, nx);
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
