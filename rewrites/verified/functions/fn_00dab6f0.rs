
// original: 0x00DAB6F0 ped_probe_gated_double (proposed)
//
// Gated double probe over a heading angle, an anchor triple and a target.
//
// Arguments (cdecl, five stack words): `a0` points to an object with a child
// pointer at +0x20 (a float is read at child+0x38); `a1` points to three
// anchor floats; `a2` is a heading angle; `a3` is a small mode integer (1
// selects the 0.5 scale, anything else 0.75) or zero, in which case a resolver
// callee derives it from `(a0, a1)`; the low byte of `a4` gates the first
// probe. Returns EAX 0/1.
//
// Behaviour: loads a lazily initialised global factor (flag word plus cached
// float; first use stores -0.3 and sets the flag). With `s = sin(a2)` and
// `c = cos(a2)` (scripted callees taking the angle in XMM0) it forms
// `tx = a1[0] - x*s`, `ty = x*c + a1[4]`, `tz = a1[8] + 0.6` and
// `q = tz - (child38 + 1.0)`, and clamps a budget to `q` when `q < 1.3`
// (NaN keeps 1.3, matching `comiss`+`jbe`). A probe descriptor (three copies
// of the global direction vector plus header/footer) is built on the stack.
// The first 9-argument probe `(a0, scratch+0x20, scratch, descriptor, 0.2f,
// 0, 1, 0x8e, 0)` runs only when the gate byte is zero and `q >= 0`
// (or NaN, matching `comiss`+`ja`); a nonzero answer returns 0. The second
// probe `(a0, scratch, scratch+0x30, descriptor, ...)` uses the rescaled
// `ux = tx - k*s`, `uy = k*c + ty`; a nonzero answer returns 0. Otherwise two
// 5-argument classifiers `(a0, a1, a2, mode, 0)` run in order; the first
// nonzero answer returns 1, else 0. Float operations are in the original's
// operand order, pinned against reassociation. (`scratch+0x28` holds a dead
// `tz - budget` store the original never reads back; it is reproduced for
// fidelity but observed nowhere.)
lf_checker_rt::export!(cdecl, rw_00dab6f0(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const RESOLVE_ID: u32 = 1;
        const SIN_ID: u32 = 2;
        const COS_ID: u32 = 3;
        const PROBE1_ID: u32 = 4;
        const PROBE2_ID: u32 = 5;
        const CLASS1_ID: u32 = 6;
        const CLASS2_ID: u32 = 7;
        const GFLAG: u32 = 0x017A652C;
        const GCACHED: u32 = 0x017A6528;
        const GDIR_X: u32 = 0x01B4B320;
        const GDIR_Y: u32 = 0x01B4B324;
        const GDIR_Z: u32 = 0x01B4B328;
        const NEG_TENTH3: u32 = 0x00FE8D74;
        const SIX_TENTHS: u32 = 0x00FE8858;
        const ONE3: u32 = 0x00FE892C;
        const ONE: u32 = 0x00FE88E8;
        const HALF: u32 = 0x00FE8830;
        const THREE_Q: u32 = 0x00FE888C;
        const FIFTH_BITS: u32 = 0x3E4CCCCD;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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

        let mode: u32 = if a3 == 0 {
            lf_checker_rt::callee_cdecl!(RESOLVE_ID, u32, a0, a1)
        } else {
            a3
        };

        let flag = rd32(lf_checker_rt::relocated(GFLAG));
        let x: f32 = if flag & 1 == 0 {
            let v = rd32(lf_checker_rt::relocated(NEG_TENTH3));
            (lf_checker_rt::global::<u32>(GFLAG) as *mut u32).write(flag | 1);
            (lf_checker_rt::global::<u32>(GCACHED) as *mut u32).write(v);
            f32::from_bits(v)
        } else {
            f32::from_bits(rd32(lf_checker_rt::relocated(GCACHED)))
        };

        let gx = rd32(lf_checker_rt::relocated(GDIR_X));
        let gy = rd32(lf_checker_rt::relocated(GDIR_Y));
        let gz = rd32(lf_checker_rt::relocated(GDIR_Z));
        let k_06 = f32::from_bits(rd32(lf_checker_rt::relocated(SIX_TENTHS)));
        let k_13 = f32::from_bits(rd32(lf_checker_rt::relocated(ONE3)));
        let k_10 = f32::from_bits(rd32(lf_checker_rt::relocated(ONE)));
        let k_half = f32::from_bits(rd32(lf_checker_rt::relocated(HALF)));
        let k_3q = f32::from_bits(rd32(lf_checker_rt::relocated(THREE_Q)));

        let a10 = f32::from_bits(rd32(a1));
        let a14 = f32::from_bits(rd32(a1 + 4));
        let a18 = f32::from_bits(rd32(a1 + 8));
        let child = rd32(a0 + 0x20);
        let mem38 = f32::from_bits(rd32(child + 0x38));

        let s = f32::from_bits(lf_checker_rt::callee_cdecl!(SIN_ID, u32, a2));
        let c = f32::from_bits(lf_checker_rt::callee_cdecl!(COS_ID, u32, a2));
        let tx = sub(a10, mul(x, s));
        let ty = add(mul(x, c), a14);
        let tz = add(a18, k_06);
        let q = sub(tz, add(mem38, k_10));
        // comiss 1.3, q / jbe: keep 1.3 unless q is strictly below it.
        let budget = if q < k_13 { q } else { k_13 };
        let adj = sub(tz, budget);

        // Mirror of the original's frame; index = E0 offset / 4.
        let mut frame = [0u32; 42];
        frame[2] = x.to_bits();
        frame[3] = s.to_bits();
        frame[4] = tx.to_bits();
        frame[5] = ty.to_bits();
        frame[6] = tz.to_bits();
        frame[11] = c.to_bits();
        frame[12] = tx.to_bits();
        frame[13] = ty.to_bits();
        frame[14] = adj.to_bits();
        frame[24] = gx;
        frame[25] = gy;
        frame[26] = gz;
        frame[28] = gx;
        frame[29] = gy;
        frame[30] = gz;
        frame[32] = gx;
        frame[33] = gy;
        frame[34] = gz;
        frame[39] = 0xFFFF;
        let p_scratch = frame.as_mut_ptr().add(4) as u32;
        let p_copy = frame.as_mut_ptr().add(12) as u32;
        let p_desc = frame.as_mut_ptr().add(20) as u32;
        let p_rescaled = frame.as_mut_ptr().add(16) as u32;

        if (a4 & 0xFF) == 0 && !(0.0 > q) {
            let r1: u32 = lf_checker_rt::callee_cdecl!(
                PROBE1_ID, u32, a0, p_copy, p_scratch, p_desc, FIFTH_BITS, 0, 1, 0x8E, 0
            );
            if r1 & 0xFF != 0 {
                return 0;
            }
        }
        let k = if mode == 1 { k_half } else { k_3q };
        let ux = sub(tx, mul(k, s));
        let uy = add(mul(k, c), ty);
        frame[16] = ux.to_bits();
        frame[17] = uy.to_bits();
        frame[18] = tz.to_bits();
        let r2: u32 = lf_checker_rt::callee_cdecl!(
            PROBE2_ID, u32, a0, p_scratch, p_rescaled, p_desc, FIFTH_BITS, 0, 1, 0x8E, 0
        );
        if r2 & 0xFF != 0 {
            return 0;
        }
        let r3: u32 = lf_checker_rt::callee_cdecl!(CLASS1_ID, u32, a0, a1, a2, mode, 0);
        if r3 & 0xFF != 0 {
            return 1;
        }
        let r4: u32 = lf_checker_rt::callee_cdecl!(CLASS2_ID, u32, a0, a1, a2, mode, 0);
        if r4 & 0xFF != 0 { 1 } else { 0 }
    }
});
