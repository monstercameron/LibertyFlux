// original: 0x00b06d90 CCamGame::vf5

/// Camera update step (thiscall, `this` in ECX, no stack arguments, returns 1
/// in AL). Runs four update callees over the camera object, marks the object
/// initialised (sets bit 0x80 of the flag byte at +0x1C4, or notifies a watcher
/// callee when already set), then, while a global singleton (callee 6) exists,
/// advances two clamped blend channels and finishes with three more update
/// callees (one of which is the neighbouring 0xB07100 entry, one the 0xB07600
/// entry; all intercepted, never re-entered).
///
/// Blend channel 1: add the step, raise to at least the cap, zero when
/// negative (without an upper clamp on that path), clamp down to the global
/// ceiling; NaN passes through unchanged (the original's comiss/jbe chain
/// keeps unordered values). When the channel-1 accumulator global reads
/// exactly +0.0 or -0.0, a packed counter global is advanced by `n + 3*(n/3)`
/// over `n = old + 1` (the original's multiply-by-0x55555555 division
/// sequence, replicated exactly); the lahf/test/jp idiom around it runs the
/// update only on ordered equality with zero. The accumulator then adds its
/// own step and saturates at 1.0, setting a done flag; if the gating callee
/// answers nonzero, either global inhibit flag is set, or the singleton is
/// inactive (with a fallback global checked), the accumulator and flag are
/// cleared instead.
/// Blend channel 2 follows the same shape with its own step/accumulator/done
/// flag, gated by the second gating callee, the same inhibit flags and the
/// singleton mode word reading exactly 1. At the end, flag bit 0x04 forces the
/// blend slot to 0. Float operation order is the original's.
lf_checker_rt::export!(thiscall, rw_00b06d90(this: u32) -> u32 {
    unsafe {
        const CC_BLEND: u32 = 0x74;
        const CC_FLAGS: u32 = 0x1C4;
        const FLAG_INIT: u8 = 0x80;
        const FLAG_ZERO_BLEND: u8 = 0x04;
        const G_CEIL: u32 = 0x00FE88E8;
        const G_INHIB_A: u32 = 0x01160C39;
        const G_INHIB_B: u32 = 0x017F5EB3;
        const G_STEP1: u32 = 0x01040070;
        const G_ACC1_STEP: u32 = 0x01040074;
        const G_STEP2: u32 = 0x01040078;
        const G_ACC2_STEP: u32 = 0x0104007C;
        const G_CAP: u32 = 0x01040080;
        const G_ACC1: u32 = 0x016154A4;
        const G_COUNT: u32 = 0x016154A8;
        const G_DONE1: u32 = 0x016154A0;
        const G_ACC2: u32 = 0x016154AC;
        const G_DONE2: u32 = 0x016154A1;
        const G_FALLBACK: u32 = 0x012BD193;
        const SING_ACTIVE: u32 = 0x210;
        const SING_MODE: u32 = 0xA70;

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
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
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
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        lf_checker_rt::callee_thiscall!(1, u32, this);
        lf_checker_rt::callee_thiscall!(2, u32, this);
        lf_checker_rt::callee_thiscall!(3, u32, this);
        lf_checker_rt::callee_thiscall!(4, u32, this);
        let flags = rd8(this + CC_FLAGS);
        if flags & FLAG_INIT == 0 {
            wr8(this + CC_FLAGS, flags | FLAG_INIT);
        } else {
            lf_checker_rt::callee_thiscall!(5, u32, this);
        }
        let sing: u32 = lf_checker_rt::callee_cdecl!(6, u32,);
        if sing != 0 {
            let gate1: u32 = lf_checker_rt::callee_cdecl!(7, u32,);
            let ceil = rdf(lf_checker_rt::relocated(G_CEIL));
            let inhibited = (gate1 as u8) != 0
                || rd8(lf_checker_rt::relocated(G_INHIB_A)) != 0
                || rd8(lf_checker_rt::relocated(G_INHIB_B)) != 0;
            if !inhibited {
                if rd8(sing + SING_ACTIVE) != 0 {
                    let mut x0 = add(rdf(this + CC_BLEND), rdf(lf_checker_rt::relocated(G_STEP1)));
                    let cap = rdf(lf_checker_rt::relocated(G_CAP));
                    if cap > x0 {
                        x0 = cap;
                    }
                    // Negative zeroes without an upper clamp; NaN is kept.
                    if x0 < 0.0 {
                        x0 = 0.0;
                    } else if x0 > ceil {
                        x0 = ceil;
                    }
                    wrf(this + CC_BLEND, x0);
                    let acc = rdf(lf_checker_rt::relocated(G_ACC1));
                    if acc == 0.0 {
                        // Ordered equality with zero only: the lahf/test/jp
                        // idiom skips this for NaN and all nonzero values.
                        let n = rd32(lf_checker_rt::relocated(G_COUNT)).wrapping_add(1);
                        let prod = 0x5555_5555i64 * ((n as i32) as i64);
                        let mut edx = (prod >> 32) as i32;
                        edx = edx.wrapping_sub(n as i32);
                        edx >>= 1;
                        let mut ecx = ((edx as u32) >> 31) as i32;
                        ecx = ecx.wrapping_add(edx);
                        let stepped = n
                            .wrapping_add(ecx as u32)
                            .wrapping_add((ecx as u32).wrapping_mul(2));
                        wr32(lf_checker_rt::relocated(G_COUNT), stepped);
                    }
                    let mut y0 = add(rdf(lf_checker_rt::relocated(G_ACC1_STEP)), acc);
                    wr8(lf_checker_rt::relocated(G_DONE1), 1);
                    if y0 > ceil {
                        y0 = 1.0;
                    }
                    wrf(lf_checker_rt::relocated(G_ACC1), y0);
                } else if rd8(lf_checker_rt::relocated(G_FALLBACK)) == 0 {
                    wrf(lf_checker_rt::relocated(G_ACC1), 0.0);
                    wr8(lf_checker_rt::relocated(G_DONE1), 0);
                }
            } else {
                wrf(lf_checker_rt::relocated(G_ACC1), 0.0);
                wr8(lf_checker_rt::relocated(G_DONE1), 0);
            }
            let gate2: u32 = lf_checker_rt::callee_cdecl!(8, u32,);
            let gated2 = (gate2 as u8) != 0
                || rd8(lf_checker_rt::relocated(G_INHIB_A)) != 0
                || rd8(lf_checker_rt::relocated(G_INHIB_B)) != 0
                || rd32(sing + SING_MODE) != 1;
            if !gated2 {
                let mut x0 = add(rdf(this + CC_BLEND), rdf(lf_checker_rt::relocated(G_STEP2)));
                let cap = rdf(lf_checker_rt::relocated(G_CAP));
                if cap > x0 {
                    x0 = cap;
                }
                let ceil2 = rdf(lf_checker_rt::relocated(G_CEIL));
                let x1 = if x0 < 0.0 {
                    0.0
                } else if x0 > ceil2 {
                    ceil2
                } else {
                    x0
                };
                wrf(this + CC_BLEND, x1);
                let mut z0 = add(rdf(lf_checker_rt::relocated(G_ACC2_STEP)), rdf(lf_checker_rt::relocated(G_ACC2)));
                wr8(lf_checker_rt::relocated(G_DONE2), 1);
                if z0 > ceil2 {
                    z0 = 1.0;
                }
                wrf(lf_checker_rt::relocated(G_ACC2), z0);
            } else {
                wrf(lf_checker_rt::relocated(G_ACC2), 0.0);
                wr8(lf_checker_rt::relocated(G_DONE2), 0);
            }
            lf_checker_rt::callee_thiscall!(9, u32, this);
        }
        lf_checker_rt::callee_thiscall!(10, u32, this);
        lf_checker_rt::callee_thiscall!(11, u32, this);
        if rd8(this + CC_FLAGS) & FLAG_ZERO_BLEND != 0 {
            wr32(this + CC_BLEND, 0);
        }
        1
    }
});
