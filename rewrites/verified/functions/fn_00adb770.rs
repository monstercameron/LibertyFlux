// original: 0x00adb770 input_ui_tls_gate_and_view_setup (proposed)

/// Select a view slot through a thread-local gate, then either publish a
/// block of view floats or swap two double-buffered regions.
///
/// No arguments (cdecl, plain `ret`); the return value is whatever the last
/// executed step left in eax: the view-query answer on the main path, the
/// validity-probe answer on the early path, or the last copied word on the
/// swap path.
///
/// Gate: a thread-local word (slot index from `TLS_INDEX`, word at `+0x8d0`
/// of the slot object) picks one of two selector globals: `SEL_A` when bit 1
/// or bit 3 of the word is set, else `SEL_B`. When `FLOAT_GATE` is zero the
/// float at `FLOAT_SRC` is published to `FLOAT_ARR[sel]`. A handler pointer
/// kept in a data slot is then called with `HANDLER_ARG` (stdcall, one
/// argument); its zero/nonzero answer is or-ed with the flag bytes
/// `FLAG_A`, `FLAG_B`, `FLAG_C` and `FLAG_D`. Any nonzero bit takes the swap
/// path below.
///
/// Next the validity probe (cdecl, no arguments) runs; a nonzero low byte
/// also takes the swap path. When `EARLY_EXIT` equals that low byte the
/// function returns the probe answer at once.
///
/// Main path: two filler callees each take a pointer to one four-word stack
/// buffer (the second overwrites the first's words), a frame callee takes a
/// pointer to a 0x94-byte stack area in ecx, and that area is then zeroed
/// (native memset on the original side). The four buffer words are published
/// to `VIEW[0..4]` and to `ROW[sel_a * 16 ..]`, while `VIEW[4..13]` receive
/// two computed floats, two copied words, two computed pointers and two more
/// copied floats:
///
/// ```text
/// x0 = ((BASE * K0) + K1) * K2      VIEW[7]
/// x1 = ((BASE * K3) + K4) * K5      VIEW[11]
/// ```
///
/// with `BASE` from `SCALE_SRC`, constants `K0..K5` from data globals, and
/// the multiplies/adds in the original's operand order. Finally the view
/// query (cdecl, four arguments: two code pointers passed as immediate
/// values, the zeroed-area pointer, zero) runs; its answer is stored to
/// `QUERY_OUT` and returned. All address immediates carry relocations, so they denote relocated addresses.
///
/// Swap path: `SEL_A` (0 or 1) scales by `REGION` (0x9c40) to pick a side of
/// two double-buffered region pairs, each side copied onto the other with a
/// 0x9c40-byte copy (native memcpy on the original side), then one 16-byte
/// row is copied from `ROW_BASE + 0x10 - sel*16` to `ROW_BASE + sel*16` and
/// its last word is returned.
///
/// Edge cases: a nonzero gate byte anywhere forces the swap path; the early
/// path returns the probe answer with its upper bytes intact; all float
/// stores are bit copies except `x0`/`x1`, whose NaN payloads follow the
/// original's SSE operand order.
lf_checker_rt::export!(cdecl, rw_00adb770() -> u32 {
    unsafe {
        const TLS_INDEX: u32 = 0x17aba14;
        const TLS_WORD_OFF: u32 = 0x8d0;
        const SEL_A: u32 = 0x1174790;
        const SEL_B: u32 = 0x1174794;
        const FLOAT_GATE: u32 = 0x154e2b7;
        const FLOAT_ARR: u32 = 0x154ec50;
        const FLOAT_SRC: u32 = 0x154ec58;
        const QUERY_OUT: u32 = 0x154ec5c;
        const HANDLER_ARG: u32 = 0x17accd8;
        const FLAG_A: u32 = 0x105b48f;
        const FLAG_B: u32 = 0x17ed8d1;
        const FLAG_C: u32 = 0x1173590;
        const FLAG_D: u32 = 0x1173591;
        const EARLY_EXIT: u32 = 0x103f496;
        const VIEW: u32 = 0x158e620;
        const ROW_BASE: u32 = 0x158de00;
        const ROW_SRC_BASE: u32 = 0x158de10;
        const REGION: u32 = 0x9c40;
        const REG_A0: u32 = 0x15664f8;
        const REG_A1: u32 = 0x1570138;
        const REG_B0: u32 = 0x1579d78;
        const REG_B1: u32 = 0x15839b8;
        const COPY_WORDS: u32 = 0x9c40 / 4;
        const SCALE_SRC: u32 = 0x12ddeb4;
        const K0: u32 = 0xfe8874;
        const K1: u32 = 0xfe87dc;
        const K2: u32 = 0x103f4d8;
        const K3: u32 = 0xfe8844;
        const K4: u32 = 0xea6d3c;
        const K5: u32 = 0x103f4e4;
        const VIEW4_WORD: u32 = 0x11735bc;
        const VIEW6_WORD: u32 = 0x103f4dc;
        const VIEW_INT0: u32 = 0x1173608;
        const VIEW_INT1: u32 = 0x103f4e8;
        const VIEW10_WORD: u32 = 0x103f4e0;
        const QUERY_CB0: u32 = 0xea69b4;
        const QUERY_CB1: u32 = 0xadc240;
        const CAL_SLOT: u32 = 0;
        const CAL_PROBE: u32 = 1;
        const CAL_FILL0: u32 = 2;
        const CAL_FILL1: u32 = 3;
        const CAL_FRAME: u32 = 4;
        const CAL_QUERY: u32 = 6;
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
        unsafe fn g32(file_va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(file_va)) }
        }
        #[inline(always)]
        unsafe fn g8(file_va: u32) -> u8 {
            unsafe { rd8(lf_checker_rt::relocated(file_va)) }
        }
        #[inline(always)]
        unsafe fn set_g32(file_va: u32, v: u32) {
            unsafe { wr32(lf_checker_rt::relocated(file_va), v) }
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
        unsafe fn copy_words(dst: u32, src: u32, n: u32) {
            unsafe {
                let mut d = dst;
                let mut s = src;
                let mut i = 0u32;
                while i < n {
                    wr32(d, rd32(s));
                    d = d.wrapping_add(4);
                    s = s.wrapping_add(4);
                    i += 1;
                }
            }
        }

        // Gate: pick the selector through the thread-local word.
        let slot = g32(TLS_INDEX);
        let w = rd32(lf_checker_rt::tls_slot(slot as usize).wrapping_add(TLS_WORD_OFF));
        let sel = if (w >> 1) & 1 != 0 {
            g32(SEL_A)
        } else if (w >> 3) & 1 != 0 {
            g32(SEL_A)
        } else {
            g32(SEL_B)
        };
        if g8(FLOAT_GATE) == 0 {
            set_g32(FLOAT_ARR.wrapping_add(sel.wrapping_mul(4)), g32(FLOAT_SRC));
        }
        let r0: u32 = lf_checker_rt::callee_stdcall!(CAL_SLOT, u32, g32(HANDLER_ARG));
        let mut b: u8 = if r0 != 0 {
            1
        } else if g8(FLAG_A) == 0 {
            0
        } else {
            (g8(FLAG_B) != 0) as u8
        };
        b |= g8(FLAG_C);
        b |= g8(FLAG_D);
        if b != 0 {
            return swap_path();
        }
        let r1: u32 = lf_checker_rt::callee_cdecl!(CAL_PROBE, u32,);
        if (r1 & 0xff) != 0 {
            return swap_path();
        }
        if g8(EARLY_EXIT) == (r1 & 0xff) as u8 {
            let _: u32 = lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
            return r1;
        }

        // Main path.
        let mut buf = [0u32; 4];
        let mut big = [0u32; 37];
        let buf_ptr = buf.as_mut_ptr() as u32;
        let big_ptr = big.as_mut_ptr() as u32;
        let _: u32 = lf_checker_rt::callee_cdecl!(CAL_FILL0, u32, buf_ptr);
        let _: u32 = lf_checker_rt::callee_cdecl!(CAL_FILL1, u32, buf_ptr);
        let _: u32 = lf_checker_rt::callee_thiscall!(CAL_FRAME, u32, big_ptr);
        for w in big.iter_mut() {
            *w = 0;
        }
        let edx = g32(SEL_A);
        set_g32(VIEW + 0x10, g32(VIEW4_WORD));
        set_g32(VIEW + 0x18, g32(VIEW6_WORD));
        let ecx = edx.wrapping_mul(REGION);
        let base = f32::from_bits(g32(SCALE_SRC));
        let x0 = add(
            mul(base, f32::from_bits(g32(K0))),
            f32::from_bits(g32(K1)),
        );
        let x1 = mul(base, f32::from_bits(g32(K3)));
        set_g32(VIEW + 0x14, g32(VIEW_INT0));
        let x1 = add(x1, f32::from_bits(g32(K4)));
        set_g32(VIEW + 0x28, g32(VIEW_INT1));
        let x0 = mul(x0, f32::from_bits(g32(K2)));
        set_g32(VIEW + 0x2c, ecx.wrapping_add(lf_checker_rt::relocated(REG_A0)));
        let x1 = mul(x1, f32::from_bits(g32(K5)));
        set_g32(VIEW + 0x30, ecx.wrapping_add(lf_checker_rt::relocated(REG_B0)));
        set_g32(VIEW + 0x1c, x0.to_bits());
        set_g32(VIEW + 0x20, g32(VIEW10_WORD));
        set_g32(VIEW, buf[0]);
        set_g32(VIEW + 4, buf[1]);
        set_g32(VIEW + 8, buf[2]);
        set_g32(VIEW + 12, buf[3]);
        set_g32(VIEW + 0x24, x1.to_bits());
        let row = lf_checker_rt::relocated(ROW_BASE).wrapping_add(edx.wrapping_mul(16));
        wr32(row, buf[0]);
        wr32(row.wrapping_add(4), buf[1]);
        wr32(row.wrapping_add(8), buf[2]);
        wr32(row.wrapping_add(12), buf[3]);
        big[0] = 0x40;
        big[1] = lf_checker_rt::relocated(VIEW);
        let r6: u32 =
            lf_checker_rt::callee_cdecl!(
                CAL_QUERY,
                u32,
                lf_checker_rt::relocated(QUERY_CB0),
                lf_checker_rt::relocated(QUERY_CB1),
                big_ptr,
                0
            );
        set_g32(QUERY_OUT, r6);
        let _: u32 = lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
        return r6;

        // Swap path: copy each double-buffered side onto the other, then one row.
        unsafe fn swap_path() -> u32 {
            unsafe {
                let edi = g32(SEL_A);
                let esi = edi.wrapping_mul(REGION);
                copy_words(
                    lf_checker_rt::relocated(REG_A0).wrapping_add(esi),
                    lf_checker_rt::relocated(REG_A1).wrapping_sub(esi),
                    COPY_WORDS,
                );
                copy_words(
                    lf_checker_rt::relocated(REG_B0).wrapping_add(esi),
                    lf_checker_rt::relocated(REG_B1).wrapping_sub(esi),
                    COPY_WORDS,
                );
                let e16 = edi.wrapping_mul(16);
                let src = lf_checker_rt::relocated(ROW_SRC_BASE).wrapping_sub(e16);
                let dst = lf_checker_rt::relocated(ROW_BASE).wrapping_add(e16);
                let w0 = rd32(src);
                let w1 = rd32(src.wrapping_add(4));
                let w2 = rd32(src.wrapping_add(8));
                let w3 = rd32(src.wrapping_add(12));
                wr32(dst, w0);
                wr32(dst.wrapping_add(4), w1);
                wr32(dst.wrapping_add(8), w2);
                wr32(dst.wrapping_add(12), w3);
                let _: u32 = lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
                w3
            }
        }
    }
});
