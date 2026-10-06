// original: 0x00DD37E0 UIMontageContainer::vf94

/// Indexed montage pick/commit for the container.
///
/// `this` is the container, `arg` a registry key; every exit path sets
/// eax, so this is thiscall/1 returning u32. The entry pre-gate compares
/// the key against slot `+0x4c` on child `+0x1e0` (equality, no
/// signedness question): on mismatch the main path runs; on match a
/// short 1c-path runs (slot `+0x1c` low byte must be zero, slot `+0x1d4`
/// must be nonzero, BSS mask bit 0 of `(B^A)&B` must be set) ending in
/// slot `+0x1c0` on `this` whose answer is returned (each failing check
/// returns what it read).
/// The main path resolves the key through the registry (callee 1) and
/// derives two axis-aligned rectangles from float getters at vtable
/// slots `+0xB8/+0xC0/+0xC8/+0xD0` on the resolved object and on `this`
/// (each returns f32 in ST0): each pair forms `center -/+ half-extent *
/// 0.5`. The boxes merge component-wise with MIXED polarity, read from
/// each `comiss`/`jbe` direction: edges 0 and 3 take the maximum, edges
/// 1 and 2 the MINIMUM; a NaN keeps the source side. The merged box is
/// checked against the viewport (two BSS integers converted to float
/// and scaled by two BSS floats, each clamped to `[0, 1]`): all four
/// comparisons are strict (`w > x7`, `x6 > w`, `h > x5`, `x4 > h`), so
/// any NaN fails gate 1. A failed gate skips the mid-section and runs
/// the tag check directly.
/// The mid-section stores the sink answer (slot `+0x1e4` with `arg`) to
/// `+0x204`, then forms `q = (r1 * d) / (mb8 * r2)` with two answers
/// from the integer callee (callee 34, SIGNED conversion) around float
/// getters: `q > 0.5` STRICT continues (`jbe` skips at exactly 0.5 and
/// on NaN) while `q`'s bits are stored to `+0x220` either way. On pass,
/// slot `+0x1d4` is read and the UNSIGNED `jb` index rule applies:
/// `esi <u m` stores `esi + 1` to `+0x204`, else `-1` (a SIGNED twin is
/// the b17 wrong version).
/// The tag section compares slot `+0x0` on the source against callee 11
/// (called with the tag constant) and returns callee 11's answer on
/// mismatch. On match, float block 2 re-runs the same getters (same
/// per-trial answers, so gate 2 agrees with gate 1) and gate 2 failing
/// returns the last getter's bits (the ST0 stub mirrors them to eax).
/// The epilogue calls callee 35 through the global listener object and
/// returns its answer when word `+0x1bc` is null, else calls callee 36
/// and callee 37 on that object and returns callee 37's answer.
/// Float operation order is the original's, pinned through `black_box`
/// helpers; the stores to `+0x204`/`+0x220` and the stack temporaries
/// are compared as heap writes.
///
/// Original: 0x00DD37E0 (thiscall, one stack word, u32 in eax).
lf_checker_rt::export!(thiscall, rw_00dd37e0(this: u32, arg: u32) -> u32 {
    unsafe {
        const REGISTRY: u32 = 0x0198_1A4C;
        const STR_TAG2: u32 = 0x00EF_AD50;
        const HALF_BITS: u32 = 0x3F00_0000;
        const ONE_BITS: u32 = 0x3F80_0000;
        const G_LISTENER: u32 = 0x018B_6C8C;
        const G_WINT: u32 = 0x018B_7A80;
        const G_HINT: u32 = 0x018B_7A8C;
        const G_WF: u32 = 0x017A_CCE8;
        const G_HF: u32 = 0x017A_CCF0;
        const G_MASKA: u32 = 0x018B_7A84;
        const G_MASKB: u32 = 0x018B_7A88;
        const C_LOOKUP: u32 = 1;
        const C_TAGID: u32 = 11;
        const C_SINK: u32 = 13;
        const C_RAND: u32 = 34;
        const C_E1: u32 = 35;
        const C_E2: u32 = 36;
        const C_E3: u32 = 37;
        const S_B8: u32 = 0xB8;
        const S_C0: u32 = 0xC0;
        const S_C8: u32 = 0xC8;
        const S_D0: u32 = 0xD0;
        const S_TAG: u32 = 0x0;
        const S_SUB: u32 = 0x1C;
        const S_SELF4C: u32 = 0x4C;
        const S_PROBE: u32 = 0x1D4;
        const S_SINK: u32 = 0x1E4;
        const S_DISMISS: u32 = 0x1C0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn icall0(obj: u32, off: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + off) as usize);
                f(obj)
            }
        }
        #[inline(always)]
        unsafe fn icall1(obj: u32, off: u32, x: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + off) as usize);
                f(obj, x)
            }
        }
        #[inline(always)]
        unsafe fn fcall0(obj: u32, off: u32) -> f32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> f32 =
                    core::mem::transmute(rd32(rd32(obj) + off) as usize);
                f(obj)
            }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        let list = rd32(this.wrapping_add(0x1e0));
        if icall0(list, S_SELF4C) == arg {
            let p2 = icall0(list, S_SUB);
            if (p2 & 0xff) != 0 {
                return p2;
            }
            let p3 = icall0(list, S_PROBE);
            if p3 == 0 {
                return 0;
            }
            let wa: u32 = lf_checker_rt::global::<u32>(G_MASKA).read();
            let wb: u32 = lf_checker_rt::global::<u32>(G_MASKB).read();
            if ((wb ^ wa) & wb) & 1 == 0 {
                return p3;
            }
            return icall0(this, S_DISMISS);
        }
        let half = f32::from_bits(HALF_BITS);
        let one = f32::from_bits(ONE_BITS);
        let src = lf_checker_rt::callee_thiscall!(
            C_LOOKUP, u32, lf_checker_rt::relocated(REGISTRY), arg);
        // Float block 1 (same shape as vf97).
        let c8_1 = fcall0(src, S_C8);
        let b8_1 = fcall0(src, S_B8);
        let s10 = sub(c8_1, mul(b8_1, half));
        let b8_2 = fcall0(src, S_B8);
        let c8_2 = fcall0(src, S_C8);
        let s18 = add(c8_2, mul(b8_2, half));
        let d0_1 = fcall0(src, S_D0);
        let c0_1 = fcall0(src, S_C0);
        let s20 = sub(d0_1, mul(c0_1, half));
        let c0_2 = fcall0(src, S_C0);
        let d0_2 = fcall0(src, S_D0);
        let s28 = add(d0_2, mul(c0_2, half));
        let c8t_1 = fcall0(this, S_C8);
        let b8t_1 = fcall0(this, S_B8);
        let s14 = sub(c8t_1, mul(b8t_1, half));
        let b8t_2 = fcall0(this, S_B8);
        let c8t_2 = fcall0(this, S_C8);
        let s1c = add(c8t_2, mul(b8t_2, half));
        let d0t_1 = fcall0(this, S_D0);
        let c0t_1 = fcall0(this, S_C0);
        let s24 = sub(d0t_1, mul(c0t_1, half));
        let c0t_2 = fcall0(this, S_C0);
        let d0t_2 = fcall0(this, S_D0);
        let x1 = add(d0t_2, mul(c0t_2, half));
        let x7 = if s14 > s10 { s14 } else { s10 };
        let x6 = if s18 > s1c { s1c } else { s18 };
        let x5 = if s20 > s24 { s24 } else { s20 };
        let x4 = if x1 > s28 { x1 } else { s28 };
        let h_raw = mul(
            lf_checker_rt::global::<i32>(G_HINT).read() as f32,
            f32::from_bits(lf_checker_rt::global::<u32>(G_HF).read()));
        let h = if 0.0 > h_raw {
            0.0
        } else if h_raw > one {
            one
        } else {
            h_raw
        };
        let w_raw = mul(
            lf_checker_rt::global::<i32>(G_WINT).read() as f32,
            f32::from_bits(lf_checker_rt::global::<u32>(G_WF).read()));
        let w = if 0.0 > w_raw {
            0.0
        } else if w_raw > one {
            one
        } else {
            w_raw
        };
        let gate1 = (w > x7) && (x6 > w) && (h > x5) && (x4 > h);
        if gate1 {
            let e = icall1(list, S_SINK, arg);
            wr32(this.wrapping_add(0x204), e);
            let mc8 = fcall0(src, S_C8);
            let mb8 = fcall0(src, S_B8);
            let d = sub(w, sub(mc8, mul(mb8, half)));
            let r1: u32 = lf_checker_rt::callee_cdecl!(C_RAND, u32,);
            let prod = mul((r1 as i32) as f32, d);
            let mb8_2 = fcall0(src, S_B8);
            let r2: u32 = lf_checker_rt::callee_cdecl!(C_RAND, u32,);
            let denom = mul(mb8_2, (r2 as i32) as f32);
            let q = div(prod, denom);
            wr32(this.wrapping_add(0x220), q.to_bits());
            if q > 0.5 {
                let m = icall0(list, S_PROBE);
                let esi = rd32(this.wrapping_add(0x204));
                // UNSIGNED `jb`: below takes index+1, else -1.
                let below = esi < m;
                wr32(
                    this.wrapping_add(0x204),
                    if below { esi.wrapping_add(1) } else { 0xffff_ffff },
                );
            }
        }
        // Tag section (reached from both gate-1 outcomes).
        let tag = icall0(src, S_TAG);
        let want = lf_checker_rt::callee_cdecl!(
            C_TAGID, u32, lf_checker_rt::relocated(STR_TAG2));
        if tag != want {
            return want;
        }
        // Float block 2 (same slots, same per-trial answers as block 1).
        let t8_1 = fcall0(src, S_C8);
        let t8_2 = fcall0(src, S_B8);
        let u10 = sub(t8_1, mul(t8_2, half));
        let t8_3 = fcall0(src, S_B8);
        let t8_4 = fcall0(src, S_C8);
        let u18 = add(t8_4, mul(t8_3, half));
        let t0_1 = fcall0(src, S_D0);
        let t0_2 = fcall0(src, S_C0);
        let u20 = sub(t0_1, mul(t0_2, half));
        let t0_3 = fcall0(src, S_C0);
        let t0_4 = fcall0(src, S_D0);
        let u28 = add(t0_4, mul(t0_3, half));
        let e8_1 = fcall0(this, S_C8);
        let e8_2 = fcall0(this, S_B8);
        let u14 = sub(e8_1, mul(e8_2, half));
        let e8_3 = fcall0(this, S_B8);
        let e8_4 = fcall0(this, S_C8);
        let u1c = add(e8_4, mul(e8_3, half));
        let e0_1 = fcall0(this, S_D0);
        let e0_2 = fcall0(this, S_C0);
        let u24 = sub(e0_1, mul(e0_2, half));
        let e0_3 = fcall0(this, S_C0);
        let e0_4 = fcall0(this, S_D0);
        let ux1 = add(e0_4, mul(e0_3, half));
        let y7 = if u14 > u10 { u14 } else { u10 };
        let y6 = if u18 > u1c { u1c } else { u18 };
        let y5 = if u20 > u24 { u24 } else { u20 };
        let y4 = if ux1 > u28 { ux1 } else { u28 };
        if !((w > y7) && (y6 > w) && (h > y5) && (y4 > h)) {
            // Gate-2 fail: eax holds the last getter's bits (the
            // f32st0 stub mirrors them to eax on both sides).
            return e0_4.to_bits();
        }
        let g: u32 = lf_checker_rt::global::<u32>(G_LISTENER).read();
        let e1 = lf_checker_rt::callee_thiscall!(C_E1, u32, g, 2);
        let o = rd32(this.wrapping_add(0x1bc));
        if o == 0 {
            return e1;
        }
        lf_checker_rt::callee_thiscall!(C_E2, u32, o, 0xe);
        lf_checker_rt::callee_thiscall!(C_E3, u32, o, 1, 1)
    }
});
