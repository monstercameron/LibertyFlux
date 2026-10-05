// original: 0x00b75320 task_candidate_probe_loop (proposed)

/// Probe up to 32 candidate records for one the handler accepts, reporting
/// through an out-byte and a boolean return.
///
/// Arguments (thiscall): `this` is the owner object (status byte at `+0x29`,
/// bit 1 = strict; mode byte at `+0x2a`, bit 0x20 selects the alternate
/// scale); `out` receives 0, or 1 once the first handler probe succeeds.
///
/// Behaviour in order:
/// 1. Scale: `f = base * factor`, where `base` is the shared base value and
///    `factor` is the alternate scale when the mode bit is set, else the
///    default scale (pristine: 1.0 times 255.0 or 155.0). Clear `*out`.
/// 2. First probe: call the handler (thiscall on `this`) with the table
///    word selected by the shared index (-1 maps to 0) and the constant
///    `C1`. When it succeeds, set `*out` to 1 and, unless strict mode is
///    set, return 0.
/// 3. Gate: call the predicate (no arguments). When it succeeds, clamp `f`
///    down to `cap_base * cap_scale` (pristine 200.0 * 0.9 = 180.0) if `f`
///    is strictly greater (NaN-safe: unordered keeps `f`).
/// 4. Second probe: call the handler again with the same table word and
///    `f * f`. When it fails, return 0. Call the predicate again; when it
///    fails, return 1 without scanning.
/// 5. Scan candidates 0..32: fetch candidate `i` (cdecl, may return null);
///    skip nulls and records whose word at `+0x598` is zero; skip records
///    the filter (thiscall on the candidate) accepts. For the rest, when
///    the word at `+0x578` is nonzero, rank the record against the current
///    pick (two rank calls, compared as signed bytes): a strictly greater
///    rank offers the record to the handler with constant `C2` at once,
///    otherwise the record goes through the strict gate (non-strict always
///    offers with `C1`; strict offers with `C1` only while the shared stop
///    flag byte is clear, else the candidate is skipped). The first offer
///    the handler accepts returns 0; an exhausted scan returns 1.
///
/// All shared words, the index table, the stop flag and the float constants
/// are read from the relocated image; float order is pinned with
/// `black_box` on both operands. Only the low byte of the return matters
/// (the original leaves the upper bytes from earlier values).
///
/// Original: 0x00B75320 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b75320(this: u32, out: u32) -> u32 {
    unsafe {
        const FACTOR_DEFAULT: u32 = 0x00EB_229C;
        const FACTOR_ALT: u32 = 0x00FE_8C08;
        const BASE: u32 = 0x0103_F6BC;
        const CAP_BASE: u32 = 0x0110_E5C4;
        const CAP_SCALE: u32 = 0x00FE_88BC;
        const INDEX: u32 = 0x0103_6F14;
        const TABLE: u32 = 0x011A_8808;
        const STOP_FLAG: u32 = 0x0167_CA1B;
        const STATUS_OFF: u32 = 0x29;
        const MODE_OFF: u32 = 0x2A;
        const STRICT_BIT: u8 = 2;
        const ALT_BIT: u8 = 0x20;
        const C1: u32 = 0x4661_0000;
        const C2: u32 = 0x474E_A400;
        const CAND_FILTER_OFF: u32 = 0x598;
        const CAND_RANK_OFF: u32 = 0x578;
        const MAX_CAND: u32 = 32;
        const HANDLER: u32 = 0;
        const FETCH: u32 = 1;
        const FILTER: u32 = 2;
        const CURRENT: u32 = 3;
        const RANK: u32 = 4;
        const PRED: u32 = 5;

        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
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
        unsafe fn table_word() -> u32 {
            unsafe {
                let idx = rd32(lf_checker_rt::relocated(INDEX));
                if idx == 0xFFFF_FFFF {
                    0
                } else {
                    rd32(lf_checker_rt::relocated(TABLE) + idx.wrapping_mul(4))
                }
            }
        }

        let status = ((this + STATUS_OFF) as *const u8).read();
        let mode = ((this + MODE_OFF) as *const u8).read();
        let factor = rdf(lf_checker_rt::relocated(
            if mode & ALT_BIT != 0 { FACTOR_ALT } else { FACTOR_DEFAULT },
        ));
        let mut f = fmul(rdf(lf_checker_rt::relocated(BASE)), factor);
        (out as *mut u8).write(0);

        // First probe.
        let t = table_word();
        let r1: u8 = lf_checker_rt::callee_thiscall!(HANDLER, u8, this, t, C1);
        if r1 != 0 {
            (out as *mut u8).write(1);
            if status & STRICT_BIT == 0 {
                return 0;
            }
        }

        // Gate + clamp.
        let g1: u8 = lf_checker_rt::callee_cdecl!(PRED, u8,);
        if g1 != 0 {
            let cap = fmul(rdf(lf_checker_rt::relocated(CAP_BASE)), rdf(lf_checker_rt::relocated(CAP_SCALE)));
            if f > cap {
                f = cap;
            }
        }

        // Second probe.
        let t2 = table_word();
        let f2 = fmul(f, f);
        let r2: u8 = lf_checker_rt::callee_thiscall!(HANDLER, u8, this, t2, f2.to_bits());
        if r2 == 0 {
            return 0;
        }
        let g2: u8 = lf_checker_rt::callee_cdecl!(PRED, u8,);
        if g2 == 0 {
            return 1;
        }

        // Candidate scan.
        let mut i = 0u32;
        while i < MAX_CAND {
            let cand: u32 = lf_checker_rt::callee_cdecl!(FETCH, u32, i);
            if cand != 0 && rd32(cand + CAND_FILTER_OFF) != 0 {
                let keep: u8 = lf_checker_rt::callee_thiscall!(FILTER, u8, cand);
                if keep == 0 {
                    let w = rd32(cand + CAND_RANK_OFF);
                    let mut offered_c2 = false;
                    if w != 0 {
                        let cur: u32 = lf_checker_rt::callee_cdecl!(CURRENT, u32,);
                        let v1 = rd32(cur + CAND_RANK_OFF);
                        let b: u8 = lf_checker_rt::callee_thiscall!(RANK, u8, v1);
                        let v2 = rd32(cand + CAND_RANK_OFF);
                        let a: u8 = lf_checker_rt::callee_thiscall!(RANK, u8, v2);
                        if (a as i8) > (b as i8) {
                            let ok: u8 =
                                lf_checker_rt::callee_thiscall!(HANDLER, u8, this, cand, C2);
                            if ok != 0 {
                                return 0;
                            }
                            offered_c2 = true;
                        }
                    }
                    if !offered_c2 {
                        let offer = if status & STRICT_BIT == 0 {
                            true
                        } else {
                            ((lf_checker_rt::relocated(STOP_FLAG)) as *const u8).read() == 0
                        };
                        if offer {
                            let ok: u8 =
                                lf_checker_rt::callee_thiscall!(HANDLER, u8, this, cand, C1);
                            if ok != 0 {
                                return 0;
                            }
                        }
                    }
                }
            }
            i += 1;
        }
        1
    }
});
