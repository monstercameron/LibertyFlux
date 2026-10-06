// original: 0x00DD3040 UIMontageContainer::vf97

/// Per-item montage visibility/layout commit for the container.
///
/// `this` is the container, `arg` a registry key. If byte `+0x21c` is set
/// the function returns at once (leaving eax untouched, so there is no
/// return channel). Otherwise the key is resolved through the registry
/// (callee 1) and two axis-aligned rectangles are derived from float
/// getters at vtable slots `+0xB8/+0xC0/+0xC8/+0xD0` on the resolved
/// object and on `this` (each getter returns f32 in ST0): each pair forms
/// `center -/+ half-extent * 0.5`. The two boxes merge component-wise
/// with MIXED polarity, read from each `comiss`/`jbe` direction: edges 0
/// and 3 take the maximum, edges 1 and 2 the MINIMUM; a NaN operand keeps
/// the source side (the jump is taken on unordered). The merged box is
/// checked against the viewport (two BSS integers converted to float and
/// scaled by two BSS floats, each clamped to `[0, 1]`, NaN surviving):
/// all four comparisons are strict (`w > x7`, `x6 > w`, `h > x5`,
/// `x4 > h`), so any NaN fails the gate.
/// A failed gate, or a nonzero low byte from the gatekeeper (callee 14
/// on child `+0x1e0`), takes the fail path through the global listener
/// object (`G_LISTENER`): word `+0x204` selects callee 24, else word
/// `+0x200` selects callee 25, then slot `+0x1a4` on `this`.
/// A passed gate verifies the source tag (slot `+0x0` against callee 11)
/// and, on match with a zero low byte from slot `+0x1c`, notifies
/// (callee 12); then slot `+0x1c0` on `this`, a second registry lookup,
/// slot `+0x4c` into slot `+0x1e4` on the child, an optional post
/// (callee 18) skipped on a negative (SIGNED `js`) index answer, slot
/// `+0x1e4` with `arg` (stored to `+0x214` with `arg` to `+0x210`),
/// callee 19, the index fanned out through the global listener
/// (`+0x208`), slot `+0x21c` into a float callee (callee 20, ST0),
/// callee 21 (thiscall/2: string below the float bits), callee 22, slot
/// `+0x21c` again, and an E9 tail (thiscall/1: `TAIL_THIS`, second
/// `+0x21c` answer) which the rewrite performs as a normal ctable call.
/// Float operation order is the original's, pinned through `black_box`
/// helpers; the `(an instruction of the original)` tail-argument store hits the dead
/// incoming-arg slot, so the stack check is off (observed instead as the
/// tail call's compared argument).
///
/// Original: 0x00DD3040 (thiscall, one stack word, void).
lf_checker_rt::export!(thiscall, rw_00dd3040(this: u32, arg: u32) -> u32 {
    unsafe {
        const REGISTRY: u32 = 0x0198_1A4C;
        const STR_TAG: u32 = 0x00EF_AD68;
        const STR_NOTIFY: u32 = 0x00EF_AD78;
        const NOTIFY_THIS: u32 = 0x0117_6888;
        const STR_F32: u32 = 0x00EF_AD90;
        const TAIL_THIS: u32 = 0x019D_2F18;
        const HALF_BITS: u32 = 0x3F00_0000;
        const ONE_BITS: u32 = 0x3F80_0000;
        const G_LISTENER: u32 = 0x018B_6C8C;
        const G_WINT: u32 = 0x018B_7A80;
        const G_HINT: u32 = 0x018B_7A8C;
        const G_WF: u32 = 0x017A_CCE8;
        const G_HF: u32 = 0x017A_CCF0;
        const C_LOOKUP: u32 = 1;
        const C_TAGID: u32 = 11;
        const C_NOTIFY: u32 = 12;
        const C_GATE: u32 = 14;
        const C_POST: u32 = 18;
        const C_TAIL1: u32 = 19;
        const C_F32: u32 = 20;
        const C_CFG2: u32 = 21;
        const C_REL: u32 = 22;
        const C_J: u32 = 24;
        const C_K: u32 = 25;
        const C_TAIL: u32 = 27;
        const S_B8: u32 = 0xB8;
        const S_C0: u32 = 0xC0;
        const S_C8: u32 = 0xC8;
        const S_D0: u32 = 0xD0;
        const S_TAG: u32 = 0x0;
        const S_SUB: u32 = 0x1C;
        const S_SELF4C: u32 = 0x4c;
        const S_DISMISS: u32 = 0x1C0;
        const S_SINK: u32 = 0x1E4;
        const S_DONE: u32 = 0x21C;
        const S_FAIL: u32 = 0x1A4;

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
        unsafe fn fail(this: u32) -> u32 {
            unsafe {
                let g: u32 = lf_checker_rt::global::<u32>(G_LISTENER).read();
                if rd32(g.wrapping_add(0x204)) != 0 {
                    lf_checker_rt::callee_thiscall!(C_J, u32, g);
                    icall0(this, S_FAIL);
                } else {
                    if rd32(g.wrapping_add(0x200)) != 0 {
                        lf_checker_rt::callee_thiscall!(C_K, u32, g);
                    }
                    icall0(this, S_FAIL);
                }
                0
            }
        }

        if ((this.wrapping_add(0x21c)) as *const u8).read() != 0 {
            return 0;
        }
        let half = f32::from_bits(HALF_BITS);
        let one = f32::from_bits(ONE_BITS);
        let src = lf_checker_rt::callee_thiscall!(
            C_LOOKUP, u32, lf_checker_rt::relocated(REGISTRY), arg);
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
        let list = rd32(this.wrapping_add(0x1e0));
        if !(w > x7) || !(x6 > w) || !(h > x5) || !(x4 > h) {
            return fail(this);
        }
        if (lf_checker_rt::callee_thiscall!(C_GATE, u32, list) & 0xff) != 0 {
            return fail(this);
        }
        let tag = icall0(src, S_TAG);
        let want = lf_checker_rt::callee_cdecl!(
            C_TAGID, u32, lf_checker_rt::relocated(STR_TAG));
        if tag == want && (icall0(src, S_SUB) & 0xff) == 0 {
            lf_checker_rt::callee_thiscall!(
                C_NOTIFY, u32, lf_checker_rt::relocated(NOTIFY_THIS),
                lf_checker_rt::relocated(STR_NOTIFY));
        }
        icall0(this, S_DISMISS);
        let src2 = lf_checker_rt::callee_thiscall!(
            C_LOOKUP, u32, lf_checker_rt::relocated(REGISTRY), arg);
        let r = icall0(src2, S_SELF4C);
        let e = icall1(list, S_SINK, r);
        if (e as i32) >= 0 {
            lf_checker_rt::callee_thiscall!(C_POST, u32, list, e, 1);
        }
        let e2 = icall1(list, S_SINK, arg);
        let o1ec = rd32(this.wrapping_add(0x1ec));
        wr32(this.wrapping_add(0x214), e2);
        wr32(this.wrapping_add(0x210), arg);
        lf_checker_rt::callee_thiscall!(C_TAIL1, u32, o1ec, e2);
        let gptr: u32 = lf_checker_rt::global::<u32>(G_LISTENER).read();
        wr32(gptr.wrapping_add(0x208), rd32(this.wrapping_add(0x214)));
        let hold = rd32(rd32(this.wrapping_add(0x1e0)).wrapping_add(0x1e0));
        let t = icall0(hold, S_DONE);
        let f: f32 = lf_checker_rt::callee_thiscall!(
            C_F32, f32, rd32(this.wrapping_add(0x1f8)), t);
        let cfg = lf_checker_rt::callee_thiscall!(
            C_CFG2, u32, lf_checker_rt::relocated(REGISTRY),
            lf_checker_rt::relocated(STR_F32), f.to_bits());
        lf_checker_rt::callee_thiscall!(C_REL, u32, cfg);
        let t2 = icall0(hold, S_DONE);
        lf_checker_rt::callee_thiscall!(
            C_TAIL, u32, lf_checker_rt::relocated(TAIL_THIS), t2);
        0
    }
});
