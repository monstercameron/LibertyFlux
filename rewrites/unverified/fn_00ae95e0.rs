// original: 0x00ae95e0 frame_stage_transform (proposed)

/// Advance one frame of a staged coefficient transform and emit two scaled rows.
///
/// `token` ([ebp+8], opaque) is only forwarded to the emit callee. The object
/// pointer is read from a global (`G_OBJ`); `STATE` (`G_OBJ + 0xB0`) holds the
/// input coefficients and `G_OBJ + 0x38` a sequence word. A global frame word
/// (`G_WORD`) is incremented (wrapping to 1 through a reset callee when it
/// would pass `0xFFFF`), mirrored into the object, and four sign variants of
/// two state coefficients are offered to a fill callee. When the fill callee
/// answers zero the object mark bit 0 is cleared and the function returns;
/// otherwise the bit is set and the transform runs.
///
/// The transform (all single-precision, in the original's operand order):
/// scale the offered coefficients by the state rate (`SI_RATE`), fold them
/// with twelve state constants through thirteen 4-wide multiply-add rows
/// (row `k` reads scratch words `k-2..k` and writes `k-2..k+1`, seed words
/// zero), normalise one row through a callee, then build two output rows in
/// scratch (`S_OUT`) by one of two recipes: a direct recipe from state
/// coefficients when the state flag byte (`SI_FLAG`) is clear and the mode
/// byte (`G_FLAG`) is clear, else a scaled recipe from the folded rows. Each
/// row is emitted through one emit call (8 stack words: buffer, count 4 or 5,
/// token, a code-address tag, three sign comparisons, zero) gated by bits in
/// the object flags word (`O_FLAGS`: first row needs any of `0x200087`,
/// second row needs `0x100`). Two globals are zeroed along the way and a done
/// byte is cleared at the end.
///
/// The original keeps a security cookie on its frame and checks it on both
/// exits; the rewrite issues the same check calls. Unwritten scratch reads as
/// zero on both sides (the contract's stack fill), matching the original's
/// reads of two slots the fill callee never stored to under interception.
/// Original: cdecl, one stack word, no meaningful return value.
lf_checker_rt::export!(cdecl, rw_00ae95e0(token: u32) -> u32 {
    unsafe {
        // Globals (file VAs).
        const G_COOKIE: u32 = 0x01057FB4;
        const G_OBJ: u32 = 0x012FB1B8;
        const G_WORD: u32 = 0x011A8908;
        const G_FLAG: u32 = 0x01593BB4;
        const G_ZERO_B: u32 = 0x01593BB6;
        const G_ZERO_W: u32 = 0x01593BC0;
        const G_DONE: u32 = 0x015B2B90;
        // Image float constants (file VAs, read-only).
        const C_LIMIT: u32 = 0x00FE8C10; // 300.0
        const C_ONE: u32 = 0x00FE88E8; // 1.0
        const C_ALT: u32 = 0x00FE8B64; // 45.0
        const C_BLEND: u32 = 0x00FE87E4; // 0.25
        const C_FOLD: u32 = 0x00FE87D0; // 0.2
        const C_SCALE1: u32 = 0x00FE8734;
        const C_BIAS1: u32 = 0x00FE8B80;
        const C_SCALE2: u32 = 0x00FE86EC;
        const C_BIAS2: u32 = 0x00FE8B20;
        const C_ABS: u32 = 0x00FE8F80; // abs mask
        const C_SIGN: u32 = 0x00FE8FA0; // sign mask
        // Object layout.
        const O_MARK: u32 = 0x19;
        const O_SEQ: u32 = 0x38;
        const O_FLAGS: u32 = 0x8E8;
        const O_STATE: u32 = 0xB0;
        // State coefficient offsets (relative to STATE = obj + O_STATE).
        const SI_K0: u32 = 0x40;
        const SI_K1: u32 = 0x44;
        const SI_K2: u32 = 0x48;
        const SI_SX: u32 = 0x50;
        const SI_SY: u32 = 0x54;
        const SI_K3: u32 = 0x58;
        const SI_N0: u32 = 0x60;
        const SI_N1: u32 = 0x64;
        const SI_N2: u32 = 0x68;
        const SI_T0: u32 = 0x70;
        const SI_T1: u32 = 0x74;
        const SI_T2: u32 = 0x78;
        const SI_SPAN: u32 = 0x2B8;
        const SI_RATE: u32 = 0x2C4;
        const SI_A: u32 = 0x2C8;
        const SI_B: u32 = 0x2CC;
        const SI_D0: u32 = 0x2E0;
        const SI_D1: u32 = 0x2E4;
        const SI_D2: u32 = 0x2E8;
        const SI_D3: u32 = 0x2EC;
        const SI_FLAG: u32 = 0x2F0;
        // Scratch slots (byte offsets into `st`, matching the original frame).
        const S_0C: usize = 0x0C;
        const S_10: usize = 0x10;
        const S_2C: usize = 0x2C;
        const S_30: usize = 0x30;
        const S_4C: usize = 0x4C;
        const S_50: usize = 0x50;
        const S_6C: usize = 0x6C;
        const S_P1: usize = 0x70;
        const S_7C: usize = 0x7C;
        const S_80: usize = 0x80;
        const S_90: usize = 0x90;
        const S_9C: usize = 0x9C;
        const S_A0: usize = 0xA0;
        const S_AC: usize = 0xAC;
        const S_B0: usize = 0xB0;
        const S_C0: usize = 0xC0;
        const S_D8: usize = 0xD8;
        const S_E0: usize = 0xE0;
        const S_E8: usize = 0xE8;
        const S_F0: usize = 0xF0;
        const S_BUF: usize = 0xF4;
        const S_OUT: usize = 0x100;
        const S_ROW: usize = 0x150;
        const S_COPY: usize = 0x228;
        // Callee ids (see contract).
        const C_FILL: u32 = 1;
        const C_NORM: u32 = 2;
        const C_EMIT: u32 = 3;
        const C_COOKIE: u32 = 4;
        const C_RESET: u32 = 5;
        // Code-address tags forwarded to the emit callee (raw immediates).
        const TAG_FIRST: u32 = 0x00AE95B0;
        const TAG_SECOND: u32 = 0x00AE93C0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn img(va: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(va))) }
        }
        // Float arithmetic in the original's operand order; the barriers keep
        // the compiler from commuting or reassociating operands.
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
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
        fn neg_bits(v: f32, sign: u32) -> f32 {
            f32::from_bits(v.to_bits() ^ sign)
        }
        #[inline(always)]
        fn abs_bits(v: f32, mask: u32) -> f32 {
            f32::from_bits(v.to_bits() & mask)
        }

        let sign = rd32(lf_checker_rt::relocated(C_SIGN));
        let absm = rd32(lf_checker_rt::relocated(C_ABS));
        // Scratch frame; zero matches the contract's stack fill for the slots
        // the original reads without writing first.
        let mut st = [0u32; 160];
        let fr = |st: &[u32; 160], byte: usize| -> f32 {
            f32::from_bits(st[byte / 4])
        };
        // Entry: object, frame word, sign variants of two coefficients.
        let cookie = lf_checker_rt::global::<u32>(G_COOKIE);
        let _cookie_val: u32 = cookie.read_unaligned();
        let mut obj = rd32(lf_checker_rt::relocated(G_OBJ));
        let state = obj.wrapping_add(O_STATE);
        let mut seq = rd16(lf_checker_rt::relocated(G_WORD));
        if seq < 0xFFFF {
            seq = seq.wrapping_add(1);
        } else {
            let _: u32 = lf_checker_rt::callee_cdecl!(C_RESET, u32,);
            obj = rd32(lf_checker_rt::relocated(G_OBJ));
            seq = 1;
        }
        let edi = token;
        wr16(lf_checker_rt::relocated(G_WORD), seq);
        wr16(obj.wrapping_add(O_SEQ), seq);
        let sb = f32::from_bits(rd32(state.wrapping_add(SI_B)));
        let sa = f32::from_bits(rd32(state.wrapping_add(SI_A)));
        st[S_BUF / 4] = neg_bits(sb, sign).to_bits();
        st[S_BUF / 4 + 1] = sb.to_bits();
        st[S_BUF / 4 + 2] = sa.to_bits();
        st[(S_BUF + 0x14) / 4] = neg_bits(sa, sign).to_bits();
        let buf_ptr = core::ptr::addr_of_mut!(st[S_BUF / 4]) as u32;
        let fill_ans: u32 =
            lf_checker_rt::callee_cdecl!(C_FILL, u32, buf_ptr, edi, obj);
        obj = rd32(lf_checker_rt::relocated(G_OBJ));
        if (fill_ans as u8) == 0 {
            wr8(obj.wrapping_add(O_MARK), rd8(obj.wrapping_add(O_MARK)) & 0xFE);
            let _: u32 = lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
            return 0;
        }
        wr8(obj.wrapping_add(O_MARK), rd8(obj.wrapping_add(O_MARK)) | 1);
        // Scale the offered coefficients by the state rate and seed rows.
        let mut x1 = f32::from_bits(rd32(state.wrapping_add(SI_RATE)));
        let mut x0 = fr(&st, S_7C);
        st[0x15C / 4] = x0.to_bits();
        st[0x16C / 4] = x0.to_bits();
        st[0x17C / 4] = x0.to_bits();
        st[0x18C / 4] = x0.to_bits();
        st[S_ROW / 4] = 0;
        st[S_ROW / 4 + 1] = 0;
        st[S_ROW / 4 + 2] = 0;
        st[0x19C / 4] = x0.to_bits();
        let mut x5 = fr(&st, S_F0);
        let mut x4 = fr(&st, S_BUF + 4);
        let mut x2 = fr(&st, S_BUF);
        let mut x6 = fr(&st, S_BUF + 8);
        x5 = mul(x5, x1);
        x4 = mul(x4, x1);
        st[0x160 / 4] = x5.to_bits();
        st[0x190 / 4] = x5.to_bits();
        st[0x164 / 4] = x4.to_bits();
        st[0x174 / 4] = x4.to_bits();
        x2 = mul(x2, x1);
        x6 = mul(x6, x1);
        st[0x170 / 4] = x2.to_bits();
        st[0x180 / 4] = x2.to_bits();
        st[0x184 / 4] = x6.to_bits();
        st[0x194 / 4] = x6.to_bits();
        let mut x3 = neg_bits(x1, sign);
        st[0x168 / 4] = x3.to_bits();
        st[0x178 / 4] = x3.to_bits();
        st[0x188 / 4] = x3.to_bits();
        st[0x198 / 4] = x3.to_bits();
        st[S_B0 / 4] = rd32(state.wrapping_add(SI_K0));
        st[S_9C / 4] = rd32(state.wrapping_add(SI_K1));
        st[S_E8 / 4] = rd32(state.wrapping_add(SI_K2));
        st[S_50 / 4] = rd32(state.wrapping_add(SI_SX));
        st[S_4C / 4] = rd32(state.wrapping_add(SI_SY));
        st[S_D8 / 4] = rd32(state.wrapping_add(SI_K3));
        st[S_C0 / 4] = rd32(state.wrapping_add(SI_N0));
        st[S_AC / 4] = rd32(state.wrapping_add(SI_N1));
        st[S_E0 / 4] = rd32(state.wrapping_add(SI_N2));
        st[S_10 / 4] = rd32(state.wrapping_add(SI_T0));
        st[S_0C / 4] = rd32(state.wrapping_add(SI_T1));
        x0 = f32::from_bits(rd32(state.wrapping_add(SI_T2)));
        wr32(lf_checker_rt::relocated(G_ZERO_W), 0);
        let flag_set = rd8(state.wrapping_add(SI_FLAG)) != 0;
        st[S_30 / 4] = x5.to_bits();
        st[S_6C / 4] = x4.to_bits();
        st[S_90 / 4] = x3.to_bits();
        st[S_80 / 4] = x2.to_bits();
        st[S_2C / 4] = x6.to_bits();
        st[S_A0 / 4] = x0.to_bits();
        let mut x7 = 0.0f32;
        if flag_set {
            // Rate-limited fold of the seeded rows.
            x0 = f32::from_bits(rd32(state.wrapping_add(SI_RATE)));
            x1 = img(C_LIMIT);
            x7 = img(C_ONE);
            if x0 > x1 {
                x2 = f32::from_bits(rd32(state.wrapping_add(SI_SPAN)));
                x7 = img(C_ALT);
                if x7 > x2 {
                    x7 = sub(x7, x2);
                    x7 = mul(x7, img(C_BLEND));
                    x7 = add(x7, x1);
                } else {
                    x7 = x1;
                }
                x7 = div(x7, x0);
            }
            x0 = fr(&st, S_6C);
            x0 = mul(x0, x7);
            x2 = fr(&st, S_7C);
            x6 = x4;
            x4 = x3;
            x3 = fr(&st, S_80);
            st[S_6C / 4] = x0.to_bits();
            st[0x1B4 / 4] = x0.to_bits();
            x0 = fr(&st, S_7C);
            x4 = mul(x4, x7);
            x1 = fr(&st, S_90);
            st[0x1BC / 4] = x0.to_bits();
            st[0x1DC / 4] = x0.to_bits();
            x0 = fr(&st, S_6C);
            st[0x1A8 / 4] = x4.to_bits();
            st[0x1C8 / 4] = x4.to_bits();
            x4 = img(C_FOLD);
            x0 = mul(x0, x4);
            x5 = mul(x5, x7);
            st[0x1D4 / 4] = x0.to_bits();
            x0 = fr(&st, S_30);
            x6 = mul(x6, x7);
            x3 = mul(x3, x7);
            x0 = mul(x0, x7);
            st[0x1A0 / 4] = x5.to_bits();
            st[0x1A4 / 4] = x6.to_bits();
            st[S_30 / 4] = x0.to_bits();
            x0 = fr(&st, S_2C);
            st[0x1B0 / 4] = x3.to_bits();
            x0 = mul(x0, x7);
            x5 = mul(x5, x4);
            x3 = mul(x3, x4);
            x6 = mul(x6, x4);
            st[0x1C0 / 4] = x5.to_bits();
            x5 = fr(&st, S_90);
            st[0x1D0 / 4] = x3.to_bits();
            x3 = fr(&st, S_2C);
            st[0x1C4 / 4] = x6.to_bits();
            x6 = fr(&st, S_80);
            x1 = mul(x1, x7);
            x6 = mul(x6, x7);
            x3 = mul(x3, x7);
            st[0x1AC / 4] = x2.to_bits();
            st[0x1CC / 4] = x2.to_bits();
            x2 = x5;
            x2 = mul(x2, x7);
            x5 = mul(x5, x7);
            x7 = fr(&st, S_30);
            st[S_2C / 4] = x0.to_bits();
            st[0x1F4 / 4] = x0.to_bits();
            x0 = fr(&st, S_7C);
            st[0x1FC / 4] = x0.to_bits();
            st[0x21C / 4] = x0.to_bits();
            x0 = fr(&st, S_2C);
            st[0x1F0 / 4] = x7.to_bits();
            x7 = mul(x7, x4);
            st[0x1E0 / 4] = x6.to_bits();
            st[0x1E4 / 4] = x3.to_bits();
            x6 = mul(x6, x4);
            x3 = mul(x3, x4);
            x0 = mul(x0, x4);
            st[0x1B8 / 4] = x1.to_bits();
            st[0x1D8 / 4] = x1.to_bits();
            x1 = fr(&st, S_7C);
            st[0x210 / 4] = x7.to_bits();
            x7 = fr(&st, S_50);
            st[0x1E8 / 4] = x2.to_bits();
            st[0x1EC / 4] = x1.to_bits();
            st[0x1F8 / 4] = x5.to_bits();
            st[0x208 / 4] = x2.to_bits();
            st[0x20C / 4] = x1.to_bits();
            st[0x218 / 4] = x5.to_bits();
            st[0x200 / 4] = x6.to_bits();
            st[0x204 / 4] = x3.to_bits();
            st[0x214 / 4] = x0.to_bits();
            // Thirteen 4-wide multiply-add rows over the folded words.
            let mut base = S_ROW + 8;
            let mut left = 13u32;
            loop {
                x3 = fr(&st, base - 4);
                x5 = fr(&st, base - 8);
                x6 = fr(&st, S_B0);
                x4 = fr(&st, base);
                x2 = fr(&st, S_9C);
                x0 = x3;
                x0 = mul(x0, x7);
                x6 = mul(x6, x5);
                x1 = fr(&st, S_D8);
                x2 = mul(x2, x5);
                x6 = add(x6, x0);
                x0 = fr(&st, S_C0);
                x0 = mul(x0, x4);
                x1 = mul(x1, x3);
                x6 = add(x6, x0);
                x0 = x3;
                x0 = mul(x0, fr(&st, S_4C));
                base += 0x10;
                x6 = add(x6, fr(&st, S_10));
                x2 = add(x2, x0);
                x0 = fr(&st, S_AC);
                x0 = mul(x0, x4);
                st[(base - 0x18) / 4] = x6.to_bits();
                x2 = add(x2, x0);
                x0 = fr(&st, S_E8);
                x0 = mul(x0, x5);
                x2 = add(x2, fr(&st, S_0C));
                x1 = add(x1, x0);
                x0 = fr(&st, S_E0);
                x0 = mul(x0, x4);
                st[(base - 0x14) / 4] = x2.to_bits();
                x1 = add(x1, x0);
                x0 = fr(&st, S_7C);
                st[(base - 0x0C) / 4] = x0.to_bits();
                x1 = add(x1, fr(&st, S_A0));
                st[(base - 0x10) / 4] = x1.to_bits();
                left -= 1;
                if left == 0 {
                    break;
                }
            }
        }
        // Normalise one row, then build the first output row.
        x0 = f32::from_bits(rd32(state.wrapping_add(SI_N0)));
        st[S_P1 / 4] = x0.to_bits();
        obj = rd32(lf_checker_rt::relocated(G_OBJ));
        x0 = f32::from_bits(rd32(state.wrapping_add(SI_N1)));
        let p1_ptr = core::ptr::addr_of_mut!(st[S_P1 / 4]) as u32;
        st[S_P1 / 4 + 1] = x0.to_bits();
        x0 = f32::from_bits(rd32(state.wrapping_add(SI_N2)));
        let p2_ptr = core::ptr::addr_of_mut!(st[S_ROW / 4]) as u32;
        let this_norm = obj.wrapping_add(0x50);
        st[S_P1 / 4 + 2] = x0.to_bits();
        let _: u32 = lf_checker_rt::callee_thiscall!(C_NORM, u32, this_norm, p2_ptr, p1_ptr);
        obj = rd32(lf_checker_rt::relocated(G_OBJ));
        wr8(obj.wrapping_add(O_MARK), rd8(obj.wrapping_add(O_MARK)) | 8);
        x7 = fr(&st, S_ROW + 4);
        x3 = fr(&st, S_ROW);
        let mut mode = rd8(lf_checker_rt::relocated(G_FLAG));
        wr8(lf_checker_rt::relocated(G_ZERO_B), 0);
        let flag2_set = rd8(state.wrapping_add(SI_FLAG)) != 0;
        let count1: u32;
        if !flag2_set && mode == 0 {
            let mut x4 = f32::from_bits(rd32(state.wrapping_add(SI_D2)));
            x1 = f32::from_bits(rd32(state.wrapping_add(SI_D1)));
            x6 = f32::from_bits(rd32(state.wrapping_add(SI_D3)));
            x0 = fr(&st, S_0C);
            x5 = fr(&st, S_10);
            x5 = add(x5, f32::from_bits(rd32(state.wrapping_add(SI_D0))));
            x3 = img(C_SCALE1);
            x1 = add(x1, fr(&st, S_10));
            x6 = add(x6, fr(&st, S_0C));
            x2 = img(C_BIAS1);
            x0 = add(x0, x4);
            x4 = add(x4, fr(&st, S_10));
            x5 = mul(x5, x3);
            x1 = mul(x1, x3);
            x0 = mul(x0, x3);
            x4 = mul(x4, x3);
            x6 = mul(x6, x3);
            x3 = fr(&st, S_ROW);
            x5 = add(x5, x2);
            x0 = add(x0, x2);
            x6 = add(x6, x2);
            x1 = add(x1, x2);
            x4 = add(x4, x2);
            st[S_OUT / 4] = x5.to_bits();
            st[S_OUT / 4 + 1] = x0.to_bits();
            st[S_OUT / 4 + 3] = x0.to_bits();
            st[S_OUT / 4 + 2] = x1.to_bits();
            st[S_OUT / 4 + 4] = x4.to_bits();
            st[S_OUT / 4 + 5] = x6.to_bits();
            st[S_OUT / 4 + 6] = x5.to_bits();
            st[S_OUT / 4 + 7] = x6.to_bits();
            count1 = 4;
        } else {
            x2 = img(C_SCALE1);
            x1 = img(C_BIAS1);
            x0 = x3;
            x0 = mul(x0, x2);
            count1 = 5;
            x0 = add(x0, x1);
            st[S_OUT / 4] = x0.to_bits();
            x0 = x7;
            x0 = mul(x0, x2);
            x0 = add(x0, x1);
            st[S_OUT / 4 + 1] = x0.to_bits();
            x0 = fr(&st, 0x1A0);
            x0 = mul(x0, x2);
            x0 = add(x0, x1);
            st[S_OUT / 4 + 2] = x0.to_bits();
            x0 = fr(&st, 0x1A4);
            x0 = mul(x0, x2);
            x0 = add(x0, x1);
            st[S_OUT / 4 + 3] = x0.to_bits();
            x0 = fr(&st, 0x1B0);
            x0 = mul(x0, x2);
            x0 = add(x0, x1);
            st[S_OUT / 4 + 4] = x0.to_bits();
            x0 = fr(&st, 0x1B4);
            x0 = mul(x0, x2);
            x0 = add(x0, x1);
            st[S_OUT / 4 + 5] = x0.to_bits();
            x0 = fr(&st, 0x1E0);
            x0 = mul(x0, x2);
            x0 = add(x0, x1);
            st[S_OUT / 4 + 6] = x0.to_bits();
            x0 = fr(&st, 0x1E4);
            x0 = mul(x0, x2);
            x0 = add(x0, x1);
            st[S_OUT / 4 + 7] = x0.to_bits();
            x0 = fr(&st, 0x1F0);
            x0 = mul(x0, x2);
            x0 = add(x0, x1);
            st[S_OUT / 4 + 8] = x0.to_bits();
            x0 = fr(&st, 0x1F4);
            x0 = mul(x0, x2);
            x0 = add(x0, x1);
            st[S_OUT / 4 + 9] = x0.to_bits();
        }
        obj = rd32(lf_checker_rt::relocated(G_OBJ));
        x4 = f32::from_bits(absm);
        if rd32(obj.wrapping_add(O_FLAGS)) & 0x200087 != 0 {
            if (count1 as i32) > 0 {
                let n = ((count1 & 0x1FFF_FFFF) * 2) as usize;
                let mut k = 0usize;
                while k < n {
                    st[S_COPY / 4 + k] = st[S_OUT / 4 + k];
                    k += 1;
                }
            }
            x2 = fr(&st, S_50);
            x3 = fr(&st, S_4C);
            x0 = x3;
            x1 = x2;
            x0 = abs_bits(x0, x4.to_bits());
            x1 = abs_bits(x1, x4.to_bits());
            let above = (x1 > x0) as u32;
            x0 = 0.0;
            let ge3 = (x3 >= x0) as u32;
            let ge2 = (x2 >= x0) as u32;
            let dbuf = core::ptr::addr_of_mut!(st[S_COPY / 4]) as u32;
            let _: u32 = lf_checker_rt::callee_cdecl!(
                C_EMIT, u32, dbuf, count1, edi, TAG_FIRST, ge2, ge3, above, 0
            );
            x7 = fr(&st, 0x174);
            x3 = fr(&st, 0x170);
            mode = rd8(lf_checker_rt::relocated(G_FLAG));
            obj = rd32(lf_checker_rt::relocated(G_OBJ));
        }
        if rd32(obj.wrapping_add(O_FLAGS)) & 0x100 != 0 {
            let flag3_set = rd8(state.wrapping_add(SI_FLAG)) != 0;
            let count2: u32;
            if !flag3_set && mode == 0 {
                x4 = f32::from_bits(rd32(state.wrapping_add(SI_D2)));
                x6 = f32::from_bits(rd32(state.wrapping_add(SI_D0)));
                x1 = f32::from_bits(rd32(state.wrapping_add(SI_D1)));
                x5 = f32::from_bits(rd32(state.wrapping_add(SI_D3)));
                x7 = fr(&st, S_10);
                x5 = add(x5, fr(&st, S_0C));
                x3 = img(C_SCALE2);
                x2 = img(C_BIAS2);
                x0 = x4;
                x0 = add(x0, fr(&st, S_0C));
                x6 = add(x6, x7);
                x1 = add(x1, x7);
                x4 = add(x4, x7);
                x5 = mul(x5, x3);
                x6 = mul(x6, x3);
                x0 = mul(x0, x3);
                x1 = mul(x1, x3);
                x4 = mul(x4, x3);
                x6 = add(x6, x2);
                x0 = add(x0, x2);
                x5 = add(x5, x2);
                x1 = add(x1, x2);
                x4 = add(x4, x2);
                st[S_OUT / 4] = x6.to_bits();
                st[S_OUT / 4 + 1] = x0.to_bits();
                st[S_OUT / 4 + 3] = x0.to_bits();
                st[S_OUT / 4 + 2] = x1.to_bits();
                st[S_OUT / 4 + 4] = x4.to_bits();
                st[S_OUT / 4 + 5] = x5.to_bits();
                st[S_OUT / 4 + 6] = x6.to_bits();
                st[S_OUT / 4 + 7] = x5.to_bits();
                count2 = 4;
            } else {
                x2 = img(C_SCALE2);
                x1 = img(C_BIAS2);
                x0 = fr(&st, 0x160);
                x0 = mul(x0, x2);
                x3 = mul(x3, x2);
                x0 = add(x0, x1);
                x7 = mul(x7, x2);
                x3 = add(x3, x1);
                count2 = 5;
                st[S_OUT / 4 + 2] = x0.to_bits();
                x0 = fr(&st, 0x164);
                x0 = mul(x0, x2);
                x7 = add(x7, x1);
                st[S_OUT / 4] = x3.to_bits();
                x0 = add(x0, x1);
                st[S_OUT / 4 + 1] = x7.to_bits();
                st[S_OUT / 4 + 3] = x0.to_bits();
                x0 = fr(&st, 0x170);
                x0 = mul(x0, x2);
                x0 = add(x0, x1);
                st[S_OUT / 4 + 4] = x0.to_bits();
                x0 = fr(&st, 0x174);
                x0 = mul(x0, x2);
                x0 = add(x0, x1);
                st[S_OUT / 4 + 5] = x0.to_bits();
                x0 = fr(&st, 0x180);
                x0 = mul(x0, x2);
                x0 = add(x0, x1);
                st[S_OUT / 4 + 6] = x0.to_bits();
                x0 = fr(&st, 0x184);
                x0 = mul(x0, x2);
                x0 = add(x0, x1);
                st[S_OUT / 4 + 7] = x0.to_bits();
                x0 = fr(&st, 0x190);
                x0 = mul(x0, x2);
                x0 = add(x0, x1);
                st[S_OUT / 4 + 8] = x0.to_bits();
                x0 = fr(&st, 0x194);
                x0 = mul(x0, x2);
                x0 = add(x0, x1);
                st[S_OUT / 4 + 9] = x0.to_bits();
            }
            x2 = fr(&st, S_50);
            x3 = fr(&st, S_4C);
            x0 = x3;
            x0 = abs_bits(x0, absm);
            x1 = x2;
            x1 = abs_bits(x1, absm);
            let above2 = (x1 > x0) as u32;
            x0 = 0.0;
            let ge3b = (x3 >= x0) as u32;
            let ge2b = (x2 >= x0) as u32;
            let dbuf2 = core::ptr::addr_of_mut!(st[S_OUT / 4]) as u32;
            let _: u32 = lf_checker_rt::callee_cdecl!(
                C_EMIT, u32, dbuf2, count2, edi, TAG_SECOND, ge2b, ge3b, above2, 0
            );
        }
        wr8(lf_checker_rt::relocated(G_DONE), 0);
        let _: u32 = lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
        0
    }
});
