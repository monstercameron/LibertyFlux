// original: 0x005B4770 display_aspect_select (proposed)

/// Match the display aspect ratio against five known ratios and fetch the two
/// getter words for the selected output.
///
/// Prologue: a flag byte chooses between a mode-table path (callee 0 answers a
/// 1-based index; two dwords are read from a 16-byte entry and converted from
/// unsigned int to float through a double) and a thread-id path (two import
/// calls, callee 1; each answer is compared against a global and selects one of
/// two signed dwords, converted to float). Either way the two floats are
/// multiplied and the product is compared, strictly greater (NaN counts as
/// false), against 921600.0; the boolean gates the second stage. A thiscall
/// (callee 2, constant object, one stack word of 1) answers the aspect ratio on
/// the x87 stack.
///
/// Six stages follow. Stages 1-4 compare |aspect - C| against 0.01 for C in
/// {16/9, 4/3, 5/3, 16/10}; stage 2 additionally requires the prologue boolean.
/// Stage 5 compares |aspect - 5/4| against 0.01. Stage 6 always runs. Each
/// comparison uses the original's instruction order with an explicit
/// not-less-than (an unordered/NaN result falls through to the next stage, and
/// the absolute value negates only on strict less-than, so -0.0 and NaN pass
/// through unchanged). The first stage whose check passes dispatches on the
/// selector argument minus one, UNSIGNED: 0-4 index that stage's pair of getter
/// indexes, anything else (including selector 0) takes the default. A taken
/// case calls the getter (callee 3) twice and copies the two dwords behind each
/// answer into the two out-pointers; the default writes (0, 0) and (1.0, 1.0).
/// Takes (out1, out2, sel), returns nothing (cdecl, 3 args).
lf_checker_rt::export!(cdecl, rw_005B4770(out1: u32, out2: u32, sel: u32) -> u32 {
    unsafe {
        const FLAG_BYTE: u32 = 0x17ACCF8;
        const MODE_TABLE: u32 = 0x1168BB0;
        const ENTRY_LEN: u32 = 16;
        const TID_CMP: u32 = 0x110DD14;
        const EDI_ELSE: u32 = 0x105C884;
        const EDI_EQ: u32 = 0x105C888;
        const ECX_ELSE: u32 = 0x105C880;
        const ECX_EQ: u32 = 0x105C87C;
        const ASPECT_OBJ: u32 = 0x118D7F0;
        const WIDE_THRESH: u32 = 0xFE8CF0;
        const TOL: u32 = 0xFE870C;
        const C_S1: u32 = 0xFE89B8;
        const C_S2: u32 = 0xFE8934;
        const C_S3: u32 = 0xFE8994;
        const C_S4: u32 = 0xFE897C;
        const C_S5: u32 = 0xFE8920;
        const C_MODE: u32 = 0;
        const C_TID: u32 = 1;
        const C_ASPECT: u32 = 2;
        const C_GETTER: u32 = 3;
        // (first-getter index, second-getter index) per (stage, case).
        const CASES: [[(u32, u32); 5]; 6] = [
            [(0x8D, 0x88), (0x8E, 0x89), (0x8F, 0x8A), (0x90, 0x8B), (0x91, 0x8C)],
            [(0xDE, 0xD9), (0xDF, 0xDA), (0xE0, 0xDB), (0xE1, 0xDC), (0xE2, 0xDD)],
            [(0xC0, 0xBB), (0xC1, 0xBC), (0xC2, 0xBD), (0xC3, 0xBE), (0xC4, 0xBF)],
            [(0xCA, 0xC5), (0xCB, 0xC6), (0xCC, 0xC7), (0xCD, 0xC8), (0xCE, 0xC9)],
            [(0xD4, 0xCF), (0xD5, 0xD0), (0xD6, 0xD1), (0xD7, 0xD2), (0xD8, 0xD3)],
            [(0x8D, 0x88), (0x8E, 0x89), (0x8F, 0x8A), (0x90, 0x8B), (0x91, 0x8C)],
        ];

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        /// Unsigned int to float exactly as the original's convert-to-double,
        /// conditionally add 2^32, convert-to-float sequence (the double holds
        /// every u32 exactly, so this is one rounding either way).
        #[inline(always)]
        fn u32_to_f32(v: u32) -> f32 {
            (v as f64) as f32
        }
        /// Absolute value exactly as comiss-zero, jbe, xor-with-minus-zero:
        /// negate only on strict ordered less-than.
        #[inline(always)]
        fn fabs_stage(x: f32) -> f32 {
            if x < 0.0 {
                -x
            } else {
                x
            }
        }

        // Prologue: two floats whose product gates stage 2.
        let (f0, f1) = if rd8(lf_checker_rt::relocated(FLAG_BYTE)) == 0 {
            let mode = lf_checker_rt::callee_cdecl!(C_MODE, u32,);
            let table = rd32(lf_checker_rt::relocated(MODE_TABLE));
            let entry = table.wrapping_add(mode.wrapping_sub(1).wrapping_mul(ENTRY_LEN));
            (u32_to_f32(rd32(entry)), u32_to_f32(rd32(entry.wrapping_add(4))))
        } else {
            let x = rd32(lf_checker_rt::relocated(TID_CMP));
            let t0 = lf_checker_rt::callee_cdecl!(C_TID, u32,);
            let edi = rd32(lf_checker_rt::relocated(if t0 == x { EDI_EQ } else { EDI_ELSE }));
            let t1 = lf_checker_rt::callee_cdecl!(C_TID, u32,);
            let ecx = rd32(lf_checker_rt::relocated(if t1 == x { ECX_EQ } else { ECX_ELSE }));
            (edi as i32 as f32, ecx as i32 as f32)
        };
        let prod = mul(f1, f0);
        // Strictly greater, NaN false: the original's comiss + seta.
        let wide = prod > rdf(lf_checker_rt::relocated(WIDE_THRESH));

        let aspect: f32 =
            lf_checker_rt::callee_thiscall!(C_ASPECT, f32, lf_checker_rt::relocated(ASPECT_OBJ), 1);
        let tol = rdf(lf_checker_rt::relocated(TOL));

        // First passing stage wins; each check is |aspect - C| < tol with the
        // original's not-less-than fall-through (NaN falls through).
        let mut stage: u32 = 5;
        let c1 = rdf(lf_checker_rt::relocated(C_S1));
        if !(fabs_stage(sub(aspect, c1)) < tol) {
            let c2 = rdf(lf_checker_rt::relocated(C_S2));
            if !(fabs_stage(sub(aspect, c2)) < tol) || !wide {
                let c3 = rdf(lf_checker_rt::relocated(C_S3));
                if !(fabs_stage(sub(aspect, c3)) < tol) {
                    let c4 = rdf(lf_checker_rt::relocated(C_S4));
                    if !(fabs_stage(sub(aspect, c4)) < tol) {
                        let c5 = rdf(lf_checker_rt::relocated(C_S5));
                        if !(fabs_stage(sub(aspect, c5)) < tol) {
                            stage = 5;
                        } else {
                            stage = 4;
                        }
                    } else {
                        stage = 3;
                    }
                } else {
                    stage = 2;
                }
            } else {
                stage = 1;
            }
        } else {
            stage = 0;
        }

        // Dispatch on selector minus one, UNSIGNED; anything above 4 defaults.
        let case = sel.wrapping_sub(1);
        if case > 4 {
            wr32(out1, 0);
            wr32(out1.wrapping_add(4), 0);
            wr32(out2, 1.0f32.to_bits());
            wr32(out2.wrapping_add(4), 1.0f32.to_bits());
            return 0;
        }
        let (ia, ib) = CASES[stage as usize][case as usize];
        let mut buf_a = [0u32; 2];
        let mut buf_b = [0u32; 2];
        let pa = (&mut buf_a as *mut u32) as u32;
        let pb = (&mut buf_b as *mut u32) as u32;
        let ra = lf_checker_rt::callee_cdecl!(C_GETTER, u32, pa, ia);
        wr32(out1, rd32(ra));
        wr32(out1.wrapping_add(4), rd32(ra.wrapping_add(4)));
        let rb = lf_checker_rt::callee_cdecl!(C_GETTER, u32, pb, ib);
        wr32(out2, rd32(rb));
        wr32(out2.wrapping_add(4), rd32(rb.wrapping_add(4)));
    }
    0
});
