// original: 0x00a6da70 peds_task_solve_direction (proposed)

/// Solve a task's aim/direction basis into a small caller struct, then scan
/// candidate targets and keep the closest one.
///
/// `out` points to a caller struct of about 0x40 bytes. Word `+0x28` points
/// at the task object; dword `+0x20` of that points at a 16-byte vector
/// block (`+0x10` id, `+0x14`/`+0x18` floats, `+0x1c` flags) which is copied
/// to `out+0x10..0x20` first.
///
/// The solver then runs, in order:
/// 1. Two integer helpers (callees 2..5) produce floats B and D (D negated).
///    A mode helper (callee 6, `this` is a game global) returns float G, and
///    a SIGNED mode word at `task+0xB80` selects the seed of `out+0x20`:
///    3.0 when the mode is >= 3, 1.5 when it is == 2, else 0.0.
/// 2. If `D*D + B*B` is not above 0.0 (a `comiss`/`jbe`, so NaN counts too),
///    the double stage is skipped: byte `out+0x24` set returns the mode word
///    with its low byte cleared, otherwise solving rejoins below.
/// 3. Double stage: a two-double helper (callee 7, doubles in XMM0/XMM1)
///    over (-B, -D), plus G, through a float helper (callee 8), then two
///    float-in/float-out helpers (callees 9, 10). Their results scale the
///    direction into `out+0x10..0x20`; every reciprocal-length scale in this
///    function is 0.0 for a zero input and 1.0/sqrt(x) otherwise (the
///    original tests this with `ucomiss`+`lahf`, which also takes the
///    square-root path for NaN).
/// 4. A second normalize pass over two global floats, then a flag word
///    (`task2+0x50`, reached via `task+0xA80`): when bit 1 is set and byte
///    `out+0x24` is clear, the function returns `((word >> 1) & ~0xFF) | 1`.
/// 5. When byte `out+0x2C` is set, words `out+0x30..0x40` are copied down to
///    `out+0x00..0x10` and the function returns 1 (low byte forced; the
///    upper bytes are the copied word `+0x3C`'s).
/// 6. Otherwise the direction just solved is copied to `out+0x00..0x10` and
///    an iterator (callees 13/17 over a stack cursor) walks candidates.
///    Each candidate carries a skip flag (`+0x211`), a matcher (callee 14,
///    compared against the task pointer), a virtual gate (slot `+0x128`,
///    thiscall on the candidate) and a second gate (callee 16); survivors
///    walk a short kind chain (`+0x224` -> `+0x2E0`, entries skipped when
///    word `+4` is 0x772) and contribute a normalized offset vector, scaled
///    by distance bands (3.0 under length 5, 0.2 over length 20, a blend in
///    between), accumulated into a running sum while tracking the closest.
///    After the walk a dot-product scan may reject the whole set; if kept,
///    the normalized sum is stored to `out+0x00..0x10`.
///
/// Returns an `eax` value whose low byte is forced on most paths (0 on the
/// early exit, 1 on the tails) with meaningful upper bytes: the mode word, the
/// copied word, or the cleanup call's answer.
///
/// Calling convention: cdecl, one stack word, plain `ret`. The original has
/// a security cookie; the cookie checks are intercepted callees here. Two
/// frame slots the original reads without writing (the float stored to
/// `out+0x1C`/`out+0x0C`, and the second float helper's input) hold the
/// checker's stack fill, 0.0; the float helper's vector input is therefore
/// the constant 0.0 and its upper lanes hold sign-mask residue the checker
/// cannot compare, so that input is uncompared (see the contract).
lf_checker_rt::export!(cdecl, rw_00a6da70(out_: u32) -> u32 {
    unsafe {
        const TASK_OF_OUT: u32 = 0x28;
        const VEC_OF_TASK: u32 = 0x20;
        const MODE_OFF: u32 = 0xB80;
        const FLAGOBJ_OFF: u32 = 0xA80;
        const FLAGWORD_OFF: u32 = 0x50;
        const POS_OF_TASK: u32 = 0x20;
        const SKIP_KIND: u32 = 0x772;
        const VT_GATE_SLOT: u32 = 0x128;
        const MODE_OBJ_GLOBAL: u32 = 0x128E310;
        const GLOB_F320: u32 = 0x128E320;
        const GLOB_F324: u32 = 0x128E324;
        const GLOB_F25: u32 = 0x103CED0;
        const GLOB_ONE: u32 = 0xFE88E8;
        const GLOB_INV120: u32 = 0xFE8708;
        const GLOB_THREE: u32 = 0xFE8A94;
        const GLOB_FIVE: u32 = 0xFE8AD8;
        const GLOB_TWENTY: u32 = 0xFE8B38;
        const GLOB_BLEND_K: u32 = 0xFE877C;
        const GLOB_BLEND_W: u32 = 0xFE87D0;
        const SIGN_BIT: u32 = 0x8000_0000;
        const UNINIT_FRAME_FLOAT: f32 = 0.0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn glob_f(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(a))) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn neg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ SIGN_BIT)
        }
        /// Original's `comiss a, b; jbe`: taken when a <= b or unordered.
        #[inline(always)]
        fn jbe(a: f32, b: f32) -> bool {
            !(a > b)
        }
        /// Original's `ucomiss x, 0; lahf; (an instruction of the original); jp/jnp` scale:
        /// 0.0 for +-0.0, else 1.0/sqrt(x) (NaN takes the root path).
        #[inline(always)]
        fn rsqrt_nz(x: f32) -> f32 {
            let b = x.to_bits();
            if b == 0 || b == SIGN_BIT {
                0.0
            } else {
                div(1.0, x.sqrt())
            }
        }
        #[inline(always)]
        unsafe fn cookie() {
            unsafe {
                lf_checker_rt::callee_cdecl!(11u32, u32,);
            }
        }

        let o1 = rd32(out_ + TASK_OF_OUT);
        let o3: u32 = lf_checker_rt::callee_thiscall!(1u32, u32, o1);
        let o2 = rd32(o1 + VEC_OF_TASK);
        let a: u32 = lf_checker_rt::callee_thiscall!(2u32, u32, o3);
        let b_int: u32 = lf_checker_rt::callee_cdecl!(3u32, u32, a);
        let fb: f32 = (b_int as i32) as f32;
        let c: u32 = lf_checker_rt::callee_thiscall!(4u32, u32, o3);
        let d_int: u32 = lf_checker_rt::callee_cdecl!(5u32, u32, c);
        let fd: f32 = neg((d_int as i32) as f32);
        wr32(out_ + 0x10, rd32(o2 + 0x10));
        wrf(out_ + 0x14, rdf(o2 + 0x14));
        wrf(out_ + 0x18, rdf(o2 + 0x18));
        wr32(out_ + 0x1C, rd32(o2 + 0x1C));
        let g: f32 = lf_checker_rt::callee_thiscall!(
            6u32, f32, lf_checker_rt::relocated(MODE_OBJ_GLOBAL), o1, 0
        );
        let mode = rd32(o1 + MODE_OFF);
        wrf(out_ + 0x20, 0.0);
        // The mode compare is SIGNED (jl after (an instruction of the original).
        if (mode as i32) >= 3 {
            wrf(out_ + 0x20, 3.0);
        } else if mode == 2 {
            wrf(out_ + 0x20, 1.5);
        }
        let norm2 = add(mul(fd, fd), mul(fb, fb));
        if jbe(norm2, 0.0) {
            if rd8(out_ + 0x24) != 0 {
                cookie();
                return mode & 0xFFFF_FF00;
            }
        } else {
            let neg_b = neg(fb);
            let d0 = f64::from(neg_b);
            let d1lo = f64::from(fd);
            let d0b = d0.to_bits();
            let d1b = d1lo.to_bits();
            let h_bits: u64 = lf_checker_rt::callee_cdecl!(
                7u32, u64,
                (d0b & 0xFFFF_FFFF) as u32, (d0b >> 32) as u32,
                (d1b & 0xFFFF_FFFF) as u32, (d1b >> 32) as u32
            );
            let h = f64::from_bits(h_bits) as f32;
            let i_arg = add(h, g);
            let i: f32 = lf_checker_rt::callee_cdecl!(8u32, f32, i_arg.to_bits());
            let j: u32 = lf_checker_rt::callee_cdecl!(9u32, u32, i.to_bits());
            let fj = f32::from_bits(j);
            let neg_j = neg(fj);
            let k: u32 = lf_checker_rt::callee_cdecl!(10u32, u32,);
            let fk = f32::from_bits(k);
            let kk2 = add(mul(fk, fk), mul(neg_j, neg_j));
            wrf(out_ + 0x1C, UNINIT_FRAME_FLOAT);
            wrf(out_ + 0x10, neg_j);
            wrf(out_ + 0x14, fk);
            wrf(out_ + 0x18, 0.0);
            let scale_a = rsqrt_nz(kk2);
            let root_n = norm2.sqrt();
            let f20 = mul(mul(root_n, glob_f(GLOB_INV120)), glob_f(GLOB_THREE));
            wrf(out_ + 0x10, mul(neg_j, scale_a));
            wrf(out_ + 0x14, mul(fk, scale_a));
            wrf(out_ + 0x18, mul(scale_a, 0.0));
            wrf(out_ + 0x20, f20);
        }
        let c320 = glob_f(GLOB_F320);
        let c324 = glob_f(GLOB_F324);
        wrf(out_, c320);
        wrf(out_ + 0x0C, UNINIT_FRAME_FLOAT);
        wrf(out_ + 0x04, c324);
        wrf(out_ + 0x08, 0.0);
        let qq2 = add(mul(c320, c320), mul(c324, c324));
        let scale_b = rsqrt_nz(qq2);
        wrf(out_, mul(c320, scale_b));
        wrf(out_ + 0x04, mul(c324, scale_b));
        wrf(out_ + 0x08, mul(scale_b, 0.0));
        let flag_word = rd32(rd32(o1 + FLAGOBJ_OFF) + FLAGWORD_OFF);
        if ((flag_word >> 1) & 1) != 0 && rd8(out_ + 0x24) == 0 {
            cookie();
            return ((flag_word >> 1) & 0xFFFF_FF00) | 1;
        }
        if rd8(out_ + 0x2C) != 0 {
            let w30 = rd32(out_ + 0x30);
            let w34 = rdf(out_ + 0x34);
            let w38 = rdf(out_ + 0x38);
            let w3c = rd32(out_ + 0x3C);
            wr32(out_, w30);
            wrf(out_ + 0x04, w34);
            wrf(out_ + 0x08, w38);
            wr32(out_ + 0x0C, w3c);
            cookie();
            return (w3c & 0xFFFF_FF00) | 1;
        }
        let t10 = rd32(out_ + 0x10);
        let t14 = rdf(out_ + 0x14);
        let t18 = rdf(out_ + 0x18);
        let t1c = rd32(out_ + 0x1C);
        wr32(out_, t10);
        wrf(out_ + 0x04, t14);
        wrf(out_ + 0x08, t18);
        wr32(out_ + 0x0C, t1c);
        let o1pos = rd32(o1 + POS_OF_TASK);
        let mut cursor = [0u32; 8];
        let cur_ptr = cursor.as_mut_ptr() as u32;
        lf_checker_rt::callee_thiscall!(
            12u32, u32, cur_ptr, 1, o1, o1pos.wrapping_add(0x30),
            glob_f(GLOB_F25).to_bits()
        );
        let mut o4: u32 = lf_checker_rt::callee_thiscall!(13u32, u32, cur_ptr);
        if o4 != 0 {
            let o2b = rd32(o1 + VEC_OF_TASK);
            let mut tx = [0.0f32; 8];
            let mut ty = [0.0f32; 8];
            let mut tz = [0.0f32; 8];
            let mut cnt = 0u32;
            let mut has_min = false;
            let mut min_val = 0.0f32;
            let mut min_idx = 0u32;
            let mut sumx = 0.0f32;
            let mut sumy = 0.0f32;
            let mut sumz = 0.0f32;
            loop {
                let mut take = rd8(o4 + 0x211) == 0;
                if take {
                    let o4b = rd32(o4 + 0x224);
                    let r15: u32 = lf_checker_rt::callee_thiscall!(
                        14u32, u32, o4b.wrapping_add(0x2E0)
                    );
                    if r15 != o1 {
                        let vt = rd32(o4);
                        let gate: extern "thiscall" fn(u32) -> u32 =
                            unsafe { core::mem::transmute(rd32(vt + VT_GATE_SLOT) as usize) };
                        let gv = gate(o4);
                        if gv == 0 {
                            take = false;
                        } else {
                            let r17: u32 = lf_checker_rt::callee_thiscall!(
                                16u32, u32, rd32(o1 + 0x224), o4
                            );
                            if r17 != 0 {
                                take = false;
                            }
                        }
                    }
                    if take {
                        // Both lane values derive from the same word, so the
                        // unsigned compare always passes and the bounded walk
                        // below is the whole behaviour.
                        let mut w = rd32(o4b.wrapping_add(0x2E0));
                        while w != 0 {
                            if rd32(w + 4) == SKIP_KIND {
                                take = false;
                                break;
                            }
                            w = rd32(w + 0x0C);
                        }
                    }
                }
                if take {
                    let o4c = rd32(o4 + 0x20);
                    let dx = sub(rdf(o4c + 0x30), rdf(o2b + 0x30));
                    let dy = sub(rdf(o4c + 0x34), rdf(o2b + 0x34));
                    let dz = sub(rdf(o4c + 0x38), rdf(o2b + 0x38));
                    let q = add(mul(dy, dy), mul(dx, dx));
                    let dz2 = mul(dz, dz);
                    if jbe(q, dz2) {
                        take = false;
                    } else {
                        let root = add(dz2, q).sqrt();
                        let scale_c = rsqrt_nz(q);
                        let mut nx = mul(dx, scale_c);
                        let mut ny = mul(dy, scale_c);
                        let mut nz = mul(scale_c, 0.0);
                        if !jbe(glob_f(GLOB_FIVE), root) {
                            let three = glob_f(GLOB_THREE);
                            nx = mul(nx, three);
                            ny = mul(ny, three);
                            nz = mul(nz, three);
                        } else if !jbe(root, glob_f(GLOB_TWENTY)) {
                            let w = glob_f(GLOB_BLEND_W);
                            nx = mul(nx, w);
                            ny = mul(ny, w);
                            nz = mul(nz, w);
                        } else {
                            let t = sub(root, glob_f(GLOB_FIVE));
                            let u = mul(t, glob_f(GLOB_BLEND_K));
                            let v = sub(glob_f(GLOB_ONE), u);
                            let w = mul(u, glob_f(GLOB_BLEND_W));
                            let z = mul(v, glob_f(GLOB_THREE));
                            let f = add(z, w);
                            nz = mul(nz, f);
                            nx = mul(f, nx);
                            ny = mul(f, ny);
                        }
                        tx[cnt as usize] = nx;
                        ty[cnt as usize] = ny;
                        tz[cnt as usize] = nz;
                        cnt += 1;
                        sumx = add(nx, sumx);
                        sumy = add(ny, sumy);
                        sumz = add(nz, sumz);
                        if !has_min || min_val > root {
                            has_min = true;
                            min_val = root;
                            min_idx = cnt - 1;
                        }
                    }
                }
                o4 = lf_checker_rt::callee_thiscall!(17u32, u32, cur_ptr);
                if o4 == 0 {
                    break;
                }
            }
            if has_min {
                let mut dl = false;
                if (cnt as i32) > 0 {
                    let mut i = 0u32;
                    while i < cnt {
                        if i != min_idx {
                            let dot = add(
                                add(mul(sumy, ty[i as usize]), mul(sumx, tx[i as usize])),
                                mul(sumz, tz[i as usize]),
                            );
                            if jbe(dot, 0.0) {
                                dl = true;
                            }
                        }
                        i += 1;
                    }
                }
                if !dl {
                    let fin2 = add(
                        add(mul(sumy, sumy), mul(sumx, sumx)),
                        mul(sumz, sumz),
                    );
                    let scale_d = rsqrt_nz(fin2);
                    wrf(out_, mul(sumx, scale_d));
                    wrf(out_ + 0x04, mul(sumy, scale_d));
                    wrf(out_ + 0x08, mul(sumz, scale_d));
                    wrf(out_ + 0x0C, UNINIT_FRAME_FLOAT);
                }
            }
        }
        let c19: u32 = lf_checker_rt::callee_thiscall!(18u32, u32, cur_ptr);
        cookie();
        (c19 & 0xFFFF_FF00) | 1
    }
});
