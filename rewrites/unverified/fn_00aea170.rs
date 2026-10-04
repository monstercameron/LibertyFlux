// original: 0x00aea170 mainloop_timing_update (proposed)

//! Rewrite of the mainloop-timing update (original 0x00AEA170).
//!
//! Two-argument cdecl function over a global state object: it bumps a 16-bit
//! call counter (resetting through a helper at wrap), maintains flag bits,
//! builds a four-word float block selected by the mode byte, and runs it
//! through a chain of float stages (recentering, rescaling, angle sort,
//! neighbor filtering, output scaling) driven by six scripted callees plus
//! the CRT cookie check. The straight-line SSE bodies are mechanically
//! translations that keep exact operand order and constant bits; the
//! surrounding skeleton keeps the control flow, calls and memory interface.
//!
//! Conventions: `fr` models the original's aligned frame in E-basis words
//! (zero-filled like the checker's `stack_fill: 0`); `x0`..`x7` are the low
//! lanes of the vector registers (the only lanes this function ever stores
//! or compares, except for one 128-bit block copy written out explicitly);
//! `r_*` are the integer registers. Float operations preserve the original's
//! operand order through pinning helpers. Branches after floating compares
//! use unordered-correct predicates (a not-a-number takes `jb`/`jbe`, never
//! `ja`/`jae`/`je`-negations), matching `comiss` flag semantics bit for bit.
lf_checker_rt::export!(cdecl, rw_00aea170(arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const GLOBAL_OBJ: u32 = 0x12FB1B8;
        const COUNTER: u32 = 0x11A8908;
        const ZERO_B: u32 = 0x1593BB6;
        const OBJ_FLAGS: u32 = 0x19;
        const OBJ_SEQ: u32 = 0x38;
        const ESI_BIAS: u32 = 0xB0;
        const SIGN_BIT: u32 = 0x8000_0000;
        // .rdata scalar constants read by the original (exact low-lane bits).
        const C_RMIN: u32 = 0x3c23d70a; // 0.009999999776482582
        const C_TWO_PI: u32 = 0x40c90fdb; // 6.2831854820251465
        const C_S1: u32 = 0x3ca3d70a; // 0.019999999552965164
        const C_S1B: u32 = 0x42700000; // 60.0
        const C_S2: u32 = 0x3ba3d70a; // 0.004999999888241291
        const C_S2B: u32 = 0x41700000; // 15.0
        const C_ABS: u32 = 0x7fffffff; // absolute-value mask

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
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
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        fn fneg_bits(b: u32) -> u32 {
            b ^ SIGN_BIT
        }
        #[inline(always)]
        fn fadd(a: u32, b: u32) -> u32 {
            (core::hint::black_box(f32::from_bits(a)) + core::hint::black_box(f32::from_bits(b))).to_bits()
        }
        #[inline(always)]
        fn fsub(a: u32, b: u32) -> u32 {
            (core::hint::black_box(f32::from_bits(a)) - core::hint::black_box(f32::from_bits(b))).to_bits()
        }
        #[inline(always)]
        fn fmul(a: u32, b: u32) -> u32 {
            (core::hint::black_box(f32::from_bits(a)) * core::hint::black_box(f32::from_bits(b))).to_bits()
        }
        #[inline(always)]
        fn fdiv(a: u32, b: u32) -> u32 {
            (core::hint::black_box(f32::from_bits(a)) / core::hint::black_box(f32::from_bits(b))).to_bits()
        }
        #[inline(always)]
        fn fsqrt(a: u32) -> u32 {
            core::hint::black_box(f32::from_bits(a)).sqrt().to_bits()
        }
        #[inline(always)]
        fn fr_addr(fr: &[u32; 0x110], off: usize) -> u32 {
            (&fr[off / 4] as *const u32) as u32
        }
        #[inline(always)]
        fn is_nan(b: u32) -> bool {
            f32::from_bits(b).is_nan()
        }
        /// `comiss a, b` + `jb`: taken when unordered or a < b.
        #[inline(always)]
        fn f_jb(a: u32, b: u32) -> bool {
            is_nan(a) || is_nan(b) || f32::from_bits(a) < f32::from_bits(b)
        }
        /// `comiss a, b` + `jbe`: taken when unordered or a <= b.
        #[inline(always)]
        fn f_jbe(a: u32, b: u32) -> bool {
            is_nan(a) || is_nan(b) || f32::from_bits(a) <= f32::from_bits(b)
        }

        let mut fr = [0u32; 0x110];
        let (mut x0, mut x1, mut x2, mut x3, mut x4, mut x5, mut x6, mut x7) =
            (0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        let (mut r_eax, mut r_ecx, mut r_edx, mut _r_ebx, mut r_esi, mut r_edi) =
            (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        // Entry: call counter with wrap reset through callee 1.
        let mut global = rd32(lf_checker_rt::relocated(GLOBAL_OBJ));
        r_esi = global.wrapping_add(ESI_BIAS);
        let ctr = lf_checker_rt::relocated(COUNTER);
        let ax = rd16(ctr);
        let ax = if ax >= 0xffff {
            let _ = lf_checker_rt::callee_cdecl!(1, u32,);
            global = rd32(lf_checker_rt::relocated(GLOBAL_OBJ));
            1u16
        } else {
            ax.wrapping_add(1)
        };
        wr16(ctr, ax);
        global = rd32(lf_checker_rt::relocated(GLOBAL_OBJ));
        wr16(global.wrapping_add(OBJ_SEQ), ax);
        global = rd32(lf_checker_rt::relocated(GLOBAL_OBJ));
        let fb = global.wrapping_add(OBJ_FLAGS);
        wr8(fb, rd8(fb) | 1);
        global = rd32(lf_checker_rt::relocated(GLOBAL_OBJ));
        let fb = global.wrapping_add(OBJ_FLAGS);
        wr8(fb, rd8(fb) & 0xf7);
        let mode_a = (arg2 as u8) == 1;
        if mode_a {
            // Mode A input block: {-m76c, m76c, m768, -m768}.
            let m76c = rd32(global.wrapping_add(0x76c));
            let m768 = rd32(global.wrapping_add(0x768));
            fr[0x64] = fneg_bits(m76c);
            fr[0x65] = m76c;
            fr[0x66] = m768;
            fr[0x67] = fneg_bits(m768);
            let ans: u32 = lf_checker_rt::callee_cdecl!(
                2, u32, fr_addr(&fr, 0x190), arg1, global
            );
            if (ans & 0xff) == 0 {
                global = rd32(lf_checker_rt::relocated(GLOBAL_OBJ));
                let fb = global.wrapping_add(OBJ_FLAGS);
                wr8(fb, rd8(fb) & 0xfe);
                let _ = lf_checker_rt::callee_cdecl!(7, u32,);
                return global;
            }
            x1 = rd32(r_esi.wrapping_add(0x2c4));
            x3 = fr[0x66];
            x0 = 0;
            x2 = x0;
            x4 = x0;
            x0 = fr[0x43];
            x5 = fr[0x64];
            x7 = fr[0x67];
            fr[0x6b] = x0;
            x0 = fr[0x43];
            fr[0x6f] = x0;
            x0 = fr[0x65];
            x0 = fmul(x0, x1);
            x3 = fmul(x3, x1);
            fr[0x34] = x0;
            fr[0x70] = x0;
            x5 = fmul(x5, x1);
            x7 = fmul(x7, x1);
            x6 = x1;
            x6 = fneg_bits(x6);
            x0 = x3;
            fr[0x51] = x0;
            fr[0x71] = x0;
            x0 = fr[0x43];
            fr[0x73] = x0;
            x0 = fr[0x34];
            fr[0x54] = x0;
            fr[0x74] = x0;
            x0 = fr[0x43];
            fr[0x77] = x0;
            x0 = x5;
            fr[0x48] = x0;
            fr[0x78] = x0;
            x0 = x7;
            fr[0x5e] = x0;
            fr[0x79] = x0;
            x1 = fr[0x86];
            x0 = x6;
            fr[0x5c] = x0;
            fr[0x7a] = x0;
            x0 = fr[0x43];
            fr[0x7b] = x0;
            x0 = fr[0x85];
            fr[0x46] = x0;
            x0 = fr[0x84];
            fr[0x10] = x0;
            x0 = fr[0x82];
            fr[0x52] = x0;
            x0 = fr[0x81];
            fr[0x60] = x0;
            x0 = fr[0x80];
            fr[0x3b] = x3;
            fr[0x6d] = x3;
            x3 = x6;
            fr[0x4] = x0;
            x0 = fr[0x7e];
            fr[0x5a] = x3;
            fr[0x76] = x3;
            x3 = fr[0x7d];
            fr[0x58] = x0;
            x0 = fr[0x7c];
            fr[0x68] = x4;
            fr[0x33] = x2;
            fr[0x69] = x2;
            fr[0x4e] = x2;
            fr[0x6a] = x2;
            fr[0x6c] = x5;
            fr[0x6e] = x6;
            fr[0x72] = x6;
            fr[0x75] = x7;
            fr[0x62] = x3;
            fr[0x1c] = x0;
        } else {
            // Mode B input block: {-m37c, m37c, m378, -m378}.
            let m37c = rd32(r_esi.wrapping_add(0x2cc));
            let m378 = rd32(r_esi.wrapping_add(0x2c8));
            fr[0x64] = fneg_bits(m37c);
            fr[0x65] = m37c;
            fr[0x66] = m378;
            fr[0x67] = fneg_bits(m378);
            let ans: u32 = lf_checker_rt::callee_cdecl!(
                2, u32, fr_addr(&fr, 0x190), arg1, global
            );
            if (ans & 0xff) == 0 {
                global = rd32(lf_checker_rt::relocated(GLOBAL_OBJ));
                let fb = global.wrapping_add(OBJ_FLAGS);
                wr8(fb, rd8(fb) & 0xfe);
                global = rd32(lf_checker_rt::relocated(GLOBAL_OBJ));
                let fb = global.wrapping_add(OBJ_FLAGS);
                wr8(fb, rd8(fb) & 0xf7);
                let _ = lf_checker_rt::callee_cdecl!(7, u32,);
                return global;
            }
            x2 = rd32(r_esi.wrapping_add(0x2c4));
            x1 = rd32(r_esi.wrapping_add(0x2c0));
            x5 = fr[0x65];
            x6 = fr[0x66];
            x0 = fr[0x43];
            x4 = fr[0x64];
            fr[0x6b] = x0;
            x7 = fr[0x67];
            x6 = fmul(x6, x1);
            x5 = fmul(x5, x2);
            x4 = fmul(x4, x2);
            x0 = x6;
            fr[0x3b] = x0;
            fr[0x6d] = x0;
            x0 = fr[0x43];
            fr[0x6f] = x0;
            x3 = x1;
            x3 = fneg_bits(x3);
            x0 = x5;
            fr[0x70] = x0;
            x0 = fr[0x43];
            fr[0x73] = x0;
            fr[0x4e] = x3;
            fr[0x6a] = x3;
            fr[0x72] = x3;
            fr[0x5a] = x3;
            fr[0x76] = x3;
            fr[0x33] = x6;
            fr[0x69] = x6;
            x6 = x3;
            x3 = fr[0x66];
            x3 = fmul(x3, x2);
            x0 = x4;
            fr[0x54] = x0;
            fr[0x74] = x0;
            x0 = fr[0x43];
            fr[0x77] = x0;
            x0 = x4;
            fr[0x48] = x0;
            fr[0x78] = x0;
            x0 = fr[0x43];
            fr[0x7b] = x0;
            x7 = fmul(x7, x1);
            fr[0x5e] = x3;
            fr[0x79] = x3;
            fr[0x62] = x3;
            fr[0x7d] = x3;
            x3 = fr[0x67];
            x3 = fmul(x3, x2);
            x0 = x5;
            fr[0x1c] = x0;
            fr[0x7c] = x0;
            x1 = x2;
            x1 = fneg_bits(x1);
            x0 = x1;
            fr[0x58] = x0;
            fr[0x7e] = x0;
            x0 = fr[0x43];
            fr[0x7f] = x0;
            x0 = x3;
            fr[0x60] = x0;
            fr[0x81] = x0;
            x0 = x1;
            fr[0x52] = x0;
            fr[0x82] = x0;
            x0 = fr[0x43];
            x2 = x5;
            fr[0x83] = x0;
            fr[0x68] = x4;
            fr[0x6c] = x5;
            fr[0x6e] = x6;
            fr[0x34] = x5;
            fr[0x51] = x7;
            fr[0x71] = x7;
            fr[0x75] = x7;
            fr[0x5c] = x1;
            fr[0x7a] = x1;
            fr[0x4] = x5;
            fr[0x46] = x3;
            fr[0x80] = x2;
            x0 = x4;
            fr[0x10] = x0;
            fr[0x84] = x0;
            x0 = fr[0x43];
            fr[0x85] = x3;
            fr[0x86] = x1;
            fr[0x87] = x0;
            x2 = x6;
        }
            x0 = rd32(r_esi.wrapping_add(0x40));
            fr[0xe] = x1;
            x1 = rd32(r_esi.wrapping_add(0x54));
            fr[0x32] = x1;
            x1 = rd32(r_esi.wrapping_add(0x58));
            fr[0x22] = x1;
            x1 = rd32(r_esi.wrapping_add(0x60));
            fr[0x24] = x1;
            x1 = rd32(r_esi.wrapping_add(0x64));
            fr[0x2b] = x1;
            x1 = rd32(r_esi.wrapping_add(0x68));
            fr[0x2c] = x0;
            x0 = rd32(r_esi.wrapping_add(0x44));
            fr[0xa] = x1;
            x1 = rd32(r_esi.wrapping_add(0x70));
            fr[0x19] = x0;
            x0 = rd32(r_esi.wrapping_add(0x48));
            fr[0x14] = x1;
            x1 = rd32(r_esi.wrapping_add(0x74));
            fr[0x1a] = x0;
            x0 = rd32(r_esi.wrapping_add(0x50));
            fr[0x18] = x1;
            x1 = rd32(r_esi.wrapping_add(0x78));
            fr[0x3c] = x0;
            fr[0xc] = x1;
            wr32(lf_checker_rt::relocated(0x1593bc0), 0x0u32);
        if mode_a {
            // Transform stage through the register-arg callee.
            global = rd32(lf_checker_rt::relocated(GLOBAL_OBJ));
            let pushed = global.wrapping_add(0x4e0);
            let _ans: u32 =
                lf_checker_rt::callee_thiscall!(3, u32, fr_addr(&fr, 0x270), pushed);
            x5 = fr[0x69];
            x3 = fr[0x68];
            x6 = fr[0xa0];
            x0 = fr[0x9c];
            x2 = fr[0x6a];
            x0 = fmul(x0, x3);
            x4 = fr[0xa1];
            x6 = fmul(x6, x5);
            x1 = fr[0xa2];
            x4 = fmul(x4, x5);
            x6 = fadd(x6, x0);
            x0 = fr[0xa4];
            x0 = fmul(x0, x2);
            x1 = fmul(x1, x5);
            x6 = fadd(x6, x0);
            x0 = fr[0x9d];
            x0 = fmul(x0, x3);
            x5 = fr[0xa1];
            x6 = fadd(x6, fr[0xa8]);
            x4 = fadd(x4, x0);
            x0 = fr[0xa5];
            x0 = fmul(x0, x2);
            x7 = fr[0xaa];
            fr[0x68] = x6;
            x4 = fadd(x4, x0);
            x0 = fr[0x9e];
            x0 = fmul(x0, x3);
            x3 = fr[0x6c];
            x4 = fadd(x4, fr[0xa9]);
            x1 = fadd(x1, x0);
            x0 = fr[0xa6];
            x6 = fr[0xa0];
            x0 = fmul(x0, x2);
            x2 = fr[0x6e];
            fr[0x69] = x4;
            x4 = fr[0x6d];
            x1 = fadd(x1, x0);
            x0 = fr[0x43];
            fr[0x6b] = x0;
            x0 = fr[0x9c];
            x0 = fmul(x0, x3);
            x6 = fmul(x6, x4);
            x5 = fmul(x5, x4);
            x6 = fadd(x6, x0);
            x0 = fr[0xa4];
            x0 = fmul(x0, x2);
            x1 = fadd(x1, x7);
            x6 = fadd(x6, x0);
            x0 = fr[0x9d];
            x0 = fmul(x0, x3);
            fr[0x6a] = x1;
            x1 = fr[0xa2];
            x5 = fadd(x5, x0);
            x0 = fr[0xa5];
            x6 = fadd(x6, fr[0xa8]);
            x0 = fmul(x0, x2);
            x1 = fmul(x1, x4);
            x5 = fadd(x5, x0);
            x0 = fr[0x9e];
            x4 = fr[0x71];
            x0 = fmul(x0, x3);
            x5 = fadd(x5, fr[0xa9]);
            x3 = fr[0x70];
            x1 = fadd(x1, x0);
            x0 = fr[0xa6];
            x0 = fmul(x0, x2);
            fr[0x6c] = x6;
            x6 = fr[0xa0];
            x1 = fadd(x1, x0);
            x0 = fr[0x43];
            fr[0x6f] = x0;
            x0 = fr[0x9c];
            fr[0x6d] = x5;
            x1 = fadd(x1, x7);
            x6 = fmul(x6, x4);
            fr[0x6e] = x1;
            x2 = fr[0x72];
            x0 = fmul(x0, x3);
            x5 = fr[0xa1];
            x1 = fr[0xa2];
            x6 = fadd(x6, x0);
            x0 = fr[0xa4];
            x0 = fmul(x0, x2);
            x5 = fmul(x5, x4);
            x6 = fadd(x6, x0);
            x0 = fr[0x9d];
            x0 = fmul(x0, x3);
            x1 = fmul(x1, x4);
            x5 = fadd(x5, x0);
            x0 = fr[0xa5];
            x0 = fmul(x0, x2);
            x6 = fadd(x6, fr[0xa8]);
            x5 = fadd(x5, x0);
            x0 = fr[0x9e];
            x0 = fmul(x0, x3);
            x3 = fr[0x74];
            x5 = fadd(x5, fr[0xa9]);
            x1 = fadd(x1, x0);
            x0 = fr[0xa6];
            x0 = fmul(x0, x2);
            fr[0x71] = x5;
            x5 = fr[0x75];
            x1 = fadd(x1, x0);
            x4 = x5;
            x4 = fmul(x4, fr[0xa0]);
            x0 = x3;
            x0 = fmul(x0, fr[0x9c]);
            x1 = fadd(x1, x7);
            x2 = x5;
            x2 = fmul(x2, fr[0xa1]);
            x5 = fmul(x5, fr[0xa2]);
            x4 = fadd(x4, x0);
            fr[0x72] = x1;
            x1 = fr[0x76];
            x0 = x1;
            x0 = fmul(x0, fr[0xa4]);
            fr[0x70] = x6;
            x6 = fr[0x43];
            x4 = fadd(x4, x0);
            x0 = x3;
            x0 = fmul(x0, fr[0x9d]);
            x3 = fmul(x3, fr[0x9e]);
            x4 = fadd(x4, fr[0xa8]);
            x2 = fadd(x2, x0);
            x0 = x1;
            x1 = fmul(x1, fr[0xa6]);
            x0 = fmul(x0, fr[0xa5]);
            x5 = fadd(x5, x3);
            x3 = fr[0x78];
            x2 = fadd(x2, x0);
            fr[0x74] = x4;
            x0 = x3;
            x0 = fmul(x0, fr[0x9c]);
            x2 = fadd(x2, fr[0xa9]);
            x5 = fadd(x5, x1);
            x1 = fr[0x7a];
            fr[0x73] = x6;
            fr[0x77] = x6;
            fr[0x75] = x2;
            x5 = fadd(x5, x7);
            fr[0x76] = x5;
            x5 = fr[0x79];
            x4 = x5;
            x4 = fmul(x4, fr[0xa0]);
            x2 = x5;
            x2 = fmul(x2, fr[0xa1]);
            x4 = fadd(x4, x0);
            x0 = x1;
            x0 = fmul(x0, fr[0xa4]);
            x4 = fadd(x4, x0);
            x0 = x3;
            x0 = fmul(x0, fr[0x9d]);
            x4 = fadd(x4, fr[0xa8]);
            x2 = fadd(x2, x0);
            x0 = x1;
            x0 = fmul(x0, fr[0xa5]);
            x2 = fadd(x2, x0);
            x5 = fmul(x5, fr[0xa2]);
            x3 = fmul(x3, fr[0x9e]);
            x1 = fmul(x1, fr[0xa6]);
            x2 = fadd(x2, fr[0xa9]);
            x5 = fadd(x5, x3);
            fr[0x78] = x4;
            fr[0x7b] = x6;
            fr[0x79] = x2;
            x5 = fadd(x5, x1);
            x5 = fadd(x5, x7);
            fr[0x7a] = x5;
        } else {
            x3 = x0;
            x3 = fmul(x3, fr[0x33]);
            x0 = fr[0x2c];
            x0 = fmul(x0, x4);
            x1 = fr[0x22];
            x1 = fmul(x1, fr[0x33]);
            x3 = fadd(x3, x0);
            x0 = fr[0x24];
            x0 = fmul(x0, x2);
            x2 = fr[0x32];
            x2 = fmul(x2, fr[0x33]);
            x3 = fadd(x3, x0);
            x0 = fr[0x19];
            x0 = fmul(x0, x4);
            x3 = fadd(x3, fr[0x14]);
            x2 = fadd(x2, x0);
            x0 = fr[0x2b];
            x0 = fmul(x0, fr[0x4e]);
            fr[0x68] = x3;
            x3 = fr[0x3c];
            x2 = fadd(x2, x0);
            x0 = fr[0x1a];
            x0 = fmul(x0, x4);
            x3 = fmul(x3, fr[0x3b]);
            x1 = fadd(x1, x0);
            x0 = fr[0xa];
            x0 = fmul(x0, fr[0x4e]);
            x2 = fadd(x2, fr[0x18]);
            x4 = fr[0x43];
            x1 = fadd(x1, x0);
            x0 = fr[0x2c];
            x0 = fmul(x0, x5);
            fr[0x69] = x2;
            x2 = fr[0x32];
            x2 = fmul(x2, fr[0x3b]);
            x3 = fadd(x3, x0);
            x0 = fr[0x24];
            x1 = fadd(x1, fr[0xc]);
            x0 = fmul(x0, x6);
            fr[0x6b] = x4;
            fr[0x6f] = x4;
            x3 = fadd(x3, x0);
            x0 = fr[0x19];
            x0 = fmul(x0, x5);
            fr[0x6a] = x1;
            x1 = fr[0x22];
            x1 = fmul(x1, fr[0x3b]);
            x3 = fadd(x3, fr[0x14]);
            x2 = fadd(x2, x0);
            x0 = fr[0x2b];
            x0 = fmul(x0, x6);
            fr[0x6c] = x3;
            x3 = fr[0x3c];
            x2 = fadd(x2, x0);
            x0 = fr[0x1a];
            x0 = fmul(x0, x5);
            x5 = fr[0x34];
            x2 = fadd(x2, fr[0x18]);
            x1 = fadd(x1, x0);
            x0 = fr[0xa];
            x0 = fmul(x0, x6);
            x6 = fr[0x51];
            x3 = fmul(x3, x6);
            x1 = fadd(x1, x0);
            x0 = fr[0x2c];
            x0 = fmul(x0, x5);
            fr[0x6d] = x2;
            x2 = fr[0x32];
            x1 = fadd(x1, fr[0xc]);
            x3 = fadd(x3, x0);
            x0 = fr[0x24];
            x0 = fmul(x0, fr[0x72]);
            x2 = fmul(x2, x6);
            x3 = fadd(x3, x0);
            x0 = fr[0x19];
            x0 = fmul(x0, x5);
            fr[0x6e] = x1;
            x3 = fadd(x3, fr[0x14]);
            x2 = fadd(x2, x0);
            x0 = fr[0x2b];
            x0 = fmul(x0, fr[0x72]);
            x2 = fadd(x2, x0);
            x1 = fr[0x22];
            x0 = fr[0x1a];
            x0 = fmul(x0, x5);
            x2 = fadd(x2, fr[0x18]);
            x5 = fr[0x5a];
            x1 = fmul(x1, x6);
            x6 = fr[0x3c];
            fr[0x71] = x2;
            x1 = fadd(x1, x0);
            x0 = fr[0xa];
            x0 = fmul(x0, fr[0x72]);
            fr[0x70] = x3;
            x3 = fr[0x54];
            x1 = fadd(x1, x0);
            x2 = x7;
            x2 = fmul(x2, x6);
            x0 = x3;
            x0 = fmul(x0, fr[0x2c]);
            x1 = fadd(x1, fr[0xc]);
            fr[0x73] = x4;
            x2 = fadd(x2, x0);
            x0 = x5;
            x0 = fmul(x0, fr[0x24]);
            fr[0x72] = x1;
            x1 = x7;
            x1 = fmul(x1, fr[0x32]);
            x7 = fmul(x7, fr[0x22]);
            x2 = fadd(x2, x0);
            x0 = x3;
            x0 = fmul(x0, fr[0x19]);
            x3 = fmul(x3, fr[0x1a]);
            x2 = fadd(x2, fr[0x14]);
            x1 = fadd(x1, x0);
            x0 = x5;
            x0 = fmul(x0, fr[0x2b]);
            x5 = fmul(x5, fr[0xa]);
            x1 = fadd(x1, x0);
            x0 = fr[0x48];
            x0 = fmul(x0, fr[0x2c]);
            x7 = fadd(x7, x3);
            x3 = fr[0x5c];
            x1 = fadd(x1, fr[0x18]);
            fr[0x74] = x2;
            fr[0x77] = x4;
            x7 = fadd(x7, x5);
            x5 = fr[0x5e];
            x2 = x5;
            x2 = fmul(x2, x6);
            x7 = fadd(x7, fr[0xc]);
            fr[0x75] = x1;
            x2 = fadd(x2, x0);
            x0 = x3;
            x0 = fmul(x0, fr[0x24]);
            x1 = x5;
            x5 = fmul(x5, fr[0x22]);
            x2 = fadd(x2, x0);
            x0 = fr[0x48];
            x0 = fmul(x0, fr[0x19]);
            fr[0x76] = x7;
            x7 = fr[0x32];
            x2 = fadd(x2, fr[0x14]);
            x1 = fmul(x1, x7);
            fr[0x7b] = x4;
            x1 = fadd(x1, x0);
            x0 = x3;
            x0 = fmul(x0, fr[0x2b]);
            x3 = fmul(x3, fr[0xa]);
            x1 = fadd(x1, x0);
            x0 = fr[0x48];
            x0 = fmul(x0, fr[0x1a]);
            fr[0x78] = x2;
            x1 = fadd(x1, fr[0x18]);
            x5 = fadd(x5, x0);
            fr[0x79] = x1;
            x5 = fadd(x5, x3);
            x3 = fr[0x62];
            x2 = x3;
            x2 = fmul(x2, x6);
            x5 = fadd(x5, fr[0xc]);
            fr[0x7a] = x5;
            x0 = fr[0x1c];
            x0 = fmul(x0, fr[0x2c]);
            x5 = fr[0x58];
            x1 = x3;
            x2 = fadd(x2, x0);
            x3 = fmul(x3, fr[0x22]);
            x0 = x5;
            x0 = fmul(x0, fr[0x24]);
            x1 = fmul(x1, x7);
            x2 = fadd(x2, x0);
            x0 = fr[0x1c];
            x0 = fmul(x0, fr[0x19]);
            fr[0x7f] = x4;
            x2 = fadd(x2, fr[0x14]);
            x1 = fadd(x1, x0);
            x0 = x5;
            x0 = fmul(x0, fr[0x2b]);
            x5 = fmul(x5, fr[0xa]);
            x1 = fadd(x1, x0);
            x0 = fr[0x1c];
            x0 = fmul(x0, fr[0x1a]);
            fr[0x7c] = x2;
            x1 = fadd(x1, fr[0x18]);
            x3 = fadd(x3, x0);
            x0 = fr[0x4];
            x0 = fmul(x0, fr[0x2c]);
            fr[0x7d] = x1;
            x3 = fadd(x3, x5);
            x5 = fr[0x52];
            fr[0x83] = x4;
            x3 = fadd(x3, fr[0xc]);
            fr[0x7e] = x3;
            x3 = fr[0x60];
            x2 = x3;
            x2 = fmul(x2, x6);
            x1 = x3;
            x3 = fmul(x3, fr[0x22]);
            x2 = fadd(x2, x0);
            x0 = x5;
            x0 = fmul(x0, fr[0x24]);
            x1 = fmul(x1, x7);
            x2 = fadd(x2, x0);
            x0 = fr[0x4];
            x0 = fmul(x0, fr[0x19]);
            x2 = fadd(x2, fr[0x14]);
            x1 = fadd(x1, x0);
            x0 = x5;
            x0 = fmul(x0, fr[0x2b]);
            x5 = fmul(x5, fr[0xa]);
            x1 = fadd(x1, x0);
            x0 = fr[0x4];
            x0 = fmul(x0, fr[0x1a]);
            fr[0x80] = x2;
            x1 = fadd(x1, fr[0x18]);
            x3 = fadd(x3, x0);
            fr[0x81] = x1;
            x3 = fadd(x3, x5);
            x5 = fr[0xe];
            x3 = fadd(x3, fr[0xc]);
            fr[0x82] = x3;
            x3 = fr[0x46];
            x2 = x3;
            x2 = fmul(x2, x6);
            x6 = fr[0x10];
            x0 = x6;
            x0 = fmul(x0, fr[0x2c]);
            x1 = x3;
            x3 = fmul(x3, fr[0x22]);
            x2 = fadd(x2, x0);
            x0 = x5;
            x0 = fmul(x0, fr[0x24]);
            x1 = fmul(x1, x7);
            x2 = fadd(x2, x0);
            x0 = x6;
            x0 = fmul(x0, fr[0x19]);
            x2 = fadd(x2, fr[0x14]);
            x1 = fadd(x1, x0);
            x0 = x5;
            x0 = fmul(x0, fr[0x2b]);
            x1 = fadd(x1, x0);
            x1 = fadd(x1, fr[0x18]);
            x6 = fmul(x6, fr[0x1a]);
            x5 = fmul(x5, fr[0xa]);
            x3 = fadd(x3, x6);
            fr[0x84] = x2;
            fr[0x85] = x1;
            fr[0x87] = x4;
            x3 = fadd(x3, x5);
            x3 = fadd(x3, fr[0xc]);
            fr[0x86] = x3;
        }
        // Join stage through the two-pointer callee.
        x0 = rd32(r_esi.wrapping_add(0x60));
        fr[0x40] = x0;
        x0 = rd32(r_esi.wrapping_add(0x64));
        fr[0x41] = x0;
        x0 = rd32(r_esi.wrapping_add(0x68));
        fr[0x42] = x0;
        let _ans: u32 = lf_checker_rt::callee_thiscall!(
            4, u32, global.wrapping_add(0x50), fr_addr(&fr, 0x1a0), fr_addr(&fr, 0x100)
        );
        global = rd32(lf_checker_rt::relocated(GLOBAL_OBJ));
        let fb = global.wrapping_add(OBJ_FLAGS);
        wr8(fb, rd8(fb) | 8);
        wr8(lf_checker_rt::relocated(ZERO_B), 0);
        if mode_a {
            x5 = fr[0x69];
            x6 = rd32(r_esi.wrapping_add(0x70));
            x1 = fr[0x68];
            x0 = fr[0x6a];
            x0 = fsub(x0, rd32(r_esi.wrapping_add(0x78)));
            x1 = fsub(x1, x6);
            x2 = x5;
            x2 = fsub(x2, rd32(r_esi.wrapping_add(0x74)));
            x3 = fr[0x6d];
            x0 = fmul(x0, rd32(r_esi.wrapping_add(0x68)));
            x1 = fmul(x1, rd32(r_esi.wrapping_add(0x60)));
            x2 = fmul(x2, rd32(r_esi.wrapping_add(0x64)));
            x7 = fr[0x71];
            x4 = fr[0x43];
            x2 = fadd(x2, x1);
            x1 = fr[0x68];
            fr[0xb3] = x4;
            fr[0xb2] = 0x0u32;
            fr[0xb7] = x4;
            x2 = fadd(x2, x0);
            fr[0xb6] = 0x0u32;
            fr[0xbb] = x4;
            fr[0xba] = 0x0u32;
            x0 = x2;
            x0 = fmul(x0, rd32(r_esi.wrapping_add(0x60)));
            x2 = fmul(x2, rd32(r_esi.wrapping_add(0x64)));
            x1 = fsub(x1, x0);
            x0 = x5;
            x5 = fr[0x6c];
            x0 = fsub(x0, x2);
            x2 = x3;
            x2 = fsub(x2, rd32(r_esi.wrapping_add(0x74)));
            fr[0xb0] = x1;
            x1 = x5;
            fr[0xb1] = x0;
            x0 = fr[0x6e];
            x0 = fsub(x0, rd32(r_esi.wrapping_add(0x78)));
            x2 = fmul(x2, rd32(r_esi.wrapping_add(0x64)));
            x1 = fsub(x1, x6);
            x0 = fmul(x0, rd32(r_esi.wrapping_add(0x68)));
            x1 = fmul(x1, rd32(r_esi.wrapping_add(0x60)));
            x2 = fadd(x2, x1);
            x1 = x5;
            x5 = fr[0x75];
            x2 = fadd(x2, x0);
            x0 = x2;
            x0 = fmul(x0, rd32(r_esi.wrapping_add(0x60)));
            x2 = fmul(x2, rd32(r_esi.wrapping_add(0x64)));
            x1 = fsub(x1, x0);
            x0 = x3;
            x3 = fr[0x70];
            x0 = fsub(x0, x2);
            x2 = x7;
            x2 = fsub(x2, rd32(r_esi.wrapping_add(0x74)));
            fr[0xb4] = x1;
            x1 = x3;
            fr[0xb5] = x0;
            x0 = fr[0x72];
            x0 = fsub(x0, rd32(r_esi.wrapping_add(0x78)));
            x2 = fmul(x2, rd32(r_esi.wrapping_add(0x64)));
            x1 = fsub(x1, x6);
            x6 = fr[0x74];
            x0 = fmul(x0, rd32(r_esi.wrapping_add(0x68)));
            x1 = fmul(x1, rd32(r_esi.wrapping_add(0x60)));
            x2 = fadd(x2, x1);
            x1 = x3;
            x2 = fadd(x2, x0);
            x0 = x2;
            x0 = fmul(x0, rd32(r_esi.wrapping_add(0x60)));
            x2 = fmul(x2, rd32(r_esi.wrapping_add(0x64)));
            x1 = fsub(x1, x0);
            x0 = x7;
            x0 = fsub(x0, x2);
            x2 = x5;
            x2 = fsub(x2, rd32(r_esi.wrapping_add(0x74)));
            fr[0xb8] = x1;
            x1 = x6;
            x1 = fsub(x1, rd32(r_esi.wrapping_add(0x70)));
            fr[0xb9] = x0;
            x0 = fr[0x76];
            x0 = fsub(x0, rd32(r_esi.wrapping_add(0x78)));
            x0 = fmul(x0, rd32(r_esi.wrapping_add(0x68)));
            x2 = fmul(x2, rd32(r_esi.wrapping_add(0x64)));
            x1 = fmul(x1, rd32(r_esi.wrapping_add(0x60)));
            x3 = fr[0x79];
            fr[0xbf] = x4;
            x2 = fadd(x2, x1);
            x4 = fr[0x78];
            x1 = x6;
            fr[0xbe] = 0x0u32;
            fr[0xc2] = 0x0u32;
            x2 = fadd(x2, x0);
            fr[0xc6] = 0x0u32;
            fr[0xca] = 0x0u32;
            fr[0xcd] = x7;
            fr[0xce] = 0x0u32;
            x0 = x2;
            x0 = fmul(x0, rd32(r_esi.wrapping_add(0x60)));
            x2 = fmul(x2, rd32(r_esi.wrapping_add(0x64)));
            x1 = fsub(x1, x0);
            x0 = x5;
            x0 = fsub(x0, x2);
            x2 = x3;
            x2 = fsub(x2, rd32(r_esi.wrapping_add(0x74)));
            fr[0xbc] = x1;
            x1 = x4;
            x1 = fsub(x1, rd32(r_esi.wrapping_add(0x70)));
            fr[0xbd] = x0;
            x0 = fr[0x7a];
            x0 = fsub(x0, rd32(r_esi.wrapping_add(0x78)));
            x2 = fmul(x2, rd32(r_esi.wrapping_add(0x64)));
            x1 = fmul(x1, rd32(r_esi.wrapping_add(0x60)));
            x0 = fmul(x0, rd32(r_esi.wrapping_add(0x68)));
            x2 = fadd(x2, x1);
            x1 = x4;
            fr[0xd0] = x6;
            fr[0xd1] = x5;
            fr[0xd2] = 0x0u32;
            x2 = fadd(x2, x0);
            fr[0xd4] = x4;
            fr[0xd5] = x3;
            fr[0xd6] = 0x0u32;
            x0 = x2;
            x0 = fmul(x0, rd32(r_esi.wrapping_add(0x60)));
            x2 = fmul(x2, rd32(r_esi.wrapping_add(0x64)));
            x1 = fsub(x1, x0);
            x0 = x3;
            x0 = fsub(x0, x2);
            r_esi = 0xau32;
            fr[0xc0] = x1;
            fr[0xc1] = x0;
            x0 = fr[0x43];
            fr[0xc3] = x0;
            x0 = fr[0x68];
            fr[0xc4] = x0;
            x0 = fr[0x69];
            fr[0xc5] = x0;
            x0 = fr[0x6b];
            fr[0xc7] = x0;
            x0 = fr[0x6c];
            fr[0xc8] = x0;
            x0 = fr[0x6d];
            fr[0xc9] = x0;
            x0 = fr[0x6f];
            fr[0xcb] = x0;
            x0 = fr[0x70];
            fr[0xcc] = x0;
            x0 = fr[0x73];
            fr[0xcf] = x0;
            x0 = fr[0x77];
            fr[0xd3] = x0;
            x0 = fr[0x7b];
            fr[0xd7] = x0;
        } else {
            // Straight 128-bit block move of eight rows into place.
            let mut i = 0usize;
            while i < 8 {
                let s = 0x68 + i * 4;
                let d = 0xb0 + i * 4;
                fr[d] = fr[s];
                fr[d + 1] = fr[s + 1];
                fr[d + 2] = fr[s + 2];
                fr[d + 3] = fr[s + 3];
                i += 1;
            }
            r_esi = 8;
        }
        // Mean pass over the N row triples at stride 16 from fr[0xb0].
            x3 = 0x3f800000u32 /* C_fe88e8 */;
            x0 = r_esi;
            x0 = ((x0 as i32) as f32).to_bits();
            x6 = 0;
            x5 = x6;
            x1 = x6;
            fr[0x1c] = r_esi;
            fr[0x34] = x5;
            x7 = x6;
            fr[0x3b] = x6;
            fr[0x4] = x1;
            x3 = fdiv(x3, x0);
        if (r_esi as i32) > 0 {
            r_eax = fr_addr(&fr, 0x2c8);
            r_ecx = r_esi;
            loop {
            x0 = rd32(r_eax.wrapping_add(0xfffffff8));
            x1 = rd32(r_eax.wrapping_add(0xfffffffc));
            x2 = rd32(r_eax);
            x0 = fmul(x0, x3);
            x1 = fmul(x1, x3);
            x2 = fmul(x2, x3);
            r_eax = r_eax.wrapping_add(0x10u32);
            x5 = fadd(x5, x0);
            x7 = fadd(x7, x1);
            x6 = fadd(x6, x2);
            r_ecx = r_ecx.wrapping_sub(1);
                if r_ecx == 0 {
                    break;
                }
            }
        }
            fr[0x4] = x6;
            x1 = fr[0x4];
            fr[0x3b] = x7;
            fr[0x34] = x5;
            x6 = 0;
            r_edi = r_edi ^ (r_edi);
        r_edi = 0;
        if (r_esi as i32) > 0 {
            r_eax = fr_addr(&fr, 0x2c8);
            fr[0x33] = r_eax;
            // Per-row radius and angle; tiny radii map to angle 0.
            loop {
                x4 = rd32(r_eax.wrapping_sub(8));
                x3 = rd32(r_eax.wrapping_sub(4));
                x2 = rd32(r_eax);
                x4 = fsub(x4, x5);
                x3 = fsub(x3, x7);
                x2 = fsub(x2, x1);
                x0 = x4;
                x1 = x3;
                x1 = fmul(x1, x3);
                x0 = fmul(x0, x4);
                x2 = fmul(x2, x2);
                x1 = fadd(x1, x0);
                x0 = 0;
                x1 = fadd(x1, x2);
                x0 = fsqrt(x1);
                x1 = C_RMIN;
                fr[((0x270 + r_edi.wrapping_mul(4)) / 4) as usize] = x0;
                if f_jbe(x1, x0) {
                    x3 = fdiv(x3, x0);
                    x0 = x3;
                    if f_jb(x4, x6) {
                        let got: f32 =
                            lf_checker_rt::callee_cdecl!(5, f32, x0);
                        x1 = C_TWO_PI;
                        x1 = fsub(x1, got.to_bits());
                        fr[((0x390 + r_edi.wrapping_mul(4)) / 4) as usize] = x1;
                    } else {
                        let got: f32 =
                            lf_checker_rt::callee_cdecl!(5, f32, x0);
                        fr[((0x390 + r_edi.wrapping_mul(4)) / 4) as usize] =
                            got.to_bits();
                    }
                    r_eax = fr[0x33];
                    x7 = fr[0x3b];
                    x5 = fr[0x34];
                    x6 = 0;
                } else {
                    fr[((0x390 + r_edi.wrapping_mul(4)) / 4) as usize] = 0;
                }
                x1 = fr[0x04];
                fr[((0x368 + r_edi.wrapping_mul(4)) / 4) as usize] = r_edi;
                r_edi = r_edi.wrapping_add(1);
                r_eax = r_eax.wrapping_add(0x10);
                fr[0x33] = r_eax;
                if !((r_edi as i32) < (r_esi as i32)) {
                    break;
                }
            }
            // Bubble pass ordering the row index by angle.
            r_ecx = 1;
            fr[0x0e] = r_ecx;
            r_edi = fr_addr(&fr, 0x368);
            r_edx = r_esi;
            fr[0x10] = r_esi;
            loop {
                r_eax = r_ecx;
                if (r_ecx as i32) < (r_esi as i32) {
                    loop {
                        let ii = rd32(r_edi);
                        let jj = fr[((0x368 + r_eax.wrapping_mul(4)) / 4) as usize];
                        let ki = fr[(0x390 / 4 + ii.wrapping_mul(4) / 4) as usize];
                        let kj = fr[(0x390 / 4 + jj.wrapping_mul(4) / 4) as usize];
                        if !f_jbe(ki, kj) {
                            wr32(r_edi, jj);
                            fr[((0x368 + r_eax.wrapping_mul(4)) / 4) as usize] = ii;
                        }
                        r_eax = r_eax.wrapping_add(1);
                        if !((r_eax as i32) < (r_esi as i32)) {
                            break;
                        }
                    }
                    r_ecx = fr[0x0e];
                    r_edx = fr[0x10];
                }
                r_ecx = r_ecx.wrapping_add(1);
                r_edi = r_edi.wrapping_add(4);
                r_edx = r_edx.wrapping_sub(1);
                fr[0x0e] = r_ecx;
                fr[0x10] = r_edx;
                if r_edx == 0 {
                    break;
                }
            }
        }
        // Neighbor filter with wrap indexing over the ordered rows.
        if (r_esi as i32) > 1 {
            r_edi = fr_addr(&fr, 0x368);
            r_eax = fr_addr(&fr, 0x364).wrapping_add(r_esi.wrapping_mul(4));
            r_edx = 1;
            fr[0x10] = r_edi;
            fr[0x0e] = r_eax;
            loop {
                r_ecx = r_edx.wrapping_sub(2);
                if (r_ecx as i32) < 0 {
                    r_ecx = r_ecx.wrapping_add(r_esi);
                }
                r_eax = r_edx;
                if !((r_edx as i32) < (r_esi as i32)) {
                    r_eax = r_eax.wrapping_sub(r_esi);
                }
                r_eax = fr[((0x368 + r_eax.wrapping_mul(4)) / 4) as usize];
                r_ecx = fr[((0x368 + r_ecx.wrapping_mul(4)) / 4) as usize];
                r_eax = r_eax.wrapping_add(r_eax);
                r_ecx = r_ecx.wrapping_add(r_ecx);
                x0 = fr[((0x2c8 + r_eax.wrapping_mul(8)) / 4) as usize];
                x0 = fsub(x0, fr[((0x2c8 + r_ecx.wrapping_mul(8)) / 4) as usize]);
                x1 = fr[((0x2c0 + r_eax.wrapping_mul(8)) / 4) as usize];
                x1 = fsub(x1, fr[((0x2c0 + r_ecx.wrapping_mul(8)) / 4) as usize]);
                x5 = fr[((0x2c4 + r_eax.wrapping_mul(8)) / 4) as usize];
                x5 = fsub(x5, fr[((0x2c4 + r_ecx.wrapping_mul(8)) / 4) as usize]);
                r_eax = rd32(r_edi);
                x0 = fmul(x0, x6);
                x3 = x1;
                x1 = fmul(x1, x6);
                x4 = x0;
                x4 = fsub(x4, x5);
                x5 = fmul(x5, x6);
                r_eax = r_eax.wrapping_add(r_eax);
                x3 = fsub(x3, x0);
                x2 = fr[((0x2c4 + r_eax.wrapping_mul(8)) / 4) as usize];
                x2 = fsub(x2, fr[((0x2c4 + r_ecx.wrapping_mul(8)) / 4) as usize]);
                x0 = fr[((0x2c8 + r_eax.wrapping_mul(8)) / 4) as usize];
                x0 = fsub(x0, fr[((0x2c8 + r_ecx.wrapping_mul(8)) / 4) as usize]);
                x5 = fsub(x5, x1);
                x1 = fr[((0x2c0 + r_eax.wrapping_mul(8)) / 4) as usize];
                x1 = fsub(x1, fr[((0x2c0 + r_ecx.wrapping_mul(8)) / 4) as usize]);
                x2 = fmul(x2, x3);
                x0 = fmul(x0, x5);
                x1 = fmul(x1, x4);
                x2 = fadd(x2, x1);
                x2 = fadd(x2, x0);
                if f_jb(x6, x2) {
                    r_edi = r_edi.wrapping_add(4);
                    fr[0x10] = r_edi;
                    r_edx = r_edx.wrapping_add(1);
                } else {
                    r_ecx = r_esi.wrapping_sub(1);
                    r_eax = r_edx.wrapping_sub(1);
                    fr[0x46] = r_ecx;
                    if (r_eax as i32) < (r_ecx as i32) {
                        r_esi = r_esi.wrapping_sub(r_edx);
                        r_ecx = r_esi;
                        r_esi = r_edi.wrapping_add(4);
                        // Overlapping forward word move of r_ecx words.
                        let mut k = 0u32;
                        while k < r_ecx {
                            let w = rd32(r_esi.wrapping_add(k.wrapping_mul(4)));
                            wr32(r_edi.wrapping_add(k.wrapping_mul(4)), w);
                            k = k.wrapping_add(1);
                        }
                        r_edi = fr[0x10];
                        r_ecx = fr[0x46];
                    }
                    r_eax = fr[0x0e];
                    r_eax = r_eax.wrapping_sub(4);
                    wr32(r_eax.wrapping_add(4), 0xffffffff);
                    r_esi = r_ecx;
                    fr[0x0e] = r_eax;
                }
                if !((r_edx as i32) < (r_esi as i32)) {
                    break;
                }
            }
        }
        fr[0x1c] = r_esi;
        // Output scaling of the surviving rows.
        r_ecx = 0;
        if (r_esi as i32) > 0 {
            x1 = C_S1;
            x2 = C_S1B;
            loop {
                r_eax = fr[((0x368 + r_ecx.wrapping_mul(4)) / 4) as usize];
                r_eax = r_eax.wrapping_add(r_eax);
                r_ecx = r_ecx.wrapping_add(1);
                x0 = fr[((0x2c0 + r_eax.wrapping_mul(8)) / 4) as usize];
                x0 = fmul(x0, x1);
                x0 = fadd(x0, x2);
                fr[((0x3b0 + r_ecx.wrapping_mul(8)) / 4) as usize] = x0;
                x0 = fr[((0x2c4 + r_eax.wrapping_mul(8)) / 4) as usize];
                x0 = fmul(x0, x1);
                x0 = fadd(x0, x2);
                fr[((0x3b4 + r_ecx.wrapping_mul(8)) / 4) as usize] = x0;
                if !((r_ecx as i32) < (r_esi as i32)) {
                    break;
                }
            }
        }
        global = rd32(lf_checker_rt::relocated(GLOBAL_OBJ));
        x3 = C_ABS;
        if rd32(global.wrapping_add(0x8e8)) & 0x200087 != 0 {
            if (r_esi as i32) > 0 {
                r_ecx = r_esi & 0x1fffffff;
                r_ecx = r_ecx.wrapping_add(r_ecx);
                r_esi = fr_addr(&fr, 0x3b8);
                r_edi = fr_addr(&fr, 0x270);
                let mut k = 0u32;
                while k < r_ecx {
                    let w = rd32(r_esi.wrapping_add(k.wrapping_mul(4)));
                    wr32(r_edi.wrapping_add(k.wrapping_mul(4)), w);
                    k = k.wrapping_add(1);
                }
                r_esi = fr[0x1c];
            }
            x2 = fr[0x3c];
            x4 = fr[0x32];
            x1 = x2;
            x0 = x4;
            x1 = x1 & x3;
            x0 = x0 & x3;
            // First report call: ordering and sign flags plus row base.
            let b1 = if !is_nan(x1) && !is_nan(x0) && f32::from_bits(x1) > f32::from_bits(x0) {
                1u32
            } else {
                0u32
            };
            let b2 = if !is_nan(x4) && !is_nan(x6) && f32::from_bits(x4) >= f32::from_bits(x6) {
                1u32
            } else {
                0u32
            };
            let b3 = if !is_nan(x2) && !is_nan(x6) && f32::from_bits(x2) >= f32::from_bits(x6) {
                1u32
            } else {
                0u32
            };
            let ans6: u32 = lf_checker_rt::callee_cdecl!(
                6, u32, fr_addr(&fr, 0x270), r_esi, arg1, lf_checker_rt::relocated(0xae95b0), b3, b2, b1, 0
            );
            x3 = C_ABS;
            r_edi = arg1;
            if !mode_a {
                let _ = lf_checker_rt::callee_cdecl!(7, u32,);
                return ans6;
            }
            x1 = C_S2;
            x2 = C_S2B;
            r_ecx = 0;
            if (r_esi as i32) > 0 {
                loop {
                    r_eax = fr[((0x368 + r_ecx.wrapping_mul(4)) / 4) as usize];
                    r_eax = r_eax.wrapping_add(r_eax);
                    r_ecx = r_ecx.wrapping_add(1);
                    x0 = fr[((0x2c0 + r_eax.wrapping_mul(8)) / 4) as usize];
                    x0 = fmul(x0, x1);
                    x0 = fadd(x0, x2);
                    fr[((0x3b0 + r_ecx.wrapping_mul(8)) / 4) as usize] = x0;
                    x0 = fr[((0x2c4 + r_eax.wrapping_mul(8)) / 4) as usize];
                    x0 = fmul(x0, x1);
                    x0 = fadd(x0, x2);
                    fr[((0x3b4 + r_ecx.wrapping_mul(8)) / 4) as usize] = x0;
                    if !((r_ecx as i32) < (r_esi as i32)) {
                        break;
                    }
                }
            }
            x2 = fr[0x3c];
            x4 = fr[0x32];
            x0 = x4;
            x1 = x2;
            x0 = x0 & x3;
            x1 = x1 & x3;
            let b1 = if !is_nan(x1) && !is_nan(x0) && f32::from_bits(x1) > f32::from_bits(x0) {
                1u32
            } else {
                0u32
            };
            let b2 = if !is_nan(x4) && !is_nan(x6) && f32::from_bits(x4) >= f32::from_bits(x6) {
                1u32
            } else {
                0u32
            };
            let b3 = if !is_nan(x2) && !is_nan(x6) && f32::from_bits(x2) >= f32::from_bits(x6) {
                1u32
            } else {
                0u32
            };
            let ans6b: u32 = lf_checker_rt::callee_cdecl!(
                6, u32, fr_addr(&fr, 0x3b8), r_esi, arg1, lf_checker_rt::relocated(0xae93c0), b3, b2, b1, 0
            );
            let _ = lf_checker_rt::callee_cdecl!(7, u32,);
            return ans6b;
        } else {
            r_edi = arg1;
            if !mode_a {
                let _ = lf_checker_rt::callee_cdecl!(7, u32,);
                return global;
            }
            x1 = C_S2;
            x2 = C_S2B;
            r_ecx = 0;
            if (r_esi as i32) > 0 {
                loop {
                    r_eax = fr[((0x368 + r_ecx.wrapping_mul(4)) / 4) as usize];
                    r_eax = r_eax.wrapping_add(r_eax);
                    r_ecx = r_ecx.wrapping_add(1);
                    x0 = fr[((0x2c0 + r_eax.wrapping_mul(8)) / 4) as usize];
                    x0 = fmul(x0, x1);
                    x0 = fadd(x0, x2);
                    fr[((0x3b0 + r_ecx.wrapping_mul(8)) / 4) as usize] = x0;
                    x0 = fr[((0x2c4 + r_eax.wrapping_mul(8)) / 4) as usize];
                    x0 = fmul(x0, x1);
                    x0 = fadd(x0, x2);
                    fr[((0x3b4 + r_ecx.wrapping_mul(8)) / 4) as usize] = x0;
                    if !((r_ecx as i32) < (r_esi as i32)) {
                        break;
                    }
                }
            }
            x2 = fr[0x3c];
            x4 = fr[0x32];
            x0 = x4;
            x1 = x2;
            x0 = x0 & x3;
            x1 = x1 & x3;
            let b1 = if !is_nan(x1) && !is_nan(x0) && f32::from_bits(x1) > f32::from_bits(x0) {
                1u32
            } else {
                0u32
            };
            let b2 = if !is_nan(x4) && !is_nan(x6) && f32::from_bits(x4) >= f32::from_bits(x6) {
                1u32
            } else {
                0u32
            };
            let b3 = if !is_nan(x2) && !is_nan(x6) && f32::from_bits(x2) >= f32::from_bits(x6) {
                1u32
            } else {
                0u32
            };
            let ans6b: u32 = lf_checker_rt::callee_cdecl!(
                6, u32, fr_addr(&fr, 0x3b8), r_esi, arg1, lf_checker_rt::relocated(0xae93c0), b3, b2, b1, 0
            );
            let _ = lf_checker_rt::callee_cdecl!(7, u32,);
            return ans6b;
        }
    }
});
