
// original: 0x00DABCA0 ped_probe_looped (proposed)
//
// Looped probe sweep (three iterations) with coherence checks and a final probe.
//
// Arguments (cdecl, five stack words): `a0` is an opaque context pointer passed
// through to every probe call; `edi` points to three floats; `a2` is a heading
// angle; `a3` is a mode integer (1 selects the 0.14 scale, anything else 0.45)
// or zero, in which case a resolver callee derives it from `(a0, edi)`; `a4`
// is an optional out-pointer for one float (may be null). Returns 1 in AL on
// success, else 0 (upper EAX bits follow the last callee answer; scripts only
// use 0/1 so the result is exactly 0/1).
//
// Behaviour: with `s = sin(a2)`, `c = cos(a2)` (scripted callees taking the
// angle in XMM0) and the selected scale, each iteration forms a probe pair
// (`gx`, `sum`) from the anchor floats and calls the 9-argument probe with
// `(a0, pair, pair_copy, descriptor, 0.095f, 0, 1, 0x8e, 0)`, where the
// descriptor repeats the global direction vector three times with
// header/footer. A zero answer returns 0. Otherwise three coherence checks run:
// the squared distance of the answered pair from the global (x, y) must not
// exceed 0.009025; the global z minus `edi[8]` must lie in [-0.5, 0.5]; past
// the first iteration the running mean of global z must stay within 0.3 of it
// (the mean provably equals it up to float rounding, so only the pass edge is
// reachable). Running sums and a running maximum are kept. After three
// iterations the mean is divided by three (multiply by 1/3), two more lazily
// initialised globals are loaded (flag word plus two cached floats; first use
// stores 0.21 and 1.6 minus that), and the final 9-argument probe runs with
// `(a0, acc, acc_copy, descriptor, 0.16f, 0, 0, 0x8e, 0)`; here a NONZERO
// answer returns 0, while zero stores the mean through `a4` when non-null and
// returns 1. Float operations are in the original's operand order, pinned
// against reassociation; NaN follows the original's unordered-compare paths
// (every `ja` check passes NaN, the maximum keeps its old value).
// Two scratch words the original writes (`edi[8]+1.4` and that minus 1.9) are
// never read back on any path and are not reproduced.
lf_checker_rt::export!(cdecl, rw_00dabca0(a0: u32, edi: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const RESOLVE_ID: u32 = 1;
        const SIN_ID: u32 = 2;
        const COS_ID: u32 = 3;
        const LOOP_ID: u32 = 4;
        const FINAL_ID: u32 = 5;
        const SEL_ONE: u32 = 0x00EE3ED8;
        const SEL_OTHER: u32 = 0x00FE8828;
        const K095: u32 = 0x00EF0234;
        const GDIR_X: u32 = 0x01B4B320;
        const GDIR_Y: u32 = 0x01B4B324;
        const GDIR_Z: u32 = 0x01B4B328;
        const KMIN: u32 = 0x00FE8E1C;
        const K9025: u32 = 0x00EF0230;
        const KNEG05: u32 = 0x00FE8D7C;
        const K05: u32 = 0x00FE8830;
        const ABS_MASK: u32 = 0x00FE8F80;
        const K03: u32 = 0x00FE87E8;
        const KTHIRD: u32 = 0x00FE8808;
        const FLAG2: u32 = 0x017A6520;
        const C2A: u32 = 0x017A651C;
        const C2B: u32 = 0x017A6524;
        const K021: u32 = 0x00EF0238;
        const K16: u32 = 0x00FE897C;
        const LOOP_K: u32 = 0x3DC28F5C;
        const FINAL_K: u32 = 0x3E23D70A;

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
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        let mode: u32 = if a3 == 0 {
            lf_checker_rt::callee_cdecl!(RESOLVE_ID, u32, a0, edi)
        } else {
            a3
        };
        let sel = f32::from_bits(rd32(lf_checker_rt::relocated(
            if mode == 1 { SEL_ONE } else { SEL_OTHER },
        )));
        let s = f32::from_bits(lf_checker_rt::callee_cdecl!(SIN_ID, u32, a2));
        let c = f32::from_bits(lf_checker_rt::callee_cdecl!(COS_ID, u32, a2));

        let edi0 = f32::from_bits(rd32(edi));
        let edi4 = f32::from_bits(rd32(edi + 4));
        let edi8 = f32::from_bits(rd32(edi + 8));
        let gx_g = f32::from_bits(rd32(lf_checker_rt::relocated(GDIR_X)));
        let gy_g = f32::from_bits(rd32(lf_checker_rt::relocated(GDIR_Y)));
        let gz_g = f32::from_bits(rd32(lf_checker_rt::relocated(GDIR_Z)));
        let k095 = f32::from_bits(rd32(lf_checker_rt::relocated(K095)));
        let k9025 = f32::from_bits(rd32(lf_checker_rt::relocated(K9025)));
        let kneg05 = f32::from_bits(rd32(lf_checker_rt::relocated(KNEG05)));
        let k05 = f32::from_bits(rd32(lf_checker_rt::relocated(K05)));
        let abs_bits = rd32(lf_checker_rt::relocated(ABS_MASK));
        let k03 = f32::from_bits(rd32(lf_checker_rt::relocated(K03)));
        let kthird = f32::from_bits(rd32(lf_checker_rt::relocated(KTHIRD)));

        let p1 = mul(s, sel);
        let p3 = mul(s, k095);
        let p7 = mul(c, sel);
        let p4 = mul(c, k095);
        let sum7 = add(p7, edi4);
        let mut sum7r = sum7;
        let mut gx = sub(edi0, p1);

        // Mirror of the original's frame; index = E0 offset / 4.
        let mut frame = [0u32; 55];
        frame[20] = gx_g.to_bits();
        frame[21] = gy_g.to_bits();
        frame[22] = gz_g.to_bits();
        frame[24] = gx_g.to_bits();
        frame[25] = gy_g.to_bits();
        frame[26] = gz_g.to_bits();
        frame[28] = gx_g.to_bits();
        frame[29] = gy_g.to_bits();
        frame[30] = gz_g.to_bits();
        frame[35] = 0xFFFF;
        let p_pair = frame.as_mut_ptr().add(40) as u32;
        let p_pair2 = frame.as_mut_ptr().add(52) as u32;
        let p_desc = frame.as_mut_ptr().add(16) as u32;

        let mut acc20 = 0.0f32;
        let mut acc38 = 0.0f32;
        let mut mean10 = 0.0f32;
        let mut cur8 = f32::from_bits(rd32(lf_checker_rt::relocated(KMIN)));
        let mut d1 = 0.0f32;
        let mut d0 = 0.0f32;
        for k in 0..3u32 {
            gx = sub(gx, p3);
            let sum_a = add(p4, sum7r);
            frame[40] = gx.to_bits();
            frame[41] = sum_a.to_bits();
            frame[52] = gx.to_bits();
            frame[53] = sum_a.to_bits();
            let r: u32 = lf_checker_rt::callee_cdecl!(
                LOOP_ID, u32, a0, p_pair, p_pair2, p_desc, LOOP_K, 0, 1, 0x8E, 0
            );
            if r & 0xFF == 0 {
                return 0;
            }
            // Post-write readback, exactly like the original.
            let w0 = f32::from_bits(frame[40]);
            let w1 = f32::from_bits(frame[41]);
            gx = w0;
            sum7r = w1;
            d1 = sub(gx_g, w0);
            d0 = sub(gy_g, w1);
            let dist = add(mul(d1, d1), mul(d0, d0));
            // comiss dist, 0.009025 / ja: NaN passes.
            if dist > k9025 {
                return 0;
            }
            let dd = sub(gz_g, edi8);
            // comiss -0.5, dd / ja, then comiss dd, 0.5 / ja: NaN passes both.
            if kneg05 > dd {
                return 0;
            }
            if dd > k05 {
                return 0;
            }
            if k > 0 {
                let kf = k as f32;
                let rr = sub(div(mean10, kf), gz_g);
                let ar = f32::from_bits(rr.to_bits() & abs_bits);
                if ar > k03 {
                    return 0;
                }
            }
            acc20 = add(gx_g, acc20);
            acc38 = add(gy_g, acc38);
            mean10 = add(gz_g, mean10);
            frame[8] = acc20.to_bits();
            frame[14] = acc38.to_bits();
            frame[4] = mean10.to_bits();
            // comiss gz, cur8 / jbe: keep the running maximum.
            if gz_g > cur8 {
                cur8 = gz_g;
            }
        }

        let acc20s = mul(acc20, kthird);
        let acc38s = mul(acc38, kthird);
        mean10 = mul(mean10, kthird);
        frame[4] = mean10.to_bits();

        let flag = rd32(lf_checker_rt::relocated(FLAG2));
        let c2a: f32 = if flag & 1 == 0 {
            let v = rd32(lf_checker_rt::relocated(K021));
            (lf_checker_rt::global::<u32>(FLAG2) as *mut u32).write(flag | 1);
            (lf_checker_rt::global::<u32>(C2A) as *mut u32).write(v);
            f32::from_bits(v)
        } else {
            f32::from_bits(rd32(lf_checker_rt::relocated(C2A)))
        };
        let flag2 = rd32(lf_checker_rt::relocated(FLAG2));
        let c2b: f32 = if flag2 & 2 == 0 {
            let k16 = f32::from_bits(rd32(lf_checker_rt::relocated(K16)));
            let v = sub(k16, c2a);
            (lf_checker_rt::global::<u32>(FLAG2) as *mut u32).write(flag2 | 2);
            (lf_checker_rt::global::<u32>(C2B) as *mut u32).write(v.to_bits());
            v
        } else {
            f32::from_bits(rd32(lf_checker_rt::relocated(C2B)))
        };

        let f2a = add(cur8, c2a);
        let f2b = add(f2a, c2b);
        frame[48] = acc20s.to_bits();
        frame[49] = acc38s.to_bits();
        frame[50] = f2a.to_bits();
        frame[44] = acc20s.to_bits();
        frame[45] = acc38s.to_bits();
        frame[46] = f2b.to_bits();
        let p_acc = frame.as_mut_ptr().add(48) as u32;
        let p_acc2 = frame.as_mut_ptr().add(44) as u32;
        let rf: u32 = lf_checker_rt::callee_cdecl!(
            FINAL_ID, u32, a0, p_acc, p_acc2, p_desc, FINAL_K, 0, 0, 0x8E, 0
        );
        if rf & 0xFF != 0 {
            return 0;
        }
        if a4 != 0 {
            (a4 as *mut u32).write(mean10.to_bits());
        }
        1
    }
});
