// original: 0x008BC550 MO_VEH (symbols)

/// Vehicle-menu refresh (input-ui): advances a frame counter or a mode
/// counter, rebuilds a set of display rows from table-helper answers, logs
/// one line per row through the text helpers, and draws the rows.
///
/// Calling convention: cdecl, no arguments, no return value (the single
/// caller ignores EAX; the contract compares `none`). Reads no incoming
/// registers or stack arguments; every input is a global, a callee answer,
/// or one fabricated TLS slot (index from a global, pinned to 11).
///
/// Behaviour in order:
/// 1. Frame gate: if the frame counter is SIGNED-less than 1500, scales a
///    global delta by a read-only rate, truncates the product toward zero
///    (x87 `fistp` with chop rounding; NaN or out-of-range`|i64|` yields
///    low dword 0) and adds it to the counter. Otherwise increments the
///    mode counter with a SIGNED greater-than-5 wrap to 1 and clears the
///    frame counter.
/// 2. Resolves a colour word: 0xFF alpha unless a gate byte is set, in
///    which case the low byte of the gate helper's answer (id 1). Fetches
///    two float pairs through the table getter (id 2, indices 0x39/0x3A),
///    lets the formatter (id 3) overwrite their first words, blends them
///    (sums in the original's SSE order) and stages eight words; runs the
///    sync helper (id 5, no arguments) and the display helper (id 6).
/// 3. Resolves a palette word (id 7, index 0x3C, observing the saved
///    counter slot), publishes it (id 8) and the eight staged words plus
///    the colour (id 9, five frame pointers observed via snapshots).
/// 4. Publishes a mode word (2 unless a mode byte is 0x6A or a flag byte is
///    set, else 0), a 1.0 and the colour (ids 10, 11, 12); fetches three
///    more pairs (ids 0x3B/0x3C/0x3D), formats two (id 3) and one through
///    the null-site formatter (id 4, third argument NULL).
/// 5. Selects entry 0, positions at (0, 1.0) (ids 13, 14); fetches pairs
///    0x74/0x75, formats them, publishes the pair (id 15); dispatches on
///    the mode selector (0, 1, else) to pick a text template (id 16),
///    draws (id 17), resolves a second palette word (ids 7, 8), publishes
///    the mode word again, a second pair (id 15), entry 2 (id 13) and a
///    position (id 14).
/// 6. Scans 24 rows (index 0..23): at row 12 selects entry 1 and
///    repositions. Each row asks the row helper (id 18) for a code; code
///    -1 skips to the tail. Otherwise dispatches on the re-read mode:
///    modes 0/1 run an UNSIGNED range check (`ja`) then a two-way index
///    table (default vs. 4-argument log call, id 19), mode 2 the same with
///    the code minus 3, higher modes a direct 3-argument log call
///    (id 20); the default arm picks one of two templates by an equality
///    test against 2. Each logged row then formats text (id 16) and draws
///    (id 17) with two running accumulators.
/// 7. Row tail: fetches pair 0x3E into one of two slots (rows 0-11 vs.
///    12-23) and adds its first word to that half's accumulator, in the
///    original's operand order (fresh word first).
///
/// Signedness that matters: the frame gate is SIGNED (`jge`), the mode
/// wrap is SIGNED (`jg`), the switch range checks are UNSIGNED (`ja`), the
/// row-index and loop bounds are SIGNED (`jl`), the mode and code-2
/// dispatches are equalities. Float order is the original's, pinned with
/// black_box.
///
/// Proof assumptions: the stack fill is zero, which defines the upper
/// bytes of the counter slot on the mode path (on the frame path its upper
/// bytes are the saved FPU control word, 0x037F, and the product's high
/// half) and the word above the low-half pair slot (seen once in a
/// pre-write snapshot). The 0x3D pair slot reuses the conversion slot, so
/// its pre-write snapshot shows the conversion result on the frame path.
/// Frame pointers are skipped in the call comparison and observed through
/// snapshots and downstream use instead.
#[inline(always)]
unsafe fn body() {
    // Callee ids (see the contract generator for conventions).
    const C_GATE: u32 = 1;
    const C_TBL: u32 = 2;
    const C_FMTW: u32 = 3;
    const C_FMTW1: u32 = 4;
    const C_SYNC: u32 = 5;
    const C_DISP: u32 = 6;
    const C_PAL: u32 = 7;
    const C_PUBW: u32 = 8;
    const C_PUB5: u32 = 9;
    const C_MODE: u32 = 10;
    const C_PUBF: u32 = 11;
    const C_PUBC: u32 = 12;
    const C_SEL: u32 = 13;
    const C_POS: u32 = 14;
    const C_PAIR: u32 = 15;
    const C_TEXT: u32 = 16;
    const C_DRAW: u32 = 17;
    const C_ROW: u32 = 18;
    const C_LOG4: u32 = 19;
    const C_LOG3: u32 = 20;
    const C_COOKIE: u32 = 21;

    // Touched globals (file VAs).
    const G_CTR: u32 = 0x11618E4;
    const G_MODECTR: u32 = 0x1030BC8;
    const G_DT: u32 = 0x117359C;
    const G_RATE: u32 = 0xFE8C58;
    const G_GATE: u32 = 0x1161548;
    const G_MODESEL: u32 = 0x11609E4;
    const G_MB: u32 = 0x116C250;
    const G_MF: u32 = 0x116C253;
    const G_TLSIDX: u32 = 0x17ABA14;
    // Thiscall object pointers.
    const T_GATE: u32 = 0x1161548;
    const T_DISP: u32 = 0x1161510;
    const T_TEXT: u32 = 0x116BFF0;
    // Text template addresses.
    const F_M0: u32 = 0xE7DD64;
    const F_M1: u32 = 0xE7DD6C;
    const F_MX: u32 = 0xE7DD74;
    const F_SW3_LOG: u32 = 0xE7DD7C;
    const F_SW3_EQ2: u32 = 0xE7DD8C;
    const F_SW3_NE: u32 = 0xE7DD9C;
    const F_SW2_LOG: u32 = 0xE7DDA8;
    const F_SW2_EQ2: u32 = 0xE7DDB8;
    const F_SW2_NE: u32 = 0xE7DDC8;
    const F_SW1_LOG: u32 = 0xE7DDD4;
    const F_SW1_EQ2: u32 = 0xE7DDE4;
    const F_SW1_NE: u32 = 0xE7DE04;
    const F_DIR_EQ2: u32 = 0xE7DE40;
    const F_DIR_NE: u32 = 0xE7DE70;

    const FRAME_LIMIT: i32 = 1500;
    const MODE_MAGIC: u8 = 0x6A;
    const MODE_WRAP: i32 = 5;
    const NROWS: i32 = 24;
    const HALF: i32 = 12;
    const TLS_OFF: u32 = 0x4C8;
    /// Surviving byte of the saved entry FPU control word (0x037F): the
    /// store writes [0x7F, 0x03] and the gate byte covers the low one.
    const CW_BITS: u32 = 0x0000_0300;
    const F_ONE: u32 = 0x3F80_0000;

    #[inline(always)]
    unsafe fn g8(va: u32) -> u8 {
        unsafe { (lf_checker_rt::relocated(va) as *const u8).read() }
    }
    #[inline(always)]
    unsafe fn g32(va: u32) -> u32 {
        unsafe {
            (lf_checker_rt::relocated(va) as *const u32).read_unaligned()
        }
    }
    #[inline(always)]
    unsafe fn gf(va: u32) -> f32 {
        unsafe { f32::from_bits(g32(va)) }
    }
    #[inline(always)]
    unsafe fn w32(va: u32, v: u32) {
        unsafe {
            (lf_checker_rt::relocated(va) as *mut u32).write_unaligned(v)
        }
    }
    #[inline(always)]
    fn addf(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) + core::hint::black_box(b)
    }
    #[inline(always)]
    fn mulf(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) * core::hint::black_box(b)
    }
    /// x87 `fistp` (chop) of an f32 to i64 as (low, high) dword: truncate
    /// toward zero; NaN or out of `|i64|` range yields the indefinite word.
    /// (-2^63 exactly is representable and also yields low 0, high 0x8000.)
    #[inline(always)]
    fn fistp_qw(f: f32) -> (u32, u32) {
        let x = f as f64;
        if x.is_nan() || x <= -9223372036854775808.0 || x >= 9223372036854775808.0
        {
            (0, 0x8000_0000)
        } else {
            let q = x as i64;
            (q as u32, (q >> 32) as u32)
        }
    }
    /// Mode-2 switch: code-minus-3 values taking the 4-argument log arm.
    #[inline(always)]
    fn sw1_log(t: u32) -> bool {
        matches!(
            t,
            0 | 11 | 12 | 13 | 14 | 15 | 19 | 20 | 24 | 25 | 27 | 28 | 29 | 32 | 33 | 34
        )
    }
    /// Mode-1 switch: codes taking the default (3-argument) arm.
    #[inline(always)]
    fn sw2_dflt(c: u32) -> bool {
        matches!(c, 2 | 4 | 5 | 7 | 9 | 10 | 12 | 13 | 20 | 21 | 25 | 29 | 34)
    }
    /// Mode-0 switch: codes taking the default (3-argument) arm.
    #[inline(always)]
    fn sw3_dflt(c: u32) -> bool {
        matches!(
            c,
            1 | 2 | 4 | 10 | 11 | 12 | 13 | 14 | 19 | 20 | 21 | 23 | 24 | 25 | 26 | 28 | 29 | 34
        )
    }

    // ---- stage 1: frame gate ----
    let ctr = g32(G_CTR);
    // SIGNED: a negative counter still takes the frame path.
    let take_ge = (ctr as i32) >= FRAME_LIMIT;
    // Upper bits of the counter slot (low byte is the gate byte below),
    // and the conversion slot the 0x3D pair slot reuses (zero otherwise).
    let s70_hi: u32;
    let conv_slot: [u32; 2];
    if !take_ge {
        let prod = mulf(gf(G_DT), gf(G_RATE));
        let (qlo, qhi) = fistp_qw(prod);
        w32(G_CTR, ctr.wrapping_add(qlo));
        s70_hi = (prod.to_bits() & 0xFFFF_0000) | CW_BITS;
        conv_slot = [qlo, qhi];
    } else {
        let m = g32(G_MODECTR).wrapping_add(1);
        w32(G_MODECTR, if (m as i32) > MODE_WRAP { 1 } else { m });
        w32(G_CTR, 0);
        s70_hi = 0;
        conv_slot = [0, 0];
    }

    // ---- stage 2: gate byte, first pairs, sync ----
    let mut bl: u8 = 0xFF;
    if g8(G_GATE) != 0 {
        let a = lf_checker_rt::callee_thiscall!(
            C_GATE,
            u32,
            lf_checker_rt::relocated(T_GATE)
        );
        bl = a as u8;
    }
    let esi_pre: u32 = s70_hi | (bl as u32);
    // Counter/palette slot; reused as the low-half pair slot in the loop.
    let mut s70: [u32; 2] = [esi_pre, 0];
    let mut s40: [u32; 2] = [0, 0];
    let mut s38: [u32; 2] = [0, 0];
    lf_checker_rt::callee_cdecl!(C_TBL, u32, s40.as_mut_ptr() as u32, 0x39);
    lf_checker_rt::callee_cdecl!(C_TBL, u32, s38.as_mut_ptr() as u32, 0x3A);
    lf_checker_rt::callee_cdecl!(
        C_FMTW,
        u32,
        1,
        s40.as_mut_ptr() as u32,
        s38.as_mut_ptr() as u32,
        0
    );
    // A pair slot's second word sits 4 bytes above its first: the second
    // blend input is the 0x39 pair's own second word, and the running sum
    // starts from the 0x3A pair's second word.
    let b3 = f32::from_bits(s40[1]);
    let b2 = addf(f32::from_bits(s38[1]), b3);
    let b1 = f32::from_bits(s40[0]);
    let b0 = addf(f32::from_bits(s38[0]), b1);
    // Eight staged words.
    let (f28, f24, f20, f1c, f18, f14, f10, f0c) = (
        b1.to_bits(),
        b2.to_bits(),
        b1.to_bits(),
        b3.to_bits(),
        b0.to_bits(),
        b2.to_bits(),
        b0.to_bits(),
        b3.to_bits(),
    );
    lf_checker_rt::callee_cdecl!(C_SYNC, u32,);
    lf_checker_rt::callee_thiscall!(
        C_DISP,
        u32,
        lf_checker_rt::relocated(T_DISP)
    );
    let color: u32 = ((bl as u32) << 24) | 0x00FF_FFFF;
    // id 12 takes EDI (shifted byte only); the OR-ed word is staged.
    let edival: u32 = (bl as u32) << 24;

    // ---- stage 3: palette word and staged publish ----
    lf_checker_rt::callee_cdecl!(
        C_PAL,
        u32,
        s70.as_mut_ptr() as u32,
        0x3C,
        esi_pre
    );
    lf_checker_rt::callee_cdecl!(C_PUBW, u32, s70[0]);
    // Snapshot neighbours: each staged word's upper neighbour.
    let a28: [u32; 2] = [f28, f24];
    let a20: [u32; 2] = [f20, f1c];
    let a18: [u32; 2] = [f18, f14];
    let a10: [u32; 2] = [f10, f0c];
    let a68: [u32; 1] = [color];
    lf_checker_rt::callee_cdecl!(
        C_PUB5,
        u32,
        a28.as_ptr() as u32,
        a20.as_ptr() as u32,
        a18.as_ptr() as u32,
        a10.as_ptr() as u32,
        a68.as_ptr() as u32
    );

    // ---- stage 4: mode word, scalars, three pairs ----
    let mode_word = |mb: u8, mf: u8| -> u32 {
        if mb == MODE_MAGIC || mf != 0 {
            2
        } else {
            0
        }
    };
    lf_checker_rt::callee_cdecl!(C_MODE, u32, mode_word(g8(G_MB), g8(G_MF)));
    lf_checker_rt::callee_cdecl!(C_PUBF, u32, F_ONE);
    lf_checker_rt::callee_cdecl!(C_PUBC, u32, edival);
    let mut s48: [u32; 2] = [0, 0];
    let mut s5c: [u32; 2] = [0, 0];
    // The 0x3D slot reuses the conversion slot: its pre-write snapshot
    // shows the conversion result on the frame path, zeroes otherwise.
    let mut s54: [u32; 2] = conv_slot;
    lf_checker_rt::callee_cdecl!(C_TBL, u32, s48.as_mut_ptr() as u32, 0x3B);
    lf_checker_rt::callee_cdecl!(C_TBL, u32, s5c.as_mut_ptr() as u32, 0x3C);
    lf_checker_rt::callee_cdecl!(C_TBL, u32, s54.as_mut_ptr() as u32, 0x3D);
    lf_checker_rt::callee_cdecl!(
        C_FMTW,
        u32,
        1,
        s48.as_mut_ptr() as u32,
        s54.as_mut_ptr() as u32,
        0
    );
    lf_checker_rt::callee_cdecl!(
        C_FMTW1,
        u32,
        1,
        s5c.as_mut_ptr() as u32,
        0,
        0
    );

    // ---- stage 5: select/position, pair publish, text, draw ----
    lf_checker_rt::callee_cdecl!(C_SEL, u32, 0);
    lf_checker_rt::callee_cdecl!(C_POS, u32, 0, F_ONE);
    let mut s64: [u32; 2] = [0, 0];
    let mut s30: [u32; 2] = [0, 0];
    lf_checker_rt::callee_cdecl!(C_TBL, u32, s64.as_mut_ptr() as u32, 0x74);
    lf_checker_rt::callee_cdecl!(C_TBL, u32, s30.as_mut_ptr() as u32, 0x75);
    lf_checker_rt::callee_cdecl!(
        C_FMTW,
        u32,
        1,
        s64.as_mut_ptr() as u32,
        s30.as_mut_ptr() as u32,
        0
    );
    lf_checker_rt::callee_cdecl!(C_PAIR, u32, s30[0], s30[1]);
    let mode = g32(G_MODESEL);
    let fmt0 = if mode == 0 {
        lf_checker_rt::relocated(F_M0)
    } else if mode == 1 {
        lf_checker_rt::relocated(F_M1)
    } else {
        lf_checker_rt::relocated(F_MX)
    };
    let t16 =
        lf_checker_rt::callee_thiscall!(C_TEXT, u32, lf_checker_rt::relocated(T_TEXT), fmt0);
    // Last pushed is arg0: the two -1 words sit highest.
    lf_checker_rt::callee_cdecl!(
        C_DRAW, u32, s64[0], s64[1], t16, 0xFFFF_FFFF, 0xFFFF_FFFF
    );
    lf_checker_rt::callee_cdecl!(
        C_PAL,
        u32,
        s70.as_mut_ptr() as u32,
        0x3C,
        esi_pre
    );
    lf_checker_rt::callee_cdecl!(C_PUBW, u32, s70[0]);
    lf_checker_rt::callee_cdecl!(C_MODE, u32, mode_word(g8(G_MB), g8(G_MF)));
    lf_checker_rt::callee_cdecl!(C_PAIR, u32, s54[0], s54[1]);
    lf_checker_rt::callee_cdecl!(C_SEL, u32, 2);
    lf_checker_rt::callee_cdecl!(C_POS, u32, 0, s48[0]);

    // ---- stage 6: 24-row scan ----
    let tls_ptr =
        lf_checker_rt::tls_slot(g32(G_TLSIDX) as usize).wrapping_add(TLS_OFF);
    // The accumulators start from the 0x3C/0x3B pairs' second words; the
    // high-half slot reuses the blend scratch (last sums b0 above b3).
    let mut acc_hi: u32 = s5c[1];
    let mut acc_lo: u32 = s48[1];
    let mut s78: [u32; 2] = [b0.to_bits(), b3.to_bits()];
    let mut edi: i32 = 0;
    while edi < NROWS {
        if edi == HALF {
            lf_checker_rt::callee_cdecl!(C_SEL, u32, 1);
            lf_checker_rt::callee_cdecl!(C_POS, u32, 0, F_ONE);
        }
        let ecx = lf_checker_rt::callee_cdecl!(C_ROW, u32, edi as u32);
        if ecx != 0xFFFF_FFFF {
            let mode = g32(G_MODESEL);
            if mode == 0 {
                if ecx > 0x25 || sw3_dflt(ecx) {
                    let f = if ecx == 2 { F_SW3_EQ2 } else { F_SW3_NE };
                    lf_checker_rt::callee_cdecl!(
                        C_LOG3,
                        u32,
                        tls_ptr,
                        lf_checker_rt::relocated(f),
                        ecx
                    );
                } else {
                    lf_checker_rt::callee_cdecl!(
                        C_LOG4,
                        u32,
                        tls_ptr,
                        lf_checker_rt::relocated(F_SW3_LOG),
                        ecx,
                        g32(G_MODECTR)
                    );
                }
            } else if mode == 1 {
                // UNSIGNED: codes above 0x25 take the default arm.
                let above: bool = ecx > 0x25;
                if above || sw2_dflt(ecx) {
                    let f = if ecx == 2 { F_SW2_EQ2 } else { F_SW2_NE };
                    lf_checker_rt::callee_cdecl!(
                        C_LOG3,
                        u32,
                        tls_ptr,
                        lf_checker_rt::relocated(f),
                        ecx
                    );
                } else {
                    lf_checker_rt::callee_cdecl!(
                        C_LOG4,
                        u32,
                        tls_ptr,
                        lf_checker_rt::relocated(F_SW2_LOG),
                        ecx,
                        g32(G_MODECTR)
                    );
                }
            } else if mode == 2 {
                let t = ecx.wrapping_sub(3);
                if t > 0x22 || !sw1_log(t) {
                    let f = if ecx == 2 { F_SW1_EQ2 } else { F_SW1_NE };
                    lf_checker_rt::callee_cdecl!(
                        C_LOG3,
                        u32,
                        tls_ptr,
                        lf_checker_rt::relocated(f),
                        ecx
                    );
                } else {
                    lf_checker_rt::callee_cdecl!(
                        C_LOG4,
                        u32,
                        tls_ptr,
                        lf_checker_rt::relocated(F_SW1_LOG),
                        ecx,
                        g32(G_MODECTR)
                    );
                }
            } else if ecx == 2 {
                lf_checker_rt::callee_cdecl!(
                    C_LOG3,
                    u32,
                    tls_ptr,
                    lf_checker_rt::relocated(F_DIR_EQ2),
                    ecx
                );
            } else {
                lf_checker_rt::callee_cdecl!(
                    C_LOG3,
                    u32,
                    tls_ptr,
                    lf_checker_rt::relocated(F_DIR_NE),
                    ecx
                );
            }
            let t16b = lf_checker_rt::callee_thiscall!(
                C_TEXT,
                u32,
                lf_checker_rt::relocated(T_TEXT),
                tls_ptr
            );
            if edi >= HALF {
                lf_checker_rt::callee_cdecl!(
                    C_DRAW,
                    u32,
                    s5c[0],
                    acc_hi,
                    t16b,
                    0xFFFF_FFFF,
                    0xFFFF_FFFF
                );
            } else {
                lf_checker_rt::callee_cdecl!(
                    C_DRAW, u32, 0, acc_lo, t16b, 0xFFFF_FFFF, 0xFFFF_FFFF
                );
            }
        }
        // ---- row tail: accumulate pair 0x3E into this half ----
        if edi >= HALF {
            lf_checker_rt::callee_cdecl!(C_TBL, u32, s78.as_mut_ptr() as u32, 0x3E);
            acc_hi = addf(f32::from_bits(s78[0]), f32::from_bits(acc_hi)).to_bits();
        } else {
            lf_checker_rt::callee_cdecl!(C_TBL, u32, s70.as_mut_ptr() as u32, 0x3E);
            acc_lo = addf(f32::from_bits(s70[0]), f32::from_bits(acc_lo)).to_bits();
        }
        edi += 1;
    }
    lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
}

lf_checker_rt::export!(cdecl, rw_008BC550() -> () {
    unsafe { body() }
});
