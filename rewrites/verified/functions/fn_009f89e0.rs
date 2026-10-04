// original: 0x009F89E0 stat_command_dispatch (proposed)

/// Run one statistic-console command selected by a numeric key.
///
/// Most keys do nothing. Eleven keys act: four run a ratio report (two
/// sampler pairs are read through a float-returning callee, and when both
/// pair sums are positive their ratio, scaled by two global factors, is
/// recorded against a fixed stat); three clear the eight trip-counter words;
/// four record float 1.0 against another fixed stat and, unless the
/// generation word already exceeds a global limit, reset it to the limit
/// plus 10000; one clears the generation word alone. The original encodes
/// the key space as two byte maps over jump tables; only the eleven live
/// mappings are reproduced here (verified against the image by the contract
/// generator), everything else returns.
///
/// The batch lists this entry as 366 bytes, but the code is 157 bytes plus
/// two shared case chunks (62 and 147 bytes) reached by tail jumps, whose
/// behaviour is inlined below.
///
/// Original: cdecl, one stack word (key), no meaningful return value.
lf_checker_rt::export!(cdecl, rw_009F89E0(key: u32) -> u32 {
    unsafe {
        const HI_KEY_BASE: u32 = 0x126;
        const HI_KEY_MAX: u32 = 0x99;
        const LO_KEY_BASE: u32 = 0x47;
        const LO_KEY_MAX: u32 = 0xb6;
        const DIRECT_KEY: u32 = 0xfe;
        const FIELD_6268: u32 = 0x12b6268;
        const FIELD_6262: u32 = 0x12b6262;
        const FIELD_6270: u32 = 0x12b6270;
        const FIELD_6274: u32 = 0x12b6274;
        const FIELD_6278: u32 = 0x12b6278;
        const FIELD_627C: u32 = 0x12b627c;
        const GEN_WORD: u32 = 0x12b628c;
        const GEN_LIMIT: u32 = 0x11735b4;
        const SCALE_A: u32 = 0xfe86b4;
        const SCALE_B: u32 = 0xfe8724;
        const SAMPLE_A1: u32 = 0x4e;
        const SAMPLE_A2: u32 = 0x4d;
        const SAMPLE_B1: u32 = 0x49;
        const SAMPLE_B2: u32 = 0x47;
        const RATIO_STAT: u32 = 0x2a;
        const TOUCH_STAT: u32 = 0x107;
        const GEN_BUMP: u32 = 0x2710;
        const ONE_BITS: u32 = 0x3f80_0000;
        const SAMPLE_CALLEE_A1: u32 = 0;
        const SAMPLE_CALLEE_A2: u32 = 1;
        const SAMPLE_CALLEE_B1: u32 = 2;
        const SAMPLE_CALLEE_B2: u32 = 3;
        const RATIO_CALLEE: u32 = 4;
        const TOUCH_CALLEE: u32 = 5;

        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        /// Clear the eight trip-counter words.
        #[inline(always)]
        unsafe fn clear_trip() {
            unsafe {
                ((lf_checker_rt::relocated(FIELD_6268)) as *mut u64).write_unaligned(0);
                ((lf_checker_rt::relocated(FIELD_6262)) as *mut u8).write(0);
                wr32(lf_checker_rt::relocated(FIELD_6270), 0);
                wr32(lf_checker_rt::relocated(FIELD_6274), 0);
                wr32(lf_checker_rt::relocated(FIELD_6278), 0);
                ((lf_checker_rt::relocated(FIELD_627C)) as *mut u16).write_unaligned(0);
                wr32(lf_checker_rt::relocated(GEN_WORD), 0);
            }
        }
        /// Read two sampler pairs and record their scaled ratio when both
        /// pair sums are positive.
        #[inline(always)]
        unsafe fn report_ratio() {
            unsafe {
                let a1: f32 =
                    lf_checker_rt::callee_cdecl!(SAMPLE_CALLEE_A1, f32, SAMPLE_A1);
                let a2: f32 =
                    lf_checker_rt::callee_cdecl!(SAMPLE_CALLEE_A2, f32, SAMPLE_A2);
                let first = add(a1, a2);
                let b1: f32 =
                    lf_checker_rt::callee_cdecl!(SAMPLE_CALLEE_B1, f32, SAMPLE_B1);
                let b2: f32 =
                    lf_checker_rt::callee_cdecl!(SAMPLE_CALLEE_B2, f32, SAMPLE_B2);
                let second = add(b1, b2);
                // Each gate is comiss+jae: it skips only on an ordered
                // less-or-equal, so a NaN sum proceeds, not skips.
                if !(second <= 0.0) && !(first <= 0.0) {
                    let scaled = mul(
                        mul(mul(second, rdf(lf_checker_rt::relocated(SCALE_A))),
                            rdf(lf_checker_rt::relocated(SCALE_B))),
                        rdf(lf_checker_rt::relocated(SCALE_B)),
                    );
                    let ratio = div(first, scaled);
                    lf_checker_rt::callee_cdecl!(RATIO_CALLEE, u32, RATIO_STAT, ratio.to_bits());
                }
            }
        }
        /// Record a touch and reset the generation word unless it already
        /// exceeds the limit.
        #[inline(always)]
        unsafe fn touch_and_reset() {
            unsafe {
                let current = rd32(lf_checker_rt::relocated(GEN_WORD));
                let limit = rd32(lf_checker_rt::relocated(GEN_LIMIT));
                lf_checker_rt::callee_cdecl!(TOUCH_CALLEE, u32, TOUCH_STAT, ONE_BITS);
                if current <= limit {
                    wr32(lf_checker_rt::relocated(GEN_WORD), limit.wrapping_add(GEN_BUMP));
                }
            }
        }

        if key > DIRECT_KEY {
            let k = key.wrapping_sub(HI_KEY_BASE);
            if k > HI_KEY_MAX {
                return 0;
            }
            match k {
                0 | 1 | 2 | 3 => touch_and_reset(),
                0x7d => clear_trip(),
                0x99 => wr32(lf_checker_rt::relocated(GEN_WORD), 0),
                _ => {}
            }
        } else if key == DIRECT_KEY {
            clear_trip();
        } else {
            let k = key.wrapping_sub(LO_KEY_BASE);
            if k > LO_KEY_MAX {
                return 0;
            }
            match k {
                0 | 2 | 6 | 7 => report_ratio(),
                0xb6 => clear_trip(),
                _ => {}
            }
        }
        0
    }
});
