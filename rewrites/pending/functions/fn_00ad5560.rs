// original: 0x00ad5560 audio_faded_lookup_with_report
// ---------------------------------------------------------------------------
// 0x00AD5560: faded table lookup with a reporting call.
// ---------------------------------------------------------------------------
// Like the neighbouring blended lookup, with the window anchors read from a
// float table (indexed by a global selector) instead of plain globals, and
// with an extra step: a clamped blend of a second table entry is reported to
// one helper with a threshold flag, and the helper's answer is dereferenced
// into an optional output. The two integer arguments must fall strictly
// inside 200-wide windows around the even-adjusted anchors, else the result
// is +0.0. An optional output pair accumulates faded table differences, an
// optional single output receives one faded entry, and the return value is
// the faded entry at the combined index.
export!(cdecl, rw_00ad5560(a: i32, b: i32, pair: *mut u32, report: *mut u32, single: *mut u32) -> f32 {
    unsafe {
        const WINDOW: i32 = 200;
        const CELLS: i32 = 100;
        const HASH_BIAS: i32 = 100_000;
        const FADE_LIMIT: i32 = 10;

        // Truncating float-to-int exactly like cvttss2si, including the
        // indefinite value for NaN and out-of-range inputs (Rust's `as`
        // saturates instead, which differs there).
        fn cvt_trunc(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                0x80000000u32 as i32
            } else {
                x as i32
            }
        }

        let g = *global::<i32>(0x1174794);
        let slot = global::<f32>(0x158de00).add((g as u32).wrapping_mul(4) as usize);
        let base_a = cvt_trunc(*slot).wrapping_sub(100);
        let base_a = base_a.wrapping_sub(base_a >> 31) & !1;
        let base_b = cvt_trunc(*slot.add(1)).wrapping_sub(100);
        let base_b = base_b.wrapping_sub(base_b >> 31) & !1;

        let d1 = a.wrapping_sub(base_a);
        if d1 <= 0 {
            return 0.0;
        }
        let e1 = base_a.wrapping_sub(a).wrapping_add(WINDOW);
        if e1 <= 0 {
            return 0.0;
        }
        let d2 = b.wrapping_sub(base_b);
        if d2 <= 0 {
            return 0.0;
        }
        let e2 = base_b.wrapping_sub(b).wrapping_add(WINDOW);
        if e2 <= 0 {
            return 0.0;
        }

        let one = f32::from_bits(*global::<u32>(0xfe88e8));
        let step = f32::from_bits(*global::<u32>(0xfe879c));
        let edge = d1.min(e1).min(d2).min(e2);
        let mut scale = one;
        if edge < FADE_LIMIT {
            scale = ((edge + 1) as f32) * step;
        }

        let r1 = (a / 2).wrapping_add(HASH_BIAS) % CELLS;
        let r2 = (b / 2).wrapping_add(HASH_BIAS) % CELLS;
        let q1 = (base_a / 2).wrapping_add(HASH_BIAS) % CELLS;
        let q2 = (base_b / 2).wrapping_add(HASH_BIAS) % CELLS;

        let main_table = global::<f32>(0x15664f8);
        let aux_table = global::<f32>(0x1579d78);
        let grown = (g.wrapping_mul(CELLS).wrapping_add(r1)).wrapping_mul(CELLS);
        let combined = (grown.wrapping_add(r2)) as usize;
        let faded = *main_table.add(combined) * scale;

        if !pair.is_null() {
            let mut row_b = (r1 + 99) % CELLS;
            let mut row_s = (r1 + 101) % CELLS;
            let mut row_c = (r2 + 99) % CELLS;
            let tail = (r2 + 101) % CELLS;
            if r1 == q1 {
                row_b = r1;
            }
            if r2 == q2 {
                row_c = r2;
            }
            if r1 == q1 + 99 {
                row_s = r1;
            }
            if r2 == q2 + 99 {
                row_c = r2;
            }
            let first = *main_table.add(grown.wrapping_add(row_c) as usize)
                - *main_table.add(grown.wrapping_add(tail) as usize);
            let row_d = (g.wrapping_mul(CELLS).wrapping_add(row_b)).wrapping_mul(CELLS).wrapping_add(r2);
            let row_e = (g.wrapping_mul(CELLS).wrapping_add(row_s)).wrapping_mul(CELLS).wrapping_add(r2);
            let second = *main_table.add(row_d as usize) - *main_table.add(row_e as usize);
            let acc0 = f32::from_bits(*pair);
            let acc1 = f32::from_bits(*pair.add(1));
            *pair.add(1) = (first * scale + acc1).to_bits();
            *pair = (second * scale + acc0).to_bits();
        }

        if !report.is_null() {
            let probe = *aux_table.add(combined);
            let half = f32::from_bits(*global::<u32>(0x103f52c));
            let bias = f32::from_bits(*global::<u32>(0x103f530));
            let x = half * probe + bias;
            let clamped = if x < 0.0 {
                0.0f32
            } else if x > 1.0 {
                1.0f32
            } else {
                x
            };
            let tiny = f32::from_bits(*global::<u32>(0x103f524));
            let tiny2 = f32::from_bits(*global::<u32>(0x103f528));
            let flag = if tiny > probe.abs() && tiny2 > faded.abs() {
                0.0f32
            } else {
                1.0f32
            };
            let answer = callee_stdcall!(
                1,
                u32,
                (clamped * scale).to_bits(),
                0u32,
                flag.to_bits(),
                1.0f32.to_bits()
            );
            *report = *(answer as *const u32);
        }

        if !single.is_null() {
            *single = (*aux_table.add(combined) * scale).to_bits();
        }
        faded
    }
});
