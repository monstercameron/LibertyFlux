// original: 0x00abab00 input_ui_weighted_pick (proposed)

/// Pick up to two weighted entries from an input-ui object and report.
///
/// `arg0` selects a record through a global table indexed by the signed word
/// at `arg0+0x2e`; the record's object at `+0x12c` holds 11 use counts
/// (bytes at `+0x2c`), 11 rows of 8 candidate bytes (rows start at `+0x37`)
/// and a non-empty flag at `+0x8f`.
///
/// Behaviour: after two setup calls (which publish `arg5` and `arg6` to
/// globals and clear a third), an empty object returns the second setup
/// answer at once. Otherwise each of the 11 by 8 cells is tested by a check
/// call and kept when the call succeeds and the position is below the row's
/// count, forming an 11 by 8 grid. Two selection phases follow: each draws a
/// scaled random threshold against `arg3` (then `arg4`) and, when the draw
/// passes and cells remain, picks the drawn-th remaining kept cell; the
/// picked row byte scales a second draw into an argument for an assign call
/// on `arg1`. The second phase skips the first phase's row. A tail call
/// finally receives `arg0`, `arg1` and `arg2`, and its answer is returned.
///
/// Float order follows the original exactly (signed conversions, scale
/// multiplies before the count multiply, truncation toward zero with the
/// invalid result on overflow), as does the unsigned wrap of the pick index.
/// Table, record and seed addresses come from the original image. Original is
/// cdecl with seven stack words.
lf_checker_rt::export!(cdecl, rw_00abab00(
    arg0: u32,
    arg1: u32,
    arg2: u32,
    arg3: u32,
    arg4: u32,
    arg5: u32,
    arg6: u32,
) -> u32 {
    unsafe {
        const ROWS: u32 = 11;
        const COLS: u32 = 8;
        const COUNT_OFF: u32 = 0x2c;
        const CAND_OFF: u32 = 0x37;
        const NONEMPTY_OFF: u32 = 0x8f;
        const REC_ARG_OFF: u32 = 0x3c;
        const REC_OBJ_OFF: u32 = 0x12c;
        const SETUP_OBJ: u32 = 0x150e0f4;
        const GLOB_A: u32 = 0x150e0ec;
        const GLOB_B: u32 = 0x150e0f0;
        const GLOB_C: u32 = 0x150e128;
        const IDX_TABLE: u32 = 0x1295cd8;
        const IDX_WORD_OFF: u32 = 0x2e;
        const SCALE_T: u32 = 0xfe8684;
        const SCALE_P: u32 = 0xfe8680;
        const CAL_SETUP1: u32 = 1;
        const CAL_SETUP2: u32 = 2;
        const CAL_CHECK: u32 = 3;
        const CAL_RAND: u32 = 4;
        const CAL_ASSIGN: u32 = 5;
        const CAL_TAIL: u32 = 6;
        const CAL_COOKIE: u32 = 7;

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
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        /// Truncating float-to-int conversion with the original's invalid
        /// result (the most negative int) on NaN and overflow.
        #[inline(always)]
        fn cvt(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                i32::MIN
            } else {
                x as i32
            }
        }

        let idx = ((rd32(arg0 + IDX_WORD_OFF) & 0xffff) as u16) as i16 as i32;
        let rec = rd32(
            lf_checker_rt::relocated(IDX_TABLE).wrapping_add((idx as u32).wrapping_mul(4)),
        );
        let obj = rd32(rec + REC_OBJ_OFF);
        let setup1 =
            lf_checker_rt::callee_thiscall!(CAL_SETUP1, u32, lf_checker_rt::relocated(SETUP_OBJ), rd32(rec + REC_ARG_OFF));
        wr32(lf_checker_rt::relocated(GLOB_A), arg5);
        wr32(lf_checker_rt::relocated(GLOB_B), arg6);
        let setup2 = lf_checker_rt::callee_thiscall!(CAL_SETUP2, u32, setup1, arg2);
        wr32(lf_checker_rt::relocated(GLOB_C), 0);
        if rd8(obj + NONEMPTY_OFF) == 0 {
            lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
            return setup2;
        }
        let mut grid = [0u8; 88];
        let mut total = 0u32;
        for b in 0..ROWS {
            for s in 0..COLS {
                let ok = lf_checker_rt::callee_thiscall!(
                    CAL_CHECK,
                    u32,
                    setup1,
                    0x64 + b,
                    s
                ) as u8;
                if ok != 0 && s < rd8(obj + COUNT_OFF + b) as u32 {
                    grid[(b * COLS + s) as usize] = 1;
                    total += 1;
                }
            }
        }
        let scale_t = rdf(lf_checker_rt::relocated(SCALE_T));
        let scale_p = rdf(lf_checker_rt::relocated(SCALE_P));
        let mut sel_row = 0u32;
        // Phase one: threshold draw, then the drawn-th kept cell.
        let r1 = lf_checker_rt::callee_cdecl!(CAL_RAND, u32,) as i32;
        let mut remaining = total;
        if f32::from_bits(arg3) >= mul(r1 as f32, scale_t) && total > 0 {
            let r2 = lf_checker_rt::callee_cdecl!(CAL_RAND, u32,) as i32;
            let mut idx_p =
                cvt(mul(mul(((r2 & 0xffff) as f32), scale_p), total as f32)) as u32;
            let mut pick: Option<(u32, u32)> = None;
            for b in 0..ROWS {
                let c = rd8(obj + COUNT_OFF + b) as u32;
                if c == 0 {
                    continue;
                }
                for s in 0..c {
                    if grid[(b * COLS + s) as usize] == 1 {
                        let e = idx_p;
                        idx_p = idx_p.wrapping_sub(1);
                        if e == 0 {
                            pick = Some((b, s));
                            break;
                        }
                    }
                }
                if pick.is_some() {
                    break;
                }
            }
            if let Some((b, s)) = pick {
                let val = rd8(obj + CAND_OFF + b * COLS + s) as i32;
                let r3 = lf_checker_rt::callee_cdecl!(CAL_RAND, u32,) as i32;
                let computed = cvt(mul(
                    val as f32,
                    mul(((r3 & 0xffff) as f32), scale_p),
                ));
                lf_checker_rt::callee_thiscall!(
                    CAL_ASSIGN,
                    u32,
                    arg1,
                    b,
                    s,
                    computed as u32,
                    0xffffffff,
                    0u32,
                    0u32
                );
                sel_row = b;
            }
            // The first phase's row leaves the pool for phase two.
            let c = rd8(obj + COUNT_OFF + sel_row) as u32;
            for s in 0..c {
                if grid[(sel_row * COLS + s) as usize] == 1 {
                    remaining -= 1;
                }
            }
            // Phase two: same shape over every row but the picked one.
            let r4 = lf_checker_rt::callee_cdecl!(CAL_RAND, u32,) as i32;
            if f32::from_bits(arg4) >= mul(r4 as f32, scale_t) && remaining > 0 {
                let r5 = lf_checker_rt::callee_cdecl!(CAL_RAND, u32,) as i32;
                let mut idx_q = cvt(mul(
                    mul(((r5 & 0xffff) as f32), scale_p),
                    remaining as f32,
                )) as u32;
                let mut pick2: Option<(u32, u32)> = None;
                for b in 0..ROWS {
                    if b == sel_row {
                        continue;
                    }
                    let c2 = rd8(obj + COUNT_OFF + b) as u32;
                    if c2 == 0 {
                        continue;
                    }
                    for s in 0..c2 {
                        if grid[(b * COLS + s) as usize] == 1 {
                            let e = idx_q;
                            idx_q = idx_q.wrapping_sub(1);
                            if e == 0 {
                                pick2 = Some((b, s));
                                break;
                            }
                        }
                    }
                    if pick2.is_some() {
                        break;
                    }
                }
                if let Some((b, s)) = pick2 {
                    let val = rd8(obj + CAND_OFF + b * COLS + s) as i32;
                    let r6 = lf_checker_rt::callee_cdecl!(CAL_RAND, u32,) as i32;
                    let computed = cvt(mul(
                        val as f32,
                        mul(((r6 & 0xffff) as f32), scale_p),
                    ));
                    lf_checker_rt::callee_thiscall!(
                        CAL_ASSIGN,
                        u32,
                        arg1,
                        b,
                        s,
                        computed as u32,
                        0xffffffff,
                        0u32,
                        0u32
                    );
                }
            }
        }
        let ans = lf_checker_rt::callee_cdecl!(CAL_TAIL, u32, arg0, arg1, arg2);
        lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
        ans
    }
});
