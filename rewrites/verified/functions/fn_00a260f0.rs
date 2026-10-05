// original: 0x00a260f0 ped_task_heading_update (proposed)

/// Advance a ped task's heading blend by one tick.
///
/// `this` is the task object, `a1` selects the active path (low byte
/// nonzero) or the decay path, `a2` points at a parameter block whose word
/// at `+0x20` leads to three floats, and `a3` is passed through to a helper.
/// On the active path the routine builds a rate from a configuration float
/// (or zero when any of three mode bytes or the window-iconic query says
/// so), clamps the accumulated weight, queries the helper for a value and
/// three more floats, maps the value through a sine call into a blend
/// factor, optionally relaxes two smoothing fields, folds a shared triple of
/// globals through the factor, and blends three position fields toward a
/// target. On the decay path it shrinks the weight and two recent values
/// toward zero instead. The tail sets the sticky bit of the flag byte at
/// `+0x1a8` whenever the active bit is set.
///
/// Every floating-point operation below is written in the original's operand
/// order with pinned evaluation order, since the results must match
/// bit for bit. Only the low byte of the return value (the final flag byte)
/// is meaningful; the upper bytes are leftover register contents.
///
/// Original: 0x00a260f0 (thiscall, three stack arguments).
lf_checker_rt::export!(thiscall, rw_00a260f0(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const LINK: u32 = 0x12c;
        const FLAG: u32 = 0x1a8;
        const ACTIVE_BIT: u8 = 1;
        const STICKY_BIT: u8 = 2;
        const G_HWND: u32 = 0x017accd8;
        const G_B1: u32 = 0x0105b48f;
        const G_B2: u32 = 0x017ed8d1;
        const G_B3A: u32 = 0x01173590;
        const G_B3B: u32 = 0x01173591;
        const G_RATE: u32 = 0x011735bc;
        const G_T0: u32 = 0x012dd600;
        const G_T1: u32 = 0x012dd604;
        const G_T2: u32 = 0x012dd608;
        const C_PER_MIN: u32 = 0x00fe8b80; // 60.0
        const C_STEP: u32 = 0x00fe8748; // 0.03
        const C_ONE: u32 = 0x00fe88e8; // 1.0
        const C_DEG: u32 = 0x00e81218; // 180.0
        const C_DEG_BASE: u32 = 0x00e8121c; // 270.0
        const C_DEG2RAD: u32 = 0x00fe8728;
        const C_HALF: u32 = 0x00fe8830; // 0.5
        const C_RELAX: u32 = 0x00fe87d0; // 0.2
        const C_SMOOTH: u32 = 0x00fe876c; // 0.05

        #[inline(always)]
        unsafe fn rd32(base: u32, off: u32) -> u32 {
            unsafe { ((base + off) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(base: u32, off: u32, v: u32) {
            unsafe { ((base + off) as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(base: u32, off: u32) -> u8 {
            unsafe { ((base + off) as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(base: u32, off: u32, v: u8) {
            unsafe { ((base + off) as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read_unaligned() }
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
        fn fadd(a: f32, b: f32) -> f32 {
            let x = core::hint::black_box(a);
            let y = core::hint::black_box(b);
            core::hint::black_box(x + y)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            let x = core::hint::black_box(a);
            let y = core::hint::black_box(b);
            core::hint::black_box(x - y)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            let x = core::hint::black_box(a);
            let y = core::hint::black_box(b);
            core::hint::black_box(x * y)
        }
        #[inline(always)]
        unsafe fn fld(base: u32, off: u32) -> f32 {
            unsafe { f32::from_bits(rd32(base, off)) }
        }
        #[inline(always)]
        unsafe fn fst(base: u32, off: u32, v: f32) {
            unsafe { wr32(base, off, v.to_bits()) }
        }

        // Gate: iconic-window query plus three mode bytes force a zero rate.
        let iconic = lf_checker_rt::callee_stdcall!(1, u32, g32(G_HWND));
        let mut gate: u8 = if iconic != 0 {
            1
        } else if g8(G_B1) == 0 {
            0
        } else if g8(G_B2) == 0 {
            0
        } else {
            1
        };
        gate |= g8(G_B3A);
        gate |= g8(G_B3B);
        let mut x0 = if gate == 0 { fmul(gf(G_RATE), gf(C_PER_MIN)) } else { 0.0 };
        let stash0 = x0;
        if (a1 & 0xff) == 0 {
            // Decay path: shrink the weight and recent values toward zero.
            if (rd8(this, FLAG) & STICKY_BIT) != 0 {
                x0 = fmul(x0, gf(C_STEP));
                let mut x1 = fsub(fld(this, 0x1a0), x0);
                if !(x1 > 0.0) {
                    x1 = 0.0;
                }
                x0 = fmul(x1, fld(this, 0x198));
                fst(this, 0x1a0, x1);
                fst(this, 0x198, x0);
                x0 = fmul(fld(this, 0x19c), x1);
                fst(this, 0x19c, x0);
            }
        } else {
            // Active path.
            x0 = fmul(x0, gf(C_STEP));
            let one = gf(C_ONE);
            wr8(this, FLAG, rd8(this, FLAG) | ACTIVE_BIT);
            x0 = fadd(x0, fld(this, 0x1a0));
            if x0 > one {
                x0 = one;
            }
            fst(this, 0x1a0, x0);
            let edi = a2;
            let mut out = [0u32; 3];
            let hv: f32 = lf_checker_rt::callee_thiscall!(
                2, f32, this, edi, a3, out.as_mut_ptr() as u32
            );
            let out0 = f32::from_bits(out[0]);
            let out1 = f32::from_bits(out[1]);
            let out2 = f32::from_bits(out[2]);
            let mut x1 = fmul(hv, gf(C_DEG));
            x0 = fsub(gf(C_DEG_BASE), x1);
            x0 = fmul(x0, gf(C_DEG2RAD));
            // Sine call: argument compared through XMM0, answer in EAX bits.
            let sin_bits = lf_checker_rt::callee_cdecl!(3, u32, x0.to_bits());
            x0 = f32::from_bits(sin_bits);
            let mut x7 = gf(C_HALF);
            let mut x6 = x0;
            x6 = fadd(x6, gf(C_ONE));
            x6 = fmul(x6, x7);
            let stash6 = x6;
            if x6 > 0.0 {
                let rv: f32 = lf_checker_rt::callee_thiscall!(4, f32, this);
                x0 = fld(this, 0x184);
                x1 = fsub(rv, x0);
                x1 = fmul(x1, gf(C_RELAX));
                x1 = fadd(x1, x0);
                fst(this, 0x184, x1);
                x0 = fmul(x1, gf(C_SMOOTH));
                x0 = fmul(x0, stash0);
                x0 = fadd(x0, fld(this, 0x188));
                if !(x1 > x0) {
                    x0 = x1;
                }
                x6 = stash6;
                x7 = gf(C_HALF);
                fst(this, 0x188, x0);
            }
            let param = rd32(edi, 0x20);
            let mut x2 = gf(C_ONE);
            let mut x5 = fsub(fld(param, 0x10), gf(G_T0));
            let mut x3 = fsub(fld(param, 0x14), gf(G_T1));
            let mut x4 = fsub(fld(param, 0x18), gf(G_T2));
            x5 = fmul(x5, x7);
            x3 = fmul(x3, x7);
            x5 = fadd(x5, gf(G_T0));
            x4 = fmul(x4, x7);
            x3 = fadd(x3, gf(G_T1));
            x2 = fsub(x2, x6);
            x4 = fadd(x4, gf(G_T2));
            wr32(lf_checker_rt::relocated(G_T0), 0, x5.to_bits());
            x5 = fmul(x5, x7);
            wr32(lf_checker_rt::relocated(G_T1), 0, x3.to_bits());
            wr32(lf_checker_rt::relocated(G_T2), 0, x4.to_bits());
            let link = rd32(this, LINK);
            x1 = fld(this, 0x18c);
            x0 = fsub(fld(link, 0x1450), x1);
            x5 = fsub(x5, out0);
            x3 = fmul(x3, x7);
            x4 = fmul(x4, x7);
            x0 = fmul(x0, x7);
            x3 = fsub(x3, out1);
            x4 = fsub(x4, out2);
            x0 = fadd(x0, x1);
            x1 = fld(this, 0x190);
            x5 = fmul(x5, x6);
            x3 = fmul(x3, x6);
            fst(this, 0x18c, x0);
            x0 = fsub(fld(link, 0x1450), x1);
            x3 = fadd(x3, out1);
            x5 = fadd(x5, out0);
            x4 = fmul(x4, x6);
            x0 = fmul(x0, gf(C_SMOOTH));
            x4 = fadd(x4, out2);
            x3 = fmul(x3, x7);
            x0 = fadd(x0, x1);
            x1 = fld(this, 0x194);
            x5 = fmul(x5, x7);
            x4 = fmul(x4, x7);
            fst(this, 0x190, x0);
            x0 = fsub(fld(link, 0x1450), x1);
            x0 = fmul(x0, gf(C_SMOOTH));
            x0 = fadd(x0, x1);
            fst(this, 0x194, x0);
            x0 = fld(this, 0x18c);
            x3 = fmul(x3, x0);
            x5 = fmul(x5, x0);
            x4 = fmul(x4, x0);
            x0 = fld(this, 0x144);
            x3 = fmul(x3, x6);
            x5 = fmul(x5, x6);
            x0 = fadd(x0, x3);
            x4 = fmul(x4, x6);
            x5 = fadd(x5, fld(this, 0x140));
            fst(this, 0x144, x0);
            x0 = fld(this, 0x148);
            x0 = fadd(x0, x4);
            fst(this, 0x140, x5);
            fst(this, 0x148, x0);
            x1 = fld(this, 0x1a0);
            x0 = fld(this, 0x194);
            x0 = fmul(x0, x1);
            x0 = fmul(x0, x2);
            fst(this, 0x19c, x0);
            x0 = fld(this, 0x188);
            x0 = fmul(x0, x6);
            x0 = fmul(x0, fld(this, 0x190));
            x0 = fmul(x0, x1);
            fst(this, 0x198, x0);
        }
        let flag = rd8(this, FLAG);
        if (flag & ACTIVE_BIT) != 0 {
            wr8(this, FLAG, flag | STICKY_BIT);
            (flag | STICKY_BIT) as u32
        } else {
            flag as u32
        }
    }
});
