// original: 0x00B020A0 append_scaled_sample (proposed)

/// Records one scaled integer triple into a table, guarded by zero counts.
///
/// The three probe floats `arg4`, `arg3` and `arg5` are read around the
/// register saves (the pushes interleave the loads), absolutized and
/// truncated toward zero (exact `cvttss2si`
/// semantics: NaN and out-of-range yield `0x80000000`). When more than one
/// of the three truncates to zero the function returns at once; when exactly
/// one does, a global sequence word is incremented. Otherwise the sample
/// `arg0`, `arg1`, `arg2` scaled by 4.0 is truncated and stored as three
/// 16-bit words into the table selected by the low byte of `arg10`: the big
/// table (4,000 rows of 20 bytes, guarded by its own fill counter, plus a
/// flag word per row) or the small table (40 rows of 20 bytes with its own
/// counter). If that table is already full the function returns without
/// writing. A sink helper (stdcall, four float words: the three truncated
/// probes as floats and `arg6` scaled by pi/180) is notified of every stored
/// row. On the big table the row flag word is then set or cleared from the
/// raw bits of `arg9` (read after the sink call, so one slot higher than the
/// pre-call layout suggests) and folded with a global key cell that also
/// remembers the row index. `arg7` and `arg8` are never read. Original
/// convention: cdecl, eleven stack words, caller cleans, no return value.
lf_checker_rt::export!(cdecl, rw_00B020A0(
    arg0: u32,
    arg1: u32,
    arg2: u32,
    arg3: u32,
    arg4: u32,
    arg5: u32,
    arg6: u32,
    _u2: u32,
    _u3: u32,
    arg9: u32,
    arg10: u32,
) -> u32 {
    unsafe {
        const ABS_MASK: u32 = 0x00FE8F80;
        const DEG_SCALE: u32 = 0x00FE8728;
        const ROW_SCALE: u32 = 0x00FE8AB8;
        const SEQ: u32 = 0x01614C88;
        const CTR1: u32 = 0x016010DC;
        const TAB1: u32 = 0x016010E0;
        const TAB1_MAX: u32 = 0xfa0;
        const CELL: u32 = 0x0104004C;
        const CTR2: u32 = 0x01614C80;
        const TAB2: u32 = 0x01614960;
        const TAB2_MAX: u32 = 0x28;
        const ROW_STRIDE: u32 = 20;
        const FLAG_WORD: u32 = 0x12;
        const TOP_BIT: u16 = 0x8000;
        const LOW15: u16 = 0x7fff;

        #[inline(always)]
        unsafe fn rd32(p: u32) -> u32 {
            unsafe { (p as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(p: u32) -> u16 {
            unsafe { (p as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(p: u32, v: u32) {
            unsafe { (p as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(p: u32, v: u16) {
            unsafe { (p as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        fn mul(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(y)
        }
        /// Exact `cvttss2si`: truncate toward zero, indefinite on NaN or
        /// out-of-range. (A plain `as` cast saturates instead.)
        #[inline(always)]
        fn cvtt(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                i32::MIN
            } else {
                x as i32
            }
        }

        let abs = rd32(lf_checker_rt::relocated(ABS_MASK));
        let ti = cvtt(f32::from_bits(arg4 & abs));
        let tj = cvtt(f32::from_bits(arg3 & abs));
        let tk = cvtt(f32::from_bits(arg5 & abs));
        let zeros = (ti == 0) as u32 + (tj == 0) as u32 + (tk == 0) as u32;
        if zeros > 1 {
            return 0;
        }
        if zeros == 1 {
            let s = rd32(lf_checker_rt::relocated(SEQ));
            wr32(lf_checker_rt::relocated(SEQ), s.wrapping_add(1));
        }
        let s1 = f32::from_bits(rd32(lf_checker_rt::relocated(DEG_SCALE)));
        let x2 = mul(f32::from_bits(arg6), s1);
        let s2 = f32::from_bits(rd32(lf_checker_rt::relocated(ROW_SCALE)));
        if (arg10 as u8) != 0 {
            let n = rd32(lf_checker_rt::relocated(CTR2));
            // Signed guard, as the original's `jge`: a negative count
            // proceeds and wraps the row address.
            if (n as i32) >= TAB2_MAX as i32 {
                return 0;
            }
            let row = lf_checker_rt::relocated(TAB2).wrapping_add(n.wrapping_mul(ROW_STRIDE));
            wr16(row, cvtt(mul(f32::from_bits(arg0), s2)) as u16);
            wr16(row.wrapping_add(2), cvtt(mul(f32::from_bits(arg1), s2)) as u16);
            wr16(row.wrapping_add(4), cvtt(mul(f32::from_bits(arg2), s2)) as u16);
            lf_checker_rt::callee_stdcall!(
                1,
                u32,
                (ti as f32).to_bits(),
                (tj as f32).to_bits(),
                (tk as f32).to_bits(),
                x2.to_bits()
            );
            wr32(lf_checker_rt::relocated(CTR2), n.wrapping_add(1));
        } else {
            let n = rd32(lf_checker_rt::relocated(CTR1));
            // Signed guard, as the original's `jge`.
            if (n as i32) >= TAB1_MAX as i32 {
                return 0;
            }
            let row = lf_checker_rt::relocated(TAB1).wrapping_add(n.wrapping_mul(ROW_STRIDE));
            wr16(row, cvtt(mul(f32::from_bits(arg0), s2)) as u16);
            wr16(row.wrapping_add(2), cvtt(mul(f32::from_bits(arg1), s2)) as u16);
            wr16(row.wrapping_add(4), cvtt(mul(f32::from_bits(arg2), s2)) as u16);
            lf_checker_rt::callee_stdcall!(
                1,
                u32,
                (ti as f32).to_bits(),
                (tj as f32).to_bits(),
                (tk as f32).to_bits(),
                x2.to_bits()
            );
            let fe = row.wrapping_add(FLAG_WORD);
            let mut w = rd16(fe);
            if arg9 != 0 {
                w |= TOP_BIT;
            } else {
                w &= LOW15;
            }
            let cell = rd32(lf_checker_rt::relocated(CELL));
            let mut ax = w ^ (cell as u16);
            wr32(lf_checker_rt::relocated(CELL), n);
            ax &= LOW15;
            wr16(fe, w ^ ax);
            wr32(lf_checker_rt::relocated(CTR1), n.wrapping_add(1));
        }
        0
    }
});
