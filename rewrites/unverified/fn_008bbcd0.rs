// original: 0x008BBCD0 input_ui_menu_options_a (proposed)

/// Menu-options refresh, group A (input-ui): recomputes a set of option rows
/// from globals and helper answers and reports whether anything is active.
///
/// Calling convention: cdecl, no arguments, boolean in AL (the upper 24 bits
/// of EAX are residue: stub-answer leftovers on one exit path, the frame
/// address on the other; the contract compares AL only). The function reads
/// no incoming registers or stack arguments; every input is a global, a
/// floating constant, a callee answer, or one fabricated TLS slot.
///
/// Behaviour in order:
/// 1. Asks the gate helper (id 1); on a zero answer compares a mode byte and
///    optionally refreshes a level byte through the level helper (id 2), then
///    runs the level through a fixed double-precision rounding sequence
///    (multiply by 0.5, add/subtract signed 2^52, a corrective step) and
///    truncates to an integer, keeping the low byte.
/// 2. Publishes three option words (constant 1.0, the level byte shifted to
///    the top byte, constant 1), then a mode word (7 unless a mode byte is
///    0x6A or a flag byte is set, else 2).
/// 3. Fetches two float pairs through the table getter (id 7, indices 0x1F
///    and 0x1E), blends them with the scale helper (id 8), and publishes two
///    float pairs; resolves a colour word through the palette helper
///    (id 11, index 0x3B, low byte of the saved level) and publishes it.
/// 4. Depending on the status helper (id 13) and a presence flag, either adds
///    a third table row (index 0x22) to the running value or keeps it, adds a
///    global offset, and, when a second status check passes, formats a label
///    through the text helpers (ids 14, 15, 16); otherwise scans up to N
///    entries (N = the count helper's SIGNED answer, ids 17/18/19) accumulating
///    a float with the measure helpers (ids 21/22) and publishing progress.
/// 5. Unless skipped by the presence flag and status, renders one row through
///    the row helpers (ids 6, 7, 23, 24, 25, 26), copies a label through the
///    string helper (id 27), scans a second table (ids 28, 11, 12, 7, 8, 9)
///    with a bounded retry loop (at most a few rounds, id 29 answering a
///    SIGNED greater-than-one test), finishes the row (ids 15, 30, 23), and
///    blends four colours selected by four status answers (id 31) with the
///    palette and paint helpers (ids 11, 32).
/// 6. Exit: when the final status check passes and the presence flag is set,
///    clears the active flag and returns 1; otherwise compares the running
///    value against a fresh table row (index 0x16) into the active flag and
///    returns 1 if the scan marked anything, else toggles the done flag and
///    returns 0.
///
/// Signedness that matters: the entry-scan bound compares SIGNED (jl), the
/// retry loop compares SIGNED against 1 (jle/jg), the 0xAA floor on the level
/// byte is UNSIGNED (cmovb). The float operation order below is the
/// original's, pinned with black_box; NaN payloads propagate through the same
/// SSE sequence on both sides.
///
/// Proof assumptions (see results.json `narrowed`): the stack fill is zero,
/// which defines the upper bytes of the saved-level slot (read once as a
/// dword) and the running float on paths that never store to it; frame
/// pointers passed to callees are skipped in the call comparison and observed
/// through snapshots and downstream use instead.
#[inline(always)]
unsafe fn body() -> u32 {
        // Callee ids (see the contract generator for conventions).
        const C_GATE: u32 = 1;
        const C_LEVEL: u32 = 2;
        const C_PUB_F: u32 = 3;
        const C_PUB_B: u32 = 4;
        const C_PUB_1: u32 = 5;
        const C_MODE: u32 = 6;
        const C_TBL: u32 = 7;
        const C_SCALE: u32 = 8;
        const C_PUB_FF: u32 = 9;
        const C_PUB_GG: u32 = 10;
        const C_PAL: u32 = 11;
        const C_PUB_W: u32 = 12;
        const C_STATUS: u32 = 13;
        const C_TEXT: u32 = 14;
        const C_LABEL: u32 = 15;
        const C_LABEL_END: u32 = 16;
        const C_COUNT: u32 = 17;
        const C_SCAN: u32 = 18;
        const C_CHECK: u32 = 19;
        const C_FLAG: u32 = 20;
        const C_MEASURE: u32 = 21;
        const C_GAIN: u32 = 22;
        const C_PREP: u32 = 23;
        const C_ROW_BEGIN: u32 = 24;
        const C_ROW: u32 = 25;
        const C_ROW_END: u32 = 26;
        const C_STRCPY: u32 = 27;
        const C_HAS: u32 = 28;
        const C_RETRY: u32 = 29;
        const C_FINISH: u32 = 30;
        const C_PICK: u32 = 31;
        const C_PAINT: u32 = 32;
        const C_COOKIE: u32 = 33;

        // Touched globals (file VAs).
        const G_MODE: u32 = 0x1161548;
        const G_MODE2: u32 = 0x116C250;
        const G_MODE2F: u32 = 0x116C253;
        const G_PRESENT: u32 = 0x11DB248;
        const G_PRESENT2: u32 = 0x11DB258;
        const G_OFFSET: u32 = 0x11609DC;
        const G_ACTIVE: u32 = 0x11609D5;
        const G_DONE: u32 = 0x11609D6;
        const G_TLSIDX: u32 = 0x17ABA14;
        const G_TABIDX: u32 = 0x118E944;
        const G_TABLE: u32 = 0x118E7F8;
        const G_KIND: u32 = 0x11D6FD4;
        const G_COL_A0: u32 = 0x105C880;
        const G_COL_A1: u32 = 0x105C87C;
        const G_COL_B0: u32 = 0x105C884;
        const G_COL_B1: u32 = 0x105C888;
        const TEXT_THIS: u32 = 0x116BFF0;
        const TEXT_ARG_A: u32 = 0x11DB258;
        const TEXT_ARG_B: u32 = 0x11DB248;

        // Floating constants from the read-only section.
        const D_SIGN: u64 = 0x8000_0000_0000_0000; // -0.0, sign mask
        const D_MAG: f64 = f64::from_bits(0x4330_0000_0000_0000); // 2^52
        const D_MAG_BITS: u64 = 0x4330_0000_0000_0000;
        const D_ONE_BITS: u64 = 0x3FF0_0000_0000_0000; // 1.0
        const F_HALF: f32 = 0.5;
        const F_ONE: f32 = 1.0;
        const F_STEP: f32 = f32::from_bits(0x3C23_D70A); // ~0.01
        const F_LIMIT: f32 = f32::from_bits(0x3F33_3333); // ~0.7
        const F_SMALL_A: f32 = f32::from_bits(0x3B03_126F); // ~0.002
        const F_SMALL_B: f32 = f32::from_bits(0x3B83_126F); // ~0.004

        const LEVEL_FLOOR: u8 = 0xAA;
        const MODE_MAGIC: u8 = 0x6A;
        const OPAQUE_BLACK: u32 = 0xFF00_0000;

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
        unsafe fn w8(va: u32, v: u8) {
            unsafe { (lf_checker_rt::relocated(va) as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn m16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        fn addf(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mulf(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn subf(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn addd(a: f64, b: f64) -> f64 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn subd(a: f64, b: f64) -> f64 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        // Truncate a double to 32 bits exactly like cvttsd2si (out of range
        // or NaN gives 0x80000000).
        #[inline(always)]
        fn cvtt(x: f64) -> u32 {
            if x.is_nan() || x < -2147483648.0 || x >= 2147483648.0 {
                0x8000_0000
            } else {
                (x as i32) as u32
            }
        }
        // The status word of the table entry selected by the global index.
        unsafe fn tab_word() -> u16 {
            let idx = g32(G_TABIDX);
            let ent = g32(G_TABLE.wrapping_add(idx.wrapping_mul(4)));
            m16(ent.wrapping_add(0x6C))
        }
        // The selected table entry pointer itself.
        unsafe fn tab_entry() -> u32 {
            let idx = g32(G_TABIDX);
            g32(G_TABLE.wrapping_add(idx.wrapping_mul(4)))
        }
        // The fabricated TLS block address the original derives from its
        // slot, plus the 0x78 header the helpers use.
        #[inline(always)]
        unsafe fn tls_blk(slot: u32) -> u32 {
            lf_checker_rt::tls_slot(slot as usize).wrapping_add(0x78)
        }

        // ---- stage 1: gate, level, fixed double rounding ----
        let mut bl: u8 = 0xFF;
        // Saved-level slot (low byte defined, upper bytes are the zero fill).
        let mut lvl_slot: u32 = 0xFF;
        let gate = lf_checker_rt::callee_cdecl!(C_GATE, u32, 0);
        if (gate as u8) == 0 {
            if g8(G_MODE) != 0 {
                let lv = lf_checker_rt::callee_thiscall!(
                    C_LEVEL,
                    u32,
                    lf_checker_rt::relocated(G_MODE)
                );
                bl = lv as u8;
            }
            let t = mulf(bl as f32, F_HALF);
            let x3 = t as f64;
            let x3b = x3.to_bits();
            let x4b = D_SIGN & x3b;
            let mag = f64::from_bits(x3b ^ x4b);
            let m0: u64 = if mag < D_MAG { !0 } else { 0 };
            let x2 = f64::from_bits((D_MAG_BITS & m0) | x4b);
            let mut x1 = addd(x3, x2);
            x1 = subd(x1, x2);
            let r = subd(x1, x3);
            let x4 = f64::from_bits(x4b);
            let m1: u64 = if !(r <= x4) { !0 } else { 0 };
            let corr = f64::from_bits(m1 & D_ONE_BITS);
            x1 = subd(x1, corr);
            bl = cvtt(x1) as u8;
            lvl_slot = bl as u32;
        }

        // ---- stage 2: publish the first option words ----
        lf_checker_rt::callee_cdecl!(C_PUB_F, u32, F_ONE.to_bits());
        lf_checker_rt::callee_cdecl!(C_PUB_B, u32, (bl as u32) << 24);
        lf_checker_rt::callee_cdecl!(C_PUB_1, u32, 1);
        let mut acc: u32 = F_ONE.to_bits(); // running step value
        let mode_arg: u32 =
            if g8(G_MODE2) == MODE_MAGIC || g8(G_MODE2F) != 0 {
                2
            } else {
                7
            };
        lf_checker_rt::callee_cdecl!(C_MODE, u32, mode_arg);

        // ---- stage 3: first float pairs ----
        let mut p68 = [0u32; 2];
        lf_checker_rt::callee_cdecl!(
            C_TBL, u32, p68.as_mut_ptr() as u32, 0x1F
        );
        let mut p3c = [0u32; 2];
        lf_checker_rt::callee_cdecl!(
            C_TBL, u32, p3c.as_mut_ptr() as u32, 0x1E
        );
        let mut run: u32 =
            subf(F_ONE, f32::from_bits(p68[0])).to_bits();
        lf_checker_rt::callee_cdecl!(
            C_SCALE, u32, 2, p68.as_mut_ptr() as u32,
            p3c.as_mut_ptr() as u32, 0
        );
        lf_checker_rt::callee_cdecl!(C_PUB_FF, u32, p3c[0], p3c[1]);
        lf_checker_rt::callee_cdecl!(C_PUB_GG, u32, p68[0], run);
        let ebp_v: u32 = lvl_slot;
        // Frame slot s44 is shared by the palette call here, the table
        // call below (full path) and the second palette call: snapshots
        // observe pre-call contents, so the buffer must carry them over.
        let mut s44 = [0u32; 2];
        lf_checker_rt::callee_cdecl!(
            C_PAL, u32, s44.as_mut_ptr() as u32, 0x3B, ebp_v
        );
        lf_checker_rt::callee_cdecl!(C_PUB_W, u32, s44[0]);

        // ---- stage 4: running value, offset, label or scan ----
        let edi_slot: u32 = g32(G_TLSIDX);
        let g1 = lf_checker_rt::callee_cdecl!(C_STATUS, u32,);
        let tail64: u32;
        // Shared frame slot (see s44): reused by the scan loop and the tail.
        let mut s5c = [0u32; 2];
        if (g1 as u8) != 0 {
            if g8(G_PRESENT) != 0 {
                lf_checker_rt::callee_cdecl!(
                    C_TBL, u32, s5c.as_mut_ptr() as u32, 0x22
                );
                tail64 = addf(
                    f32::from_bits(s5c[1]),
                    f32::from_bits(p68[1]),
                )
                .to_bits();
            } else {
                tail64 = p68[1];
            }
        } else if tab_word() != 0 {
            lf_checker_rt::callee_cdecl!(
                C_TBL, u32, s5c.as_mut_ptr() as u32, 0x22
            );
            tail64 =
                addf(f32::from_bits(s5c[1]), f32::from_bits(p68[1]))
                    .to_bits();
        } else {
            tail64 = p68[1];
        }
        let mut marked = false;
        // Products slot (zero fill until the scan loop stores to it).
        let mut f4c_slot: u32 = 0;
        run = addf(gf(G_OFFSET), f32::from_bits(tail64)).to_bits();
        let g2 = lf_checker_rt::callee_cdecl!(C_STATUS, u32,);
        if (g2 as u8) != 0 && g8(G_PRESENT2) != 0 {
            let r = lf_checker_rt::callee_thiscall!(
                C_TEXT,
                u32,
                lf_checker_rt::relocated(TEXT_THIS),
                lf_checker_rt::relocated(TEXT_ARG_A)
            );
            lf_checker_rt::callee_cdecl!(
                C_LABEL, u32, p68[0], run, r, 0xFFFF_FFFF, 0xFFFF_FFFF
            );
            lf_checker_rt::callee_cdecl!(C_LABEL_END, u32,);
        } else {
            let d6 = g8(G_DONE) as u32;
            let r0 = lf_checker_rt::callee_cdecl!(C_COUNT, u32, d6);
            if (r0 as i32) > 0 {
                let mut esi: i32 = 0;
                loop {
                    let rr = lf_checker_rt::callee_cdecl!(
                        C_SCAN, u32, d6, esi as u32, tls_blk(edi_slot)
                    );
                    if (rr as u8) != 0 {
                        marked = true;
                        let rr2 = lf_checker_rt::callee_cdecl!(
                            C_CHECK, u32, d6, esi as u32
                        );
                        if (rr2 as u8) != 0 {
                            lf_checker_rt::callee_cdecl!(C_FLAG, u32, 1);
                        }
                        lf_checker_rt::callee_cdecl!(
                            C_LABEL, u32, p68[0], run, tls_blk(edi_slot),
                            0xFFFF_FFFF, 0xFFFF_FFFF
                        );
                        lf_checker_rt::callee_cdecl!(C_LABEL_END, u32,);
                        lf_checker_rt::callee_cdecl!(C_FLAG, u32, 0);
                        let cm = lf_checker_rt::callee_cdecl!(
                            C_MEASURE, u32, p68[0], run, tls_blk(edi_slot),
                            0
                        );
                        let gain: f32 =
                            lf_checker_rt::callee_cdecl!(C_GAIN, f32,);
                        lvl_slot = gain.to_bits();
                        let prod = mulf(
                            f32::from_bits(lvl_slot),
                            (cm as i32) as f32,
                        );
                        f4c_slot = prod.to_bits();
                        lf_checker_rt::callee_cdecl!(
                            C_TBL, u32, s5c.as_mut_ptr() as u32, 0x20
                        );
                        run = addf(
                            addf(f32::from_bits(s5c[0]), prod),
                            f32::from_bits(run),
                        )
                        .to_bits();
                    }
                    esi = esi.wrapping_add(1);
                    let r =
                        lf_checker_rt::callee_cdecl!(C_COUNT, u32, d6);
                    if !(esi < (r as i32)) {
                        break;
                    }
                }
            }
        }

        // ---- stage 5: the long middle (skippable) ----
        let mut full = false;
        if g8(G_PRESENT) != 0 {
            full = true;
        } else {
            let g4 = lf_checker_rt::callee_cdecl!(C_STATUS, u32,);
            if (g4 as u8) == 0 && tab_word() != 0 {
                full = true;
            }
        }
        if full {
            lf_checker_rt::callee_cdecl!(C_MODE, u32, 7);
            lf_checker_rt::callee_cdecl!(
                C_TBL, u32, s44.as_mut_ptr() as u32, 0x6C
            );
            let mut s74 = [0u32; 2];
            lf_checker_rt::callee_cdecl!(
                C_TBL, u32, s74.as_mut_ptr() as u32, 0x22
            );
            let mut s4c = [f4c_slot, 0];
            lf_checker_rt::callee_cdecl!(
                C_TBL, u32, s4c.as_mut_ptr() as u32, 0
            );
            lvl_slot = addf(
                f32::from_bits(s74[1]),
                f32::from_bits(s4c[0]),
            )
            .to_bits();
            lf_checker_rt::callee_cdecl!(C_PREP, u32, 0);
            let x1 = addf(
                f32::from_bits(s44[1]),
                f32::from_bits(lvl_slot),
            );
            lvl_slot = x1.to_bits();
            let x0b = addf(x1, f32::from_bits(s44[0]));
            // The original builds the row block through this slot as
            // scratch, leaving 1.0f behind; the paint call below snapshots
            // it, so the value is observed.
            s74[0] = F_ONE.to_bits();
            let (m08, m18) = (x0b.to_bits(), x0b.to_bits());
            lf_checker_rt::callee_cdecl!(C_ROW_BEGIN, u32,);
            s4c[0] = OPAQUE_BLACK;
            let a0 = [0u32, 0u32];
            let a1 = [0u32, m18];
            let a2 = [F_ONE.to_bits(), 0u32];
            let a3 = [F_ONE.to_bits(), m08];
            lf_checker_rt::callee_cdecl!(
                C_ROW, u32, a0.as_ptr() as u32, a1.as_ptr() as u32,
                a2.as_ptr() as u32, a3.as_ptr() as u32,
                s4c.as_ptr() as u32
            );
            lf_checker_rt::callee_cdecl!(C_ROW_END, u32,);
            ((tls_blk(edi_slot)) as *mut u16).write(0);
            let g3 = lf_checker_rt::callee_cdecl!(C_STATUS, u32,);
            if (g3 as u8) != 0 {
                if g8(G_PRESENT) != 0 {
                    let r = lf_checker_rt::callee_thiscall!(
                        C_TEXT,
                        u32,
                        lf_checker_rt::relocated(TEXT_THIS),
                        lf_checker_rt::relocated(TEXT_ARG_B)
                    );
                    lf_checker_rt::callee_cdecl!(
                        C_STRCPY, u32, tls_blk(edi_slot), r, 0xFFFF_FFFF
                    );
                }
            } else if tab_word() != 0 {
                let ent = tab_entry().wrapping_add(0x6C);
                lf_checker_rt::callee_cdecl!(
                    C_STRCPY, u32, tls_blk(edi_slot), ent, 0xFFFF_FFFF
                );
            }
            let _has = lf_checker_rt::callee_cdecl!(C_HAS, u32, 0);
            lf_checker_rt::callee_cdecl!(
                C_PAL, u32, s44.as_mut_ptr() as u32, 0x41, ebp_v
            );
            lf_checker_rt::callee_cdecl!(C_PUB_W, u32, s44[0]);
            let mut k54 = [0u32; 2];
            lf_checker_rt::callee_cdecl!(
                C_TBL, u32, k54.as_mut_ptr() as u32, 0x23
            );
            lf_checker_rt::callee_cdecl!(
                C_TBL, u32, s5c.as_mut_ptr() as u32, 0x24
            );
            lf_checker_rt::callee_cdecl!(
                C_SCALE, u32, 2, k54.as_mut_ptr() as u32,
                s5c.as_mut_ptr() as u32, 0
            );
            lf_checker_rt::callee_cdecl!(C_PUB_FF, u32, s5c[0], s5c[1]);
            let c1 = lf_checker_rt::callee_cdecl!(
                C_RETRY, u32, k54[0], k54[1], tls_blk(edi_slot), 0
            );
            if (c1 as i32) > 1 {
                loop {
                    let x1 = subf(f32::from_bits(acc), F_STEP);
                    acc = x1.to_bits();
                    if F_LIMIT > x1 {
                        break;
                    }
                    let px = mulf(x1, f32::from_bits(s5c[0]));
                    lf_checker_rt::callee_cdecl!(
                        C_PUB_FF, u32, px.to_bits(), s5c[1]
                    );
                    let cn = lf_checker_rt::callee_cdecl!(
                        C_RETRY, u32, k54[0], k54[1], tls_blk(edi_slot), 0
                    );
                    if !((cn as i32) > 1) {
                        break;
                    }
                }
            }
            lf_checker_rt::callee_cdecl!(
                C_LABEL, u32, k54[0], k54[1], tls_blk(edi_slot),
                0xFFFF_FFFF, 0xFFFF_FFFF
            );
            let f980: f32 = lf_checker_rt::callee_cdecl!(
                C_FINISH, f32, tls_blk(edi_slot), 1
            );
            let fin: u32 = f980.to_bits();
            lf_checker_rt::callee_cdecl!(C_PREP, u32, 0);
            let bl2: u8 = if bl < LEVEL_FLOOR { LEVEL_FLOOR } else { bl };
            let pal_arg: u32 = (s44[0] & 0xFFFF_FF00) | (bl2 as u32);
            acc = if g32(G_KIND) != 2 {
                F_SMALL_A.to_bits()
            } else {
                F_SMALL_B.to_bits()
            };
            let b1 = lf_checker_rt::callee_cdecl!(C_PICK, u32,);
            let mut ebx_v: u32 = g32(G_COL_A0);
            if (b1 as u8) != 0 {
                ebx_v = g32(G_COL_A1);
            }
            let b2 = lf_checker_rt::callee_cdecl!(C_PICK, u32,);
            let mut edi_v: u32 = g32(G_COL_B0);
            if (b2 as u8) != 0 {
                edi_v = g32(G_COL_B1);
            }
            let b3 = lf_checker_rt::callee_cdecl!(C_PICK, u32,);
            let mut esi_v: u32 = g32(G_COL_A0);
            if (b3 as u8) != 0 {
                esi_v = g32(G_COL_A1);
            }
            let b4 = lf_checker_rt::callee_cdecl!(C_PICK, u32,);
            let mut ecx_v: u32 = g32(G_COL_B0);
            if (b4 as u8) != 0 {
                ecx_v = g32(G_COL_B1);
            }
            let x3_0 = f32::from_bits(lvl_slot);
            let x0_1 = mulf((ecx_v as i32) as f32, f32::from_bits(k54[0]));
            let x1_1 =
                addf(f32::from_bits(fin), f32::from_bits(k54[0]));
            let h34 = x0_1.to_bits();
            let x0_2 = mulf((esi_v as i32) as f32, x3_0);
            let x3_1 = subf(x3_0, f32::from_bits(acc));
            let h28 = x0_2.to_bits();
            let x1_2 = mulf(x1_1, (edi_v as i32) as f32);
            let h2c = x1_2.to_bits();
            let x3_2 = mulf(x3_1, (ebx_v as i32) as f32);
            let h30 = x3_2.to_bits();
            // The palette helper returns its frame-pointer argument; the
            // original forwards it to the paint helper. The stub cannot
            // return our frame pointer (preserve keeps its own entry
            // garbage), so pass our buffer address directly; the argument
            // is skipped in the comparison (layouts differ) and its word
            // is snapshotted instead.
            lf_checker_rt::callee_cdecl!(
                C_PAL, u32, s74.as_mut_ptr() as u32, 0x42, pal_arg
            );
            let hblk = [h34, h30, h2c, h28];
            lf_checker_rt::callee_cdecl!(
                C_PAINT, u32, hblk.as_ptr() as u32,
                s74.as_mut_ptr() as u32
            );
            lf_checker_rt::callee_cdecl!(C_MODE, u32, 7);
        }

        // ---- stage 6: exit ----
        let g5 = lf_checker_rt::callee_cdecl!(C_STATUS, u32,);
        if (g5 as u8) != 0 && g8(G_PRESENT) != 0 {
            w8(G_ACTIVE, 0);
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
            return 1;
        }
        lf_checker_rt::callee_cdecl!(
            C_TBL, u32, s5c.as_mut_ptr() as u32, 0x16
        );
        w8(
            G_ACTIVE,
            if f32::from_bits(run) > f32::from_bits(s5c[0]) {
                1
            } else {
                0
            },
        );
        if marked {
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
            return 1;
        }
        let d6c = g8(G_DONE);
        w8(G_DONE, if d6c == 0 { 1 } else { 0 });
        lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
        0
}

lf_checker_rt::export!(cdecl, rw_008BBCD0() -> u32 {
    unsafe { body() }
});
