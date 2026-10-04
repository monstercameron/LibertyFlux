// original: 0x0068b750 blend_pose_track_indexed (proposed)

/// Blend one animation pose track through indexed and merge-joined passes.
///
/// `this` and `a0` are track headers as in the sibling blenders, `a2` is the
/// blend-factor array. A gate call first resolves the index array: when it
/// answers, loop 1 blends the packed (source, destination, factor) triples it
/// lists; when the gate is skipped or answers null, loop 2 merge-joins the
/// two element arrays on their key bytes instead. Both loops run the same
/// per-mode blends as the siblings (quaternion blend with normalize, xyz
/// lerp with snap, x-only lerp with snap, threshold copy of x) with this
/// function's own float operation order. The epilogue drops a mutex around a
/// shared-counter decrement when the gate supplied the objects.
export!(thiscall, rw_0068b750(this: u32, a0: u32, _a1: u32, a2: u32) -> u32 {
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
        #[inline(always)]
        unsafe fn flag_sync(s: u32, d: u32, cl: u8) {
            // SAFETY: checker-fabricated heap objects, offsets in bounds.
            unsafe {
                let df = ((d + 4) as *const u8).read();
                ((s + 4) as *mut u8).write(if df & 0x10 != 0 { cl | 0x10 } else { cl & 0xef });
            }
        }

        let h1 = (this as *const u32).read_unaligned();
        let s3 = f32::from_bits(xmm_word(3, 0));
        let mut gout = [0u32; 2];
        if ((h1 + 8) as *const u32).read_unaligned() != 0
            && ((a0 + 8) as *const u32).read_unaligned() != 0
        {
            let mut cx = ((h1 + 4) as *const u32).read_unaligned();
            if cx == 0 {
                cx = ((a0 + 4) as *const u32).read_unaligned();
            }
            if cx != 0 {
                callee_stdcall!(1, u32, core::ptr::addr_of_mut!(gout) as u32, h1, a0, cx);
            }
        }
        let m = gout[0];
        let g = gout[1];
        let gate = if g != 0 { ((g + 0x0c) as *const u32).read_unaligned() } else { 0 };
        let n1 = ((h1 + 0x10) as *const u16).read_unaligned() as u32;
        let n2 = ((a0 + 0x10) as *const u16).read_unaligned() as u32;
        if gate != 0 {
            let count = ((gate as *const u32).read_unaligned()) as i32;
            if count > 0 {
                let pa = ((a0 + 0x0c) as *const u32).read_unaligned();
                let pb = ((h1 + 0x0c) as *const u32).read_unaligned();
                let mut s0 = EPS;
                let mut k = 0u32;
                while k < count as u32 {
                    let packed = ((gate + 4 + k * 4) as *const u32).read_unaligned();
                    k += 1;
                    let hi = packed >> 16;
                    let lo = packed & 0xffff;
                    let s = ((pb + hi * 4) as *const u32).read_unaligned();
                    let d = ((pa + lo * 4) as *const u32).read_unaligned();
                    let tbits = ((a2 + hi * 4) as *const u32).read_unaligned();
                    let t = f32::from_bits(tbits);
                    let df = ((d + 4) as *const u8).read();
                    if df & 0x10 != 0 {
                        continue;
                    }
                    if tbits & 0x7fff_ffff == 0 {
                        continue;
                    }
                    let cl = ((s + 4) as *const u8).read();
                    let t3 = fmul(t, s3);
                    match cl & 0x0f {
                        1 if cl & 0x10 == 0 => {
                            let (mut sx, mut sy, mut sz, mut sw) =
                                (cell(s, 0x10), cell(s, 0x14), cell(s, 0x18), cell(s, 0x1c));
                            let (dx, dy, dz, dw) =
                                (cell(d, 0x10), cell(d, 0x14), cell(d, 0x18), cell(d, 0x1c));
                            let mut dot = fmul(dy, sy);
                            dot = fadd(dot, fmul(dx, sx));
                            dot = fadd(dot, fmul(dz, sz));
                            dot = fadd(dot, fmul(sw, dw));
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
                            let ny = fadd(fmul(sy, u), fmul(dy, t3));
                            let nz = fadd(fmul(sz, u), fmul(dz, t3));
                            let nw = fadd(fmul(sw, u), fmul(dw, t3));
                            put_cell(s, 0x10, nx);
                            put_cell(s, 0x14, ny);
                            put_cell(s, 0x18, nz);
                            put_cell(s, 0x1c, nw);
                            let mut n2v = fmul(nx, nx);
                            n2v = fadd(n2v, fmul(ny, ny));
                            n2v = fadd(n2v, fmul(nz, nz));
                            n2v = fadd(n2v, fmul(nw, nw));
                            if n2v.to_bits() & 0x7fff_ffff != 0 {
                                let r: f64 = callee_cdecl!(0, f64, n2v.to_bits());
                                let f = r as f32;
                                let inv = fdiv(1.0, f);
                                put_cell(s, 0x10, fmul(inv, cell(s, 0x10)));
                                put_cell(s, 0x14, fmul(inv, ny));
                                put_cell(s, 0x18, fmul(inv, nz));
                                put_cell(s, 0x1c, fmul(cell(s, 0x1c), inv));
                            } else {
                                put_cell(s, 0x10, fmul(0.0, cell(s, 0x10)));
                                put_cell(s, 0x14, fmul(0.0, ny));
                                put_cell(s, 0x18, fmul(0.0, nz));
                                put_cell(s, 0x1c, fmul(cell(s, 0x1c), 0.0));
                            }
                            s0 = EPS;
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
                            }
                        }
                        2 => {
                            if !(t3 < SNAP) || cl & 0x10 != 0 {
                                if t3 > s0 {
                                    copy_word(s, d, 0x10);
                                    flag_sync(s, d, cl);
                                }
                            } else {
                                let nx = fadd(
                                    fmul(fsub(cell(d, 0x10), cell(s, 0x10)), t3),
                                    cell(s, 0x10),
                                );
                                put_cell(s, 0x10, nx);
                                s0 = EPS;
                                flag_sync(s, d, cl);
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
        } else if n1 != 0 && n2 != 0 {
            let pa2 = ((a0 + 0x0c) as *const u32).read_unaligned();
            let pc0 = ((h1 + 0x0c) as *const u32).read_unaligned();
            let mut edx = pa2;
            let mut pd = pa2;
            let mut pc = pc0;
            let mut pt = a2;
            let mut edi = 0u32;
            let mut s0 = EPS;
            let mut left = n1;
            while left != 0 {
                left -= 1;
                let tbits = (pt as *const u32).read_unaligned();
                let t = f32::from_bits(tbits);
                let s = (pc as *const u32).read_unaligned();
                pt += 4;
                // The outer top also advances a second arg2 cursor whose word
                // feeds the zero test below; it shadows `t` exactly, so `t`
                // is tested directly. The PA2 cursor is untouched here: it
                // advances only on inner misses and after matches.
                if (edi as i32) >= (n2 as i32) {
                    pc += 4;
                    continue;
                }
                let ks = (((s + 5) as *const u8).read() as u32) << 16
                    | ((s + 6) as *const u16).read_unaligned() as u32;
                let mut d = (edx as *const u32).read_unaligned();
                loop {
                    let kd = (((d + 5) as *const u8).read() as u32) << 16
                        | ((d + 6) as *const u16).read_unaligned() as u32;
                    if kd == ks {
                        break;
                    }
                    if kd > ks {
                        pc += 4;
                        d = 0;
                        break;
                    }
                    edi += 1;
                    edx = pd;
                    edx += 4;
                    pd = edx;
                    if !((edi as i32) < (n2 as i32)) {
                        pc += 4;
                        d = 0;
                        break;
                    }
                    d = (edx as *const u32).read_unaligned();
                }
                if d == 0 {
                    continue;
                }
                let df = ((d + 4) as *const u8).read();
                if df & 0x10 == 0 && tbits & 0x7fff_ffff != 0 {
                    let cl = ((s + 4) as *const u8).read();
                    let t3 = fmul(t, s3);
                    match cl & 0x0f {
                        1 if cl & 0x10 == 0 => {
                            let (mut sx, mut sy, mut sz, mut sw) =
                                (cell(s, 0x10), cell(s, 0x14), cell(s, 0x18), cell(s, 0x1c));
                            let (dx, dy, dz, dw) =
                                (cell(d, 0x10), cell(d, 0x14), cell(d, 0x18), cell(d, 0x1c));
                            let mut dot = fmul(sx, dx);
                            dot = fadd(dot, fmul(dy, sy));
                            dot = fadd(dot, fmul(dz, sz));
                            dot = fadd(dot, fmul(sw, dw));
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
                            let nx = fadd(fmul(t3, dx), fmul(u, sx));
                            let ny = fadd(fmul(t3, dy), fmul(sy, u));
                            let nz = fadd(fmul(sz, u), fmul(dz, t3));
                            let nw = fadd(fmul(sw, u), fmul(dw, t3));
                            put_cell(s, 0x10, nx);
                            put_cell(s, 0x14, ny);
                            put_cell(s, 0x18, nz);
                            put_cell(s, 0x1c, nw);
                            let mut n2v = fmul(nx, nx);
                            n2v = fadd(n2v, fmul(ny, ny));
                            n2v = fadd(n2v, fmul(nz, nz));
                            n2v = fadd(n2v, fmul(nw, nw));
                            if n2v.to_bits() & 0x7fff_ffff != 0 {
                                let r: f64 = callee_cdecl!(0, f64, n2v.to_bits());
                                let f = r as f32;
                                let inv = fdiv(1.0, f);
                                put_cell(s, 0x10, fmul(inv, cell(s, 0x10)));
                                put_cell(s, 0x14, fmul(inv, ny));
                                put_cell(s, 0x18, fmul(inv, nz));
                                put_cell(s, 0x1c, fmul(cell(s, 0x1c), inv));
                            } else {
                                put_cell(s, 0x10, fmul(0.0, cell(s, 0x10)));
                                put_cell(s, 0x14, fmul(0.0, ny));
                                put_cell(s, 0x18, fmul(0.0, nz));
                                put_cell(s, 0x1c, fmul(cell(s, 0x1c), 0.0));
                            }
                            s0 = EPS;
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
                            }
                        }
                        2 => {
                            if !(t3 < SNAP) || cl & 0x10 != 0 {
                                if t3 > s0 {
                                    copy_word(s, d, 0x10);
                                    flag_sync(s, d, cl);
                                }
                            } else {
                                let nx = fadd(
                                    fmul(fsub(cell(d, 0x10), cell(s, 0x10)), t3),
                                    cell(s, 0x10),
                                );
                                put_cell(s, 0x10, nx);
                                s0 = EPS;
                                flag_sync(s, d, cl);
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
                edx = pd;
                edi += 1;
                edx += 4;
                pd = edx;
                pc += 4;
            }
        }
        if g != 0 {
            let slot_w = global::<u32>(0x00e73188).read();
            let slot_r = global::<u32>(0x00e731b0).read();
            let h = ((m + 0x10) as *const u32).read_unaligned();
            if h != 0 {
                let wait: extern "stdcall" fn(u32, u32) -> u32 =
                    core::mem::transmute(slot_w as usize);
                wait(h, 0xffff_ffff);
            }
            let c = ((g + 8) as *const u32).read_unaligned();
            ((g + 8) as *mut u32).write(c.wrapping_sub(1));
            let h = ((m + 0x10) as *const u32).read_unaligned();
            if h != 0 {
                let release: extern "stdcall" fn(u32) -> u32 =
                    core::mem::transmute(slot_r as usize);
                release(h);
            }
        }
    }
    0
});
