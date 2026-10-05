// original: 0x00a2a180 pedtask_accumulator_update

/// Update a ped-task accumulator through mode gates and float tables.
///
/// `this` is the task object, `idx` selects a row of a global float table
/// (row bytes are `idx * 32`) and `flag` steers the update path. Returns an
/// f32 (ST0): 0.0 on the early exits, otherwise a computed value.
///
/// Behaviour: a null (or `+0x70`-wrapping) record word at `[this+0x228]`
/// and a null answer from the resolve callee each return 0.0. Otherwise a
/// mode bit at `[P+0x3d0]` (`P` is the record base `+0x70`) is cleared when
/// the limit at `[P+0x3b4]` exceeds its constant, and a run flag is built
/// from the global mode word, task words and gate calls. The non-zero-`flag`
/// path probes bytes from the classify callee into an add/max store, a
/// scaled-subtract/max store, or a positive-gated scaled-subtract store;
/// the zero-`flag` path stores a table value or zero. The tail compares the
/// stored accumulator against the table, divides by it, and either returns
/// through the confirm callee (scaling the over-one part by another table
/// value) or through a fallback that may store 1.0 and refresh the
/// accumulator from the table. All float comparisons are the original's
/// unsigned conditional jumps over ordered comparisons, replicated with
/// forms that agree on unordered inputs too.
///
/// The original uses its incoming argument slots as float temporaries; the
/// rewrite keeps them in locals (their values are all observed through
/// returns and branches). On the scaled-subtract path the tail's temporaries
/// keep the incoming argument bits; the contract's coupled stub answers
/// cannot steer that path into the fallback read of the first temporary, so
/// that combination is exercised for its addressing only.
///
/// Original: 0x00a2a180 (thiscall, two stack words; returns f32 in ST0).
#[allow(clippy::all)]
#[allow(unsafe_code)]
unsafe fn run_00a2a180(this: u32, idx: u32, flag: u32, mutate_min: bool) -> f32 {
    unsafe {
        const RESOLVE_CALLEE: u32 = 1;
        const GATE_CALLEE: u32 = 2;
        const CLASSIFY_CALLEE: u32 = 3;
        const CONFIRM_CALLEE: u32 = 4;
        const G_CMP0: u32 = 0xfe8628;
        const G_ONE: u32 = 0xfe88e8;
        const G_MUL: u32 = 0xfe8b68;
        const G_SUB: u32 = 0xfe870c;
        const G_DT: u32 = 0x11735bc;
        const G_MODE: u32 = 0x11d6fd4;
        const G_FLAG: u32 = 0x103ce4a;
        const G_TAB: u32 = 0x103c7c8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read() }
        }
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits(g32(va)) }
        }
        #[inline(always)]
        unsafe fn g8(va: u32) -> u8 {
            unsafe { lf_checker_rt::global::<u8>(va).read() }
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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        // Record base at each site, exactly like the original's repeated
        // null checks (the record word is stable under the scripted callee).
        #[inline(always)]
        unsafe fn base(this: u32) -> u32 {
            unsafe {
                let rec = rd32(this.wrapping_add(0x228));
                if rec != 0 {
                    rec.wrapping_add(0x70)
                } else {
                    0
                }
            }
        }

        let rec0 = rd32(this.wrapping_add(0x228));
        if rec0 == 0 || rec0.wrapping_add(0x70) == 0 {
            return 0.0;
        }
        let resolved: u32 = lf_checker_rt::callee_thiscall!(RESOLVE_CALLEE, u32, this);
        if resolved == 0 {
            return 0.0;
        }
        let p = base(this);
        if rd32(p.wrapping_add(0x3d0)) & 0x1000 != 0
            && rdf(p.wrapping_add(0x3b4)) > gf(G_CMP0)
        {
            wr32(
                p.wrapping_add(0x3d0),
                rd32(p.wrapping_add(0x3d0)) & 0xffffefff,
            );
        }
        let mut bl: u8 =
            if (g32(G_MODE) as i32) >= 2 && rd32(this.wrapping_add(0x2a0)) & 0x180000 != 0 {
                1
            } else {
                0
            };
        let mut via_gate = true;
        let p = base(this);
        if rd8(p.wrapping_add(0x414)) != 0 {
            bl = 0;
            via_gate = false;
        } else if g8(G_FLAG) != 0 && rd32(this.wrapping_add(0x24)) & 0x8000000 != 0 {
            let gate: u32 =
                lf_checker_rt::callee_thiscall!(GATE_CALLEE, u32, this.wrapping_add(0x80));
            if gate as u8 == 0 && bl == 0 {
                via_gate = false;
            }
        }
        if via_gate {
            let p = base(this);
            bl = if rd32(p.wrapping_add(0x3d0)) & 0x1000 == 0 {
                1
            } else {
                0
            };
        }

        let t = idx.wrapping_shl(5);
        // (save14, save18) temporaries and the tail's x1. The
        // scaled-subtract path skips the zeroing stores, so its temporaries
        // keep the incoming argument bits.
        let (mut save14, mut save18, tail_x1);
        if flag as u8 != 0 {
            let a: u32 = lf_checker_rt::callee_thiscall!(CLASSIFY_CALLEE, u32, resolved);
            let cl = rd8(a.wrapping_add(6)) ^ rd8(a.wrapping_add(4));
            if cl > 0x7f && {
                let al = rd8(a.wrapping_add(7)) ^ rd8(a.wrapping_add(4));
                al <= 0x7f
            } {
                // Add/max store.
                let p = base(this);
                let x0 = add(gf(G_TAB.wrapping_add(t)), rdf(p.wrapping_add(0x3b8)));
                let x1in = gf(G_TAB.wrapping_add(t).wrapping_add(0x10));
                // MUTANT (mut_00a2a180): store the max instead of the min.
                let x1 = if mutate_min {
                    if x0 > x1in { x0 } else { x1in }
                } else if x0 > x1in {
                    x1in
                } else {
                    x0
                };
                wr32(p.wrapping_add(0x3b8), x1.to_bits());
                save14 = 0.0;
                save18 = 0.0;
                tail_x1 = 0.0;
            } else {
                let a: u32 = lf_checker_rt::callee_thiscall!(CLASSIFY_CALLEE, u32, resolved);
                let cl = rd8(a.wrapping_add(6)) ^ rd8(a.wrapping_add(4));
                if cl > 0x7f {
                    // Scaled-subtract/max store.
                    let p = base(this);
                    let x1 = rdf(p.wrapping_add(0x3b8));
                    let x3 = gf(G_ONE);
                    let x0 = mul(
                        mul(gf(G_TAB.wrapping_add(t).wrapping_add(0x04)), gf(G_MUL)),
                        gf(G_DT),
                    );
                    let x1 = sub(x1, x0);
                    let x1 = if x3 > x1 { x1 } else { x3 };
                    wr32(p.wrapping_add(0x3b8), x1.to_bits());
                    save14 = f32::from_bits(idx);
                    save18 = f32::from_bits(flag);
                    tail_x1 = 0.0;
                } else {
                    // Positive-gated scaled-subtract store.
                    let p = base(this);
                    let x0 = rdf(p.wrapping_add(0x3b8));
                    if x0 > 0.0 {
                        let p = base(this);
                        let x2 = rdf(p.wrapping_add(0x3b8));
                        let x0 = mul(
                            mul(gf(G_TAB.wrapping_add(t).wrapping_add(0x08)), gf(G_MUL)),
                            gf(G_DT),
                        );
                        let x2 = sub(x2, x0);
                        let x2 = if 0.0 > x2 { 0.0 } else { x2 };
                        wr32(p.wrapping_add(0x3b8), x2.to_bits());
                    }
                    save14 = 0.0;
                    save18 = 0.0;
                    tail_x1 = 0.0;
                }
            }
        } else {
            let a: u32 = lf_checker_rt::callee_thiscall!(CLASSIFY_CALLEE, u32, resolved);
            let cl = rd8(a.wrapping_add(6)) ^ rd8(a.wrapping_add(4));
            let x0 = if cl > 0x7f {
                gf(G_TAB.wrapping_add(t).wrapping_add(0x10))
            } else {
                0.0
            };
            let p = base(this);
            wr32(p.wrapping_add(0x3b8), x0.to_bits());
            save14 = 0.0;
            save18 = 0.0;
            tail_x1 = 0.0;
        }

        // Tail.
        let x3 = gf(G_ONE);
        let p = base(this);
        let x0 = rdf(p.wrapping_add(0x3b8));
        let x2 = gf(G_TAB.wrapping_add(t).wrapping_add(0x0c));
        // `jb`: taken on ordered-less or unordered alike.
        let x1h;
        if !(x0 >= x2) {
            let p = base(this);
            let x0 = rdf(p.wrapping_add(0x3b8));
            if !(x0 > tail_x1) {
                return tail2(
                    this, resolved, t, save14, save18, G_TAB, G_ONE, G_SUB,
                    CLASSIFY_CALLEE,
                );
            }
            x1h = gf(G_TAB.wrapping_add(t).wrapping_add(0x14));
        } else {
            x1h = gf(G_TAB.wrapping_add(t).wrapping_add(0x18));
        }
        let p = base(this);
        let mut x0 = rdf(p.wrapping_add(0x3b8));
        x0 = div(x0, x2);
        save14 = x0;
        save18 = x0;
        if !(x0 > x3) {
            return tail2(
                this, resolved, t, save14, save18, G_TAB, G_ONE, G_SUB,
                CLASSIFY_CALLEE,
            );
        }
        if bl == 0 {
            return tail2(
                this, resolved, t, save14, save18, G_TAB, G_ONE, G_SUB,
                CLASSIFY_CALLEE,
            );
        }
        let confirm: u32 =
            lf_checker_rt::callee_thiscall!(CONFIRM_CALLEE, u32, this, 1, x1h.to_bits());
        if confirm as u8 == 0 {
            let p = base(this);
            wr32(
                p.wrapping_add(0x3d0),
                rd32(p.wrapping_add(0x3d0)) | 0x1000,
            );
            return save18;
        }
        let mut x0 = save14;
        let x2b = gf(G_ONE);
        x0 = sub(x0, x2b);
        let x1r = if 0.0 > x0 { 0.0 } else { save18 = x0; x0 };
        let mut x0 = gf(G_TAB.wrapping_add(t).wrapping_add(0x1c));
        x0 = mul(x0, x1r);
        x0 = add(x0, x2b);
        return x0;

        #[inline(always)]
        #[allow(clippy::too_many_arguments)]
        unsafe fn tail2(
            this: u32,
            resolved: u32,
            t: u32,
            save14: f32,
            mut save18: f32,
            g_tab: u32,
            g_one: u32,
            g_sub: u32,
            classify: u32,
        ) -> f32 {
            unsafe {
                #[inline(always)]
                unsafe fn rd32(a: u32) -> u32 {
                    unsafe { (a as *const u32).read_unaligned() }
                }
                #[inline(always)]
                unsafe fn rd8(a: u32) -> u8 {
                    unsafe { (a as *const u8).read() }
                }
                #[inline(always)]
                unsafe fn rdf(a: u32) -> f32 {
                    unsafe { f32::from_bits(rd32(a)) }
                }
                #[inline(always)]
                unsafe fn wr32(a: u32, v: u32) {
                    unsafe { (a as *mut u32).write_unaligned(v) }
                }
                #[inline(always)]
                unsafe fn gf(va: u32) -> f32 {
                    unsafe { f32::from_bits(lf_checker_rt::global::<u32>(va).read()) }
                }
                #[inline(always)]
                fn sub(a: f32, b: f32) -> f32 {
                    core::hint::black_box(a) - core::hint::black_box(b)
                }
                let a: u32 = lf_checker_rt::callee_thiscall!(classify, u32, resolved);
                let cl = rd8(a.wrapping_add(6)) ^ rd8(a.wrapping_add(4));
                if cl > 0x7f {
                    save18 = 1.0;
                    let rec = rd32(this.wrapping_add(0x228));
                    let p = if rec != 0 {
                        rec.wrapping_add(0x70)
                    } else {
                        0
                    };
                    let x0 = rdf(p.wrapping_add(0x3b4));
                    if x0 > 0.0 {
                        let mut x0 = gf(g_tab.wrapping_add(t).wrapping_add(0x0c));
                        x0 = sub(x0, gf(g_sub));
                        wr32(p.wrapping_add(0x3b8), x0.to_bits());
                    }
                    return save18;
                }
                let x0 = save14;
                if x0 > gf(g_one) {
                    save18 = 1.0;
                }
                save18
            }
        }
    }
}

lf_checker_rt::export!(thiscall, rw_00a2a180(this: u32, idx: u32, flag: u32) -> f32 {
    unsafe { run_00a2a180(this, idx, flag, false) }
});

lf_checker_rt::export!(thiscall, mut_00a2a180(this: u32, idx: u32, flag: u32) -> f32 {
    unsafe { run_00a2a180(this, idx, flag, true) }
});
