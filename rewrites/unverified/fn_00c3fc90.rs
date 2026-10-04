// original: 0x00c3fc90 timing_mode_dispatch (proposed)

/// Run one dispatch step: sample the source, pick a mode, publish outputs.
///
/// No arguments. Three locals start at 0.0, 0.0 and mode 4. When the enable
/// byte is set, a poll callee is asked and a positive answer fills the
/// locals through a sampler callee. The first local must then reach 100.0
/// without passing 145.0, and when the hold byte is set the stored level
/// must not be positive, else the step resets (mode 4, stored peak zeroed).
///
/// Otherwise the mode word decides the peak: modes equal to 4 or to the
/// sampled mode reuse the stored peak, any other mode stores the second
/// local as the new peak. A non-positive peak becomes 1.0; otherwise the
/// second local is divided by it and one minus that quotient is clamped to
/// [0, 1] (NaN-aware: unordered comparisons take the reset/clamp branch the
/// original's `jb`/`jbe` take). The clamped value drives a math callee
/// (scaled by 0.0225 into the third output) and a four-way mode switch
/// producing the first output from 0.06/0.12 steps; an out-of-range mode
/// yields 0.0 and mode 4. The second output is always zero. Finally the mode
/// word is published, the clock (negated for mode 4) joins the stored
/// level, the hold byte records whether the mode is 4, and the level is
/// clamped to [0, 1].
///
/// Original: 0x00c3fc90 (cdecl, no stack words). No return value.
lf_checker_rt::export!(cdecl, rw_00c3fc90() -> u32 {
    unsafe {
        fc90_core(false);
        0
    }
});

unsafe fn fc90_core(plain_cmp: bool) -> u32 {
    unsafe {
        const K_100: f32 = 100.0;
        const K_145: f32 = 145.0;
        const K_00225: f32 = f32::from_bits(0x3cb8_51ec); // 0.0225
        const K_012: f32 = f32::from_bits(0x3df5_c28f); // 0.12
        const K_006: f32 = f32::from_bits(0x3d75_c28f); // 0.06
        const K_N006: f32 = f32::from_bits(0xbd75_c28f); // -0.06
        const K_TWO: f32 = 2.0;
        const K_PI: f32 = f32::from_bits(0x4049_0fdb);
        const SRC_OBJ: u32 = 0x1284a60;
        const C_POLL: u32 = 1;
        const C_SAMPLE: u32 = 2;
        const C_MATH: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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
            f32::from_bits(a.to_bits() ^ 0x8000_0000)
        }
        // comiss jb/jbe take the branch on unordered (NaN); plain Rust
        // </<= return false there, so the negated forms are used.
        #[inline(always)]
        fn below(a: f32, b: f32, plain: bool) -> bool {
            if plain {
                a < b
            } else {
                !(a >= b)
            }
        }
        #[inline(always)]
        fn below_eq(a: f32, b: f32) -> bool {
            !(a > b)
        }

        let g_enable = lf_checker_rt::global::<u8>(0x16d21a4);
        let g_hold = lf_checker_rt::global::<u8>(0x16d21a5);
        let g_level = lf_checker_rt::global::<f32>(0x16d21a8);
        let g_peak = lf_checker_rt::global::<f32>(0x16d21ac);
        let g_out0 = lf_checker_rt::global::<f32>(0x16d21b0);
        let g_out1 = lf_checker_rt::global::<u32>(0x16d21b4);
        let g_out2 = lf_checker_rt::global::<f32>(0x16d21b8);
        let g_mode = lf_checker_rt::global::<u32>(0x1048acc);
        let g_clock = lf_checker_rt::global::<f32>(0x11735cc);

        let mut l0 = 0.0f32;
        let mut l1 = 0.0f32;
        let mut l2 = 4u32;
        let mut reset = g_enable.read() == 0;
        if !reset {
            let src = lf_checker_rt::relocated(SRC_OBJ);
            let n: u32 = lf_checker_rt::callee_thiscall!(C_POLL, u32, src);
            if (n as i32) > 0 {
                lf_checker_rt::callee_thiscall!(
                    C_SAMPLE,
                    u32,
                    src,
                    core::ptr::addr_of_mut!(l0) as u32,
                    core::ptr::addr_of_mut!(l1) as u32,
                    core::ptr::addr_of_mut!(l2) as u32,
                    0
                );
            }
            if below(l0, K_100, plain_cmp) {
                reset = true;
            } else if below(K_145, l0, false) {
                reset = true;
            } else if g_hold.read() != 0 && g_level.read() > 0.0 {
                reset = true;
            }
        }
        let mut mode: u32;
        if reset {
            mode = 4;
            g_peak.write(0.0);
        } else {
            let e = g_mode.read();
            let peak: f32;
            if e == 4 || e == l2 {
                peak = g_peak.read();
            } else {
                g_peak.write(l1);
                peak = l1;
            }
            let clamped: f32;
            if below_eq(peak, 0.0) {
                clamped = 1.0;
            } else {
                let q = div(l1, peak);
                let c0 = sub(1.0, q);
                l0 = c0;
                if below_eq(0.0, c0) {
                    if below_eq(c0, 1.0) {
                        clamped = c0;
                    } else {
                        clamped = 1.0;
                    }
                } else {
                    clamped = 0.0;
                }
            }
            l0 = clamped;
            let w: u32 = lf_checker_rt::callee_cdecl!(
                C_MATH,
                u32,
                mul(mul(clamped, K_TWO), K_PI).to_bits()
            );
            let scaled = mul(f32::from_bits(w), K_00225);
            let x1: f32;
            if l2 > 3 {
                x1 = 0.0;
                mode = 4;
            } else {
                mode = l2;
                x1 = match l2 {
                    0 => K_006,
                    1 => sub(K_006, mul(l0, K_012)),
                    2 => K_N006,
                    _ => sub(mul(l0, K_012), K_006),
                };
            }
            g_out0.write(x1);
            g_out1.write(0);
            g_out2.write(scaled);
        }
        let mut t = g_clock.read();
        g_mode.write(mode);
        let is4 = mode == 4;
        if is4 {
            t = neg(t);
        }
        t = add(t, g_level.read());
        g_hold.write(is4 as u8);
        g_level.write(t);
        if below_eq(0.0, t) {
            if !below_eq(t, 1.0) {
                g_level.write(1.0);
            }
        } else {
            g_level.write(0.0);
        }
        0
    }
}
