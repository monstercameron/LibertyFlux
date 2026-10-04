// original: 0x00c3f0d0 timing_channels_update (proposed)

/// Update thirteen timing channels and fold their outputs into an object.
///
/// `this` points to a controller with an object pointer at `+0x270`, an
/// optional helper object at `+0x278` and an active flag byte at `+0x27c`.
/// When the helper is present its virtual slot `+0x28` is polled: a `1`
/// answer copies the helper's three floats at `+0x20` through two transform
/// callees. The main path runs only when the flag is set, the object pointer
/// is non-null and a gate callee answers nonzero; otherwise the result is 0.
///
/// The main path clears a global byte, evaluates the thirteen channels by
/// calling the channel evaluator once per channel (stride `0x30`, matching
/// the evaluator's own layout) and accumulates the thirteen results into
/// the object: result triples are added to offset groups `+0x30`, `+0x50`,
/// `+0x5c`/`+0x60` whenever any member is nonzero (an exact not-equal-zero
/// test, NaN counts as taken), one triple goes through a 3x4 matrix
/// accumulation, and the last result is added to `+0x64` and clamped to
/// [0, 1] (NaN stays NaN). A middle block guarded by two globals and a table
/// lookup runs two more callees and a math call. Float operation order is
/// the original's.
///
/// Original: 0x00c3f0d0 (thiscall, no stack words). Returns 0 or 1 in `al`;
/// the upper bits of `eax` are leftovers. The security-cookie calls are
/// mirrored through the checker stub, not reimplemented.
lf_checker_rt::export!(thiscall, rw_00c3f0d0(this: u32) -> u32 {
    unsafe { f0d0_core(this, false) }
});

#[allow(clippy::too_many_lines)]
unsafe fn f0d0_core(this: u32, skip_flag_store: bool) -> u32 {
    unsafe {
        const F_PTR: u32 = 0x270;
        const F_VT: u32 = 0x278;
        const F_FLAG: u32 = 0x27c;
        const VT_SLOT: u32 = 0x28;
        const N_CH: usize = 13;
        const CH_STRIDE: u32 = 0x30;
        const ONE: f32 = 1.0;
        const ZERO: f32 = 0.0;
        const SCALE_002: f32 = f32::from_bits(0x3c_a7d7_0a); // 0.02
        const C_VT: u32 = 1;
        const C_A: u32 = 2;
        const C_B: u32 = 3;
        const C_GATE: u32 = 4;
        const C_CH: u32 = 5;
        const C_M: u32 = 6;
        const C_N: u32 = 7;
        const C_T: u32 = 8;
        const C_E: u32 = 9;
        const C_S: u32 = 10;
        const C_U: u32 = 11;
        const C_COOKIE: u32 = 12;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        // scratch mirroring the original's [E+0x10..E+0x40] window: the
        // phase-A triple, the two callee structs and the shared word.
        let mut work = [0u32; 12];

        // Phase A: optional helper transform.
        let vt_obj = rd32(this + F_VT);
        if vt_obj != 0 {
            let vt = rd32(vt_obj);
            let poll: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(vt + VT_SLOT) as usize);
            if poll(vt_obj) == 1 {
                work[0] = rd32(vt_obj + 0x20);
                work[1] = rd32(vt_obj + 0x24);
                work[2] = rd32(vt_obj + 0x28);
                let wp = work.as_mut_ptr() as u32;
                lf_checker_rt::callee_thiscall!(C_A, u32, vt_obj, wp);
                lf_checker_rt::callee_thiscall!(C_B, u32, rd32(this + F_PTR), wp);
            }
        }

        // Phase B: gate. Any failure exits with 0.
        let field = this + F_PTR;
        let gated = rd8(this + F_FLAG) != 0 && rd32(this + F_PTR) != 0;
        if gated {
            let gate: u32 = lf_checker_rt::callee_thiscall!(C_GATE, u32, this);
            if gate as u8 != 0 {
                // Main path.
                if !skip_flag_store {
                    lf_checker_rt::global::<u8>(0x103f425).write(0);
                }
                let mut res = [ZERO; N_CH];
                let mut buf_lo = [ZERO; N_CH];
                let mut buf_hi = [ZERO; N_CH];
                for (k, slot) in res.iter_mut().enumerate() {
                    let ch = this + (k as u32) * CH_STRIDE;
                    // stack order matches the original: low pointer first.
                    let v: f32 = lf_checker_rt::callee_thiscall!(
                        C_CH,
                        f32,
                        ch,
                        core::ptr::addr_of_mut!(buf_lo[k]) as u32,
                        core::ptr::addr_of_mut!(buf_hi[k]) as u32
                    );
                    *slot = v;
                }
                let r = res;
                // Group 1: results 0..2.
                if r[0] != ZERO || r[1] != ZERO || r[2] != ZERO {
                    let obj = rd32(field);
                    let got: u32 = lf_checker_rt::callee_thiscall!(
                        C_M,
                        u32,
                        obj,
                        work.as_mut_ptr() as u32,
                        lf_checker_rt::relocated(0xec8658)
                    );
                    let s_a = add(r[1], rdf(got + 4));
                    let s_b = add(r[0], rdf(got));
                    let s_c = add(r[2], rdf(got + 8));
                    let shared = f32::from_bits(work[11]);
                    work[0] = s_b.to_bits();
                    work[1] = s_a.to_bits();
                    work[2] = s_c.to_bits();
                    work[3] = shared.to_bits();
                    lf_checker_rt::callee_thiscall!(
                        C_N,
                        u32,
                        rd32(field),
                        work.as_mut_ptr() as u32
                    );
                    let obj2 = rd32(field);
                    if obj2 != 0 && obj2 == lf_checker_rt::global::<u32>(0x103e494).read() {
                        let idx = lf_checker_rt::global::<i32>(0x1036f14).read();
                        let base = if idx == -1 {
                            0
                        } else {
                            rd32(lf_checker_rt::relocated(0x11a8808)
                                .wrapping_add((idx as u32).wrapping_mul(4)))
                        };
                        if rd32(base + 0x4c8) == 0
                            && lf_checker_rt::global::<u8>(0x11609f6).read() == 0
                        {
                            work[0] = ONE.to_bits();
                            work[1] = ONE.to_bits();
                            work[2] = ONE.to_bits();
                            let t: u32 = lf_checker_rt::callee_cdecl!(C_T, u32,);
                            if t as u8 == 0 {
                                let ctx = [r[0].to_bits(), r[1].to_bits(), r[2].to_bits()];
                                let v: f32 = lf_checker_rt::callee_thiscall!(
                                    C_E,
                                    f32,
                                    ctx.as_ptr() as u32,
                                    work.as_ptr() as u32
                                );
                                let w: u32 =
                                    lf_checker_rt::callee_cdecl!(C_S, u32, v.to_bits());
                                let scaled = mul(f32::from_bits(w), SCALE_002);
                                lf_checker_rt::callee_cdecl!(C_U, u32, 0x1e, scaled.to_bits());
                            }
                        }
                    }
                }
                // Group 2: results 3..5 accumulate into +0x30..0x3c.
                if r[3] != ZERO || r[4] != ZERO || r[5] != ZERO {
                    let obj = rd32(field);
                    let n34 = add(r[4], rdf(obj + 0x34));
                    let n30 = add(r[3], rdf(obj + 0x30));
                    let n38 = add(r[5], rdf(obj + 0x38));
                    wrf(obj + 0x34, n34);
                    wrf(obj + 0x30, n30);
                    wrf(obj + 0x38, n38);
                    wrf(obj + 0x3c, f32::from_bits(work[11]));
                }
                // Group 3: results 6..8 through the 3x4 accumulation.
                if r[6] != ZERO || r[7] != ZERO || r[8] != ZERO {
                    let obj = rd32(field);
                    let (m00, m01, m02) = (rdf(obj), rdf(obj + 4), rdf(obj + 8));
                    let (m10, m11, m12) = (rdf(obj + 0x10), rdf(obj + 0x14), rdf(obj + 0x18));
                    let (m20, m21, m22) = (rdf(obj + 0x20), rdf(obj + 0x24), rdf(obj + 0x28));
                    let (a30, a34, a38) = (rdf(obj + 0x30), rdf(obj + 0x34), rdf(obj + 0x38));
                    let mut acc_c = mul(r[8], m22);
                    let mut acc_a = mul(r[6], m00);
                    let mut acc_b = mul(r[6], m01);
                    let tmp0 = mul(r[6], m02);
                    acc_a = add(acc_a, a30);
                    acc_b = add(acc_b, a34);
                    let mut acc0 = mul(r[7], m10);
                    let save_c = acc_c;
                    acc_c = add(tmp0, a38);
                    let acc1 = mul(r[7], m11);
                    let tmp2 = mul(r[7], m12);
                    acc_a = add(acc_a, acc0);
                    acc0 = add(acc_c, tmp2);
                    let acc2 = mul(r[8], m20);
                    let acc3 = mul(r[8], m21);
                    acc0 = add(acc0, save_c);
                    acc_b = add(acc_b, acc1);
                    acc_a = add(acc_a, acc2);
                    wrf(obj + 0x38, acc0);
                    let shared = f32::from_bits(work[11]);
                    acc_b = add(acc_b, acc3);
                    wrf(obj + 0x30, acc_a);
                    wrf(obj + 0x3c, shared);
                    wrf(obj + 0x34, acc_b);
                }
                // Group 4: result 9 into +0x50.
                if r[9] != ZERO {
                    let obj = rd32(field);
                    wrf(obj + 0x50, add(r[9], rdf(obj + 0x50)));
                }
                // Group 5: results 10..11 into +0x5c/+0x60 (both if either).
                if r[10] != ZERO || r[11] != ZERO {
                    let obj = rd32(field);
                    wrf(obj + 0x5c, add(r[10], rdf(obj + 0x5c)));
                    let obj = rd32(field);
                    wrf(obj + 0x60, add(r[11], rdf(obj + 0x60)));
                }
                // Group 6: result 12 into +0x64, clamped to [0, 1]. The
                // second test reads the word past the results, which no
                // callee ever writes: uninitialized stack, proven only
                // under the contract's zero stack fill.
                const GARBAGE_SLOT: f32 = ZERO;
                if r[12] != ZERO || GARBAGE_SLOT != ZERO {
                    let obj = rd32(field);
                    let x = add(rdf(obj + 0x64), r[12]);
                    let c = if x < ZERO {
                        ZERO
                    } else if x > ONE {
                        ONE
                    } else {
                        x
                    };
                    wrf(obj + 0x64, c);
                }
                lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
                return 1;
            }
            wr8(this + F_FLAG, 0);
        }
        lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
        0
    }
}
