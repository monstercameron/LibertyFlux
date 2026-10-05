// original: 0x00cadfe0 move_task_steer_update (proposed)

/// Advance a complex move task by one steering tick: refresh the goal from
/// the driver, blend avoidance, validate through the heading check and the
/// path probe, and commit the new waypoint. Returns 1 when the probe
/// accepts immediately, otherwise the validation byte.
///
/// `this` (ECX) is the task object; `ctx` is the move context. The tick has
/// two macro-paths. When the context is already attached
/// (`ctx+0x218` clear, `ctx+0x219` set, `ctx+0xd68 == ctx+0xe70`) the
/// waypoint stages are skipped and the tick goes straight to validation
/// and commit. Otherwise the driver kind (`[ctx+0xd68] & 7`) selects
/// further work for kinds 2-5: the goal point is refreshed through the
/// driver (callee 2), the goal is transformed by the entity matrix twice
/// (callees 3-8, created on demand), an avoidance direction is picked
/// (callee 9 for samples, callee 10 for the mode table), and a second
/// transform runs when the task is busy (callees 11-14, or a direct blend
/// when the matrix handle is null).
///
/// Validation calls the heading check (callee 16, the neighbouring
/// function at 0x00cadd10) and steers by its -1/0/1 answer; a second stage
/// rotates the offset by scripted calibration angles (callees 17-23) and
/// reconciles it with the driver (callees 24-25). The commit stage probes
/// the path (callee 26, 1 on immediate accept), re-syncs the driver
/// (callees 27-29), applies a final rotation picked by the mode word
/// (callees 30-32) and records the waypoint distance (callee 33, x87
/// single result) and the arrival flag (callee 34).
///
/// Float order is the original's everywhere, including the mixed-order
/// translation adds and the per-site squared-length associations. The
/// `lahf` zero tests are `!= 0.0` (NaN counts as non-zero), the window
/// tests strict ordered `>`. Packed square roots only ever expose their
/// low lane, so scalar roots match bit for bit. The double calibration
/// answers reach the rewrite as their low word in `eax` (all the stub
/// exposes); the contract fixes every high word to `F64_HI` (doubles in
/// [1, 2)), which the rewrite rejoins before narrowing. Slots the
/// original never writes before reading (`E+0x20/0x24/0x44` at the
/// reconcile step, the saved goal on skipped paths) read the checker's
/// zero stack fill, which the rewrite mirrors with zeroed locals; the
/// dead store to `E+0x5C` is omitted. Only the low return byte is
/// compared.
///
/// Original: 0x00cadfe0 (thiscall, ECX plus one stack word, callee
/// cleans 4). All globals are read relocated.
lf_checker_rt::export!(thiscall, rw_00cadfe0(this: u32, ctx: u32) -> u32 {
    unsafe {
        const D68: u32 = 0xd68;
        const ANCHOR_PTR: u32 = 0x20;
        const PT_X: u32 = 0x30;
        const MATRIX_SLOT: u32 = 0x20;
        const ONE_FILE: u32 = 0x00fe88e8;
        const D2MAX_FILE: u32 = 0x00fe876c;
        const T0_FILE: u32 = 0x0110db00;
        const T5_FILE: u32 = 0x0110db50;
        const T7_FILE: u32 = 0x0110db70;
        const G50_FILE: u32 = 0x01050e88;
        const F64_HI: u32 = 0x3FF00000;

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
        fn f64ans(lo: u32) -> f64 {
            f64::from_bits(((F64_HI as u64) << 32) | lo as u64)
        }
        #[inline(always)]
        unsafe fn rgf(file_va: u32) -> f32 {
            unsafe { lf_checker_rt::global::<f32>(file_va).read() }
        }

        let one = unsafe { lf_checker_rt::global::<f32>(ONE_FILE).read() };
        let anchor = rd32(ctx + ANCHOR_PTR);
        let anch_x = rdf(anchor + PT_X);
        let anch_y = rdf(anchor + PT_X + 4);
        let anch_z = rdf(anchor + PT_X + 8);
        let driver = rd32(ctx + D68);

        // Phase 1: driver refresh; the stub fills the four words.
        let mut w = [0u32; 4];
        lf_checker_rt::callee_thiscall!(1, u32, driver, w.as_mut_ptr() as u32, 0u32);
        let w0 = f32::from_bits(w[0]);
        let w1 = f32::from_bits(w[1]);
        let w2 = f32::from_bits(w[2]);
        let w3 = f32::from_bits(w[3]);
        let mut e50 = sub(w0, anch_x);
        let mut e54 = sub(w1, anch_y);
        let mut e58 = sub(w2, anch_z);

        // Phase 2: hold override when the hold distance is non-zero.
        // (The original also stores the fourth hold word to a slot that is
        // never read; that dead store is omitted here.)
        if rdf(this + 0x40) != 0.0 {
            e54 = rdf(this + 0x44);
            e58 = rdf(this + 0x48);
            e50 = rdf(this + 0x40);
        }

        // Saved-goal slots: zero unless the waypoint path writes them
        // (they read the checker's zero stack fill on skipped paths).
        let mut e10 = 0.0f32;
        let mut e0c = 0.0f32;
        let mut e30 = 0.0f32;

        // Phase 3: attached contexts skip the waypoint stages.
        let attached = rd8(ctx + 0x218) == 0 && rd8(ctx + 0x219) != 0 && driver == ctx + 0xe70;
        // Callee-9/28 output slots, shared across stages.
        let mut e60 = 0.0f32;
        let mut e64 = 0.0f32;
        let mut e68 = 0.0f32;
        // Phase-7 carried value (see below), set on every path that runs it.
        let mut ph7_t7 = 0.0f32;
        let mut run_phase7 = false;

        if !attached {
            let kind = rd32(driver) & 7;
            if kind == 2 || kind == 3 || kind == 4 || kind == 5 {
                // Phase 4: save the goal, refresh through the driver.
                e10 = rdf(this + 0x30);
                e0c = rdf(this + 0x34);
                e30 = rdf(this + 0x38);
                let mut v60 = [0u32; 3];
                lf_checker_rt::callee_thiscall!(2, u32, driver, v60.as_mut_ptr() as u32);
                e60 = f32::from_bits(v60[0]);
                e64 = f32::from_bits(v60[1]);
                e68 = f32::from_bits(v60[2]);
                if rd8(this + 0x54) != 0 && rd8(this + 0x55) == 0 {
                    // Direct to phase 7 with the refreshed vector.
                    ph7_t7 = e68;
                    run_phase7 = true;
                } else {
                    (this as *mut u8).wrapping_byte_add(0x55).write(0);
                    // Phase 5: goal transforms through the entity matrix.
                    let x = rd32(rd32(ctx + D68) + 0x10);
                    if x == 0 {
                        wrf(this + 0x28, add(e68, e30));
                        wrf(this + 0x2c, w3);
                        wrf(this + 0x20, add(e60, e10));
                        wrf(this + 0x24, add(e64, e0c));
                        wrf(ctx + 0xd70, sub(anch_x, e60));
                        wrf(ctx + 0xd74, sub(anch_y, e64));
                        wrf(ctx + 0xd78, sub(anch_z, e68));
                        wrf(ctx + 0xd7c, w3);
                        e60 = 0.0;
                        e64 = 0.0;
                        e68 = 0.0;
                    } else {
                        if rd32(x + MATRIX_SLOT) == 0 {
                            lf_checker_rt::callee_thiscall!(3, u32, x);
                            lf_checker_rt::callee_thiscall!(4, u32, x + 0x10, rd32(x + MATRIX_SLOT));
                        }
                        let m = rd32(x + MATRIX_SLOT);
                        let o0 = add(add(mul(rdf(m + 0x10), e0c), mul(e10, rdf(m))), mul(rdf(m + 0x20), e30));
                        let o1 = add(add(mul(rdf(m + 0x14), e0c), mul(rdf(m + 4), e10)), mul(rdf(m + 0x24), e30));
                        let o2 = add(add(mul(rdf(m + 0x18), e0c), mul(rdf(m + 8), e10)), mul(rdf(m + 0x28), e30));
                        wrf(this + 0x20, o0);
                        wrf(this + 0x24, o1);
                        wrf(this + 0x2c, w3);
                        wrf(this + 0x28, o2);
                        if rd32(x + MATRIX_SLOT) == 0 {
                            lf_checker_rt::callee_thiscall!(5, u32, x);
                            lf_checker_rt::callee_thiscall!(6, u32, x + 0x10, rd32(x + MATRIX_SLOT));
                        }
                        let m = rd32(x + MATRIX_SLOT);
                        wrf(this + 0x20, add(rdf(this + 0x20), rdf(m + 0x30)));
                        wrf(this + 0x24, add(rdf(m + 0x34), rdf(this + 0x24)));
                        wrf(this + 0x28, add(rdf(m + 0x38), rdf(this + 0x28)));
                        if rd32(x + MATRIX_SLOT) == 0 {
                            lf_checker_rt::callee_thiscall!(7, u32, x);
                            lf_checker_rt::callee_thiscall!(8, u32, x + 0x10, rd32(x + MATRIX_SLOT));
                        }
                        let m = rd32(x + MATRIX_SLOT);
                        let dx = sub(anch_x, rdf(m + 0x30));
                        let dy = sub(anch_y, rdf(m + 0x34));
                        let dz = sub(anch_z, rdf(m + 0x38));
                        wrf(ctx + 0xd70, add(add(mul(rdf(m + 4), dy), mul(dx, rdf(m))), mul(rdf(m + 8), dz)));
                        wrf(ctx + 0xd74, add(add(mul(rdf(m + 0x14), dy), mul(dx, rdf(m + 0x10))), mul(rdf(m + 0x18), dz)));
                        wrf(ctx + 0xd78, add(add(mul(rdf(m + 0x24), dy), mul(dx, rdf(m + 0x20))), mul(rdf(m + 0x28), dz)));
                    }
                    // Phase 6: avoidance direction pick.
                    let dmax = unsafe { lf_checker_rt::global::<f32>(D2MAX_FILE).read() };
                    let vx4 = sub(e0c, e64);
                    let vx5 = sub(e10, e60);
                    let vx2 = sub(e30, e68);
                    let d2 = add(add(mul(vx4, vx4), mul(vx5, vx5)), mul(vx2, vx2));
                    let mut e3c_f = d2.sqrt();
                    let (dir_a, dir_c, dir_b);
                    if dmax > d2 {
                        let driver = rd32(ctx + D68);
                        let mut v70 = [0u32; 3];
                        lf_checker_rt::callee_thiscall!(9, u32, driver, v70.as_mut_ptr() as u32);
                        let ex = f32::from_bits(v70[0]);
                        let ey = f32::from_bits(v70[1]);
                        let ez = f32::from_bits(v70[2]);
                        let x = rd32(driver + 0x10);
                        let (t3, t6, t7) = if x == 0 {
                            (rgf(T7_FILE), rgf(T7_FILE + 4), rgf(T7_FILE + 8))
                        } else {
                            match lf_checker_rt::callee_cdecl!(10, u32, x) {
                                3 => (rgf(T0_FILE), rgf(T0_FILE + 4), rgf(T0_FILE + 8)),
                                2 => (rgf(T5_FILE), rgf(T5_FILE + 4), rgf(T5_FILE + 8)),
                                _ => (rgf(T7_FILE), rgf(T7_FILE + 4), rgf(T7_FILE + 8)),
                            }
                        };
                        dir_a = sub(mul(ey, t7), mul(ez, t6));
                        dir_b = sub(mul(ex, t6), mul(ey, t3));
                        dir_c = sub(mul(ez, t3), mul(ex, t7));
                        let q1 = sub(e10, rdf(ctx + 0xd70));
                        let q0 = sub(e0c, rdf(ctx + 0xd74));
                        e3c_f = add(mul(q0, q0), mul(q1, q1)).sqrt();
                        ph7_t7 = t7;
                    } else {
                        let s = if d2 != 0.0 { div(one, d2.sqrt()) } else { 0.0 };
                        dir_a = mul(vx5, s);
                        dir_c = mul(vx4, s);
                        dir_b = mul(vx2, s);
                        ph7_t7 = e0c;
                    }
                    let mut t = add(
                        add(mul(sub(rdf(ctx + 0xd74), e64), dir_c), mul(sub(rdf(ctx + 0xd70), e60), dir_a)),
                        mul(sub(rdf(ctx + 0xd78), e68), dir_b),
                    );
                    if t > e3c_f {
                        t = e3c_f;
                    }
                    wrf(ctx + 0xd70, mul(dir_a, t));
                    wrf(ctx + 0xd74, mul(dir_c, t));
                    wrf(ctx + 0xd78, mul(dir_b, t));
                    wrf(ctx + 0xd7c, w3);
                    if rd8(ctx + 0x219) == 0 {
                        wrf(ctx + 0xd78, 0.0);
                        wrf(ctx + 0xd74, 0.0);
                        wrf(ctx + 0xd70, 0.0);
                    }
                    run_phase7 = true;
                }
            }
        }
        // Phase 7: second transform while busy (skipped for idle tasks).
        if run_phase7 && rd8(this + 0x54) != 0 {
            let x = rd32(rd32(ctx + D68) + 0x10);
            // (The original spills X to the E+0x3c slot here, but phase 8
            // overwrites it before any read; that dead store is omitted.)
            if x == 0 {
                let driver = rd32(ctx + D68);
                let mut v60 = [0u32; 3];
                lf_checker_rt::callee_thiscall!(15, u32, driver, v60.as_mut_ptr() as u32);
                e60 = f32::from_bits(v60[0]);
                e64 = f32::from_bits(v60[1]);
                e68 = f32::from_bits(v60[2]);
                wrf(this + 0x28, add(rdf(ctx + 0xd78), e68));
                wrf(this + 0x20, add(e60, rdf(ctx + 0xd70)));
                wrf(this + 0x24, add(rdf(ctx + 0xd74), e64));
                wrf(this + 0x2c, w3);
            } else {
                let v6 = add(e60, rdf(ctx + 0xd70));
                let v4 = add(rdf(ctx + 0xd74), e64);
                let v5 = add(rdf(ctx + 0xd78), ph7_t7);
                // (The original saves these three to stack slots and
                // reloads them after the calls because the real callees
                // may clobber vector registers; the stubs preserve them,
                // so the reloads are no-ops and are omitted here.)
                if rd32(x + MATRIX_SLOT) == 0 {
                    lf_checker_rt::callee_thiscall!(11, u32, x);
                    lf_checker_rt::callee_thiscall!(12, u32, x + 0x10, rd32(x + MATRIX_SLOT));
                }
                let m = rd32(x + MATRIX_SLOT);
                let o0 = add(add(mul(rdf(m + 0x10), v4), mul(v6, rdf(m))), mul(rdf(m + 0x20), v5));
                let o1 = add(add(mul(rdf(m + 0x14), v4), mul(rdf(m + 4), v6)), mul(rdf(m + 0x24), v5));
                let o2 = add(add(mul(rdf(m + 0x18), v4), mul(rdf(m + 8), v6)), mul(rdf(m + 0x28), v5));
                wrf(this + 0x20, o0);
                wrf(this + 0x24, o1);
                wrf(this + 0x2c, w3);
                wrf(this + 0x28, o2);
                if rd32(x + MATRIX_SLOT) == 0 {
                    lf_checker_rt::callee_thiscall!(13, u32, x);
                    lf_checker_rt::callee_thiscall!(14, u32, x + 0x10, rd32(x + MATRIX_SLOT));
                }
                let m = rd32(x + MATRIX_SLOT);
                wrf(this + 0x20, add(rdf(this + 0x20), rdf(m + 0x30)));
                wrf(this + 0x24, add(rdf(m + 0x34), rdf(this + 0x24)));
                wrf(this + 0x28, add(rdf(m + 0x38), rdf(this + 0x28)));
            }
        }
        // Phase 8: heading check; its answer steers what follows.
        let mut result = 0u8;
        let mut e3c = 0u32;
        let ec: i32;
        if rd8(ctx + 0x219) != 0 && rd8(this + 0x54) == 0 {
            let ans = lf_checker_rt::callee_thiscall!(16, u32, this, ctx) as i32;
            ec = ans;
            e3c = ans as u32;
            if ans == 0 {
                result = 1;
            } else if rd32(this + 0x50) == 0 {
                if ans > 0 {
                    result = 1;
                }
            } else if rd32(this + 0x50) == 1 {
                if ans < 0 {
                    result = 1;
                }
            }
        } else {
            ec = 0;
        }
        // Phase 9: calibration rotation and driver reconcile.
        if rd32(driver) & 7 == 1 {
            let q = add(add(mul(e54, e54), mul(e50, e50)), mul(e58, e58));
            let s = if q != 0.0 { div(one, q.sqrt()) } else { 0.0 };
            e10 = mul(e50, s);
            e50 = e10;
            e0c = mul(e54, s);
            e54 = e0c;
            e30 = mul(e58, s);
            e58 = e30;
            if ec < 0 {
                let f1 = f64ans(lf_checker_rt::callee_stdcall!(17, u32,)) as f32;
                let f2 = f64ans(lf_checker_rt::callee_stdcall!(18, u32,)) as f32;
                e50 = sub(mul(e10, f2), mul(e0c, f1));
                e54 = add(mul(e0c, f2), mul(e10, f1));
                let f3 = f64ans(lf_checker_rt::callee_stdcall!(19, u32,)) as f32;
                let f4 = f64ans(lf_checker_rt::callee_stdcall!(23, u32,)) as f32;
                let ne0c = add(mul(e0c, f4), mul(e10, f3));
                let ne10 = sub(mul(e10, f4), mul(e0c, f3));
                e0c = ne0c;
                e10 = ne10;
            } else if ec > 0 {
                let f1 = f64ans(lf_checker_rt::callee_stdcall!(20, u32,)) as f32;
                let f2 = f64ans(lf_checker_rt::callee_stdcall!(21, u32,)) as f32;
                e50 = sub(mul(e10, f2), mul(e0c, f1));
                e54 = add(mul(e0c, f2), mul(e10, f1));
                let f3 = f64ans(lf_checker_rt::callee_stdcall!(22, u32,)) as f32;
                let f4 = f64ans(lf_checker_rt::callee_stdcall!(23, u32,)) as f32;
                let ne0c = add(mul(e0c, f4), mul(e10, f3));
                let ne10 = sub(mul(e10, f4), mul(e0c, f3));
                e0c = ne0c;
                e10 = ne10;
            }
            let v50 = [e50.to_bits(), e54.to_bits(), e58.to_bits()];
            lf_checker_rt::callee_cdecl!(24, u32, driver, ctx, v50.as_ptr() as u32, this + 0x20, 0u32);
            // The scratch slots read here are the checker's zero fill.
            let s70 = add(0.0, anch_x);
            let s74 = add(anch_y, 0.0);
            let s78 = add(anch_z, 0.0);
            let v70b = [s70.to_bits(), s74.to_bits(), s78.to_bits()];
            lf_checker_rt::callee_thiscall!(25, u32, ctx, v70b.as_ptr() as u32);
        }
        // Phases 10-11 run only on the attached path; otherwise the tick
        // ends here with the validation byte.
        if attached {
            if rd32(this + 8) != 0 {
                let mut f70 = anch_x;
                let mut f74 = anch_y;
                let mut f78 = anch_z;
                let b38 = rd32(ctx + 0x38);
                if b38 != 0 {
                    f78 = rdf(b38 + 0x48);
                }
                let v70c = [f70.to_bits(), f74.to_bits(), f78.to_bits()];
                let al = lf_checker_rt::callee_cdecl!(26, u32, ctx, v70c.as_ptr() as u32, 1u32, 0xc479c000u32, 1u32) as u8;
                if al == 0 {
                    return 1;
                }
                lf_checker_rt::callee_thiscall!(27, u32, rd32(ctx + D68), ctx);
                let e = e3c as i32;
                let cont = if rd8(this + 0x54) != 0 {
                    true
                } else if e > 0 {
                    rd8(ctx + 0xe95) != 0
                } else if e == 0 {
                    false
                } else {
                    rd8(ctx + 0xe94) != 0
                };
                if !cont {
                    return 1;
                }
            }
            let mut v60 = [0u32; 3];
            lf_checker_rt::callee_thiscall!(28, u32, driver, v60.as_mut_ptr() as u32);
            e60 = f32::from_bits(v60[0]);
            e64 = f32::from_bits(v60[1]);
            e68 = f32::from_bits(v60[2]);
            lf_checker_rt::callee_thiscall!(29, u32, driver, this + 0x20, 0u32);
            let g = rgf(G50_FILE);
            match rd32(this + 0x50) {
                0 => {
                    let f1 = f64ans(lf_checker_rt::callee_stdcall!(30, u32,)) as f32;
                    let f2 = f64ans(lf_checker_rt::callee_stdcall!(31, u32,)) as f32;
                    let t0 = mul(e64, f1);
                    let t1 = mul(e60, f1);
                    let d4 = sub(mul(e60, f2), t0);
                    let s5 = add(mul(e64, f2), t1);
                    e60 = mul(d4, g);
                    e64 = mul(s5, g);
                    e68 = mul(e68, g);
                    wrf(this + 0x20, add(e60, rdf(this + 0x20)));
                    wrf(this + 0x24, add(e64, rdf(this + 0x24)));
                    wrf(this + 0x28, add(e68, rdf(this + 0x28)));
                }
                1 => {
                    let f1 = f64ans(lf_checker_rt::callee_stdcall!(32, u32,)) as f32;
                    let f2 = f64ans(lf_checker_rt::callee_stdcall!(31, u32,)) as f32;
                    let t0 = mul(e64, f1);
                    let t1 = mul(e60, f1);
                    let d4 = sub(mul(e60, f2), t0);
                    let s5 = add(mul(e64, f2), t1);
                    e60 = mul(d4, g);
                    e64 = mul(s5, g);
                    e68 = mul(e68, g);
                    wrf(this + 0x20, add(e60, rdf(this + 0x20)));
                    wrf(this + 0x24, add(e64, rdf(this + 0x24)));
                    wrf(this + 0x28, add(e68, rdf(this + 0x28)));
                }
                _ => {}
            }
            let d: f32 = lf_checker_rt::callee_cdecl!(33, f32, rd32(this + 0x20), rd32(this + 0x24), anch_x.to_bits(), anch_y.to_bits());
            lf_checker_rt::callee_thiscall!(34, u32, ctx, d.to_bits());
        }
        result as u32
    }
});
