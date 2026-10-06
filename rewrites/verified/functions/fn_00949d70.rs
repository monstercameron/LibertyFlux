// original: 0x00949d70 process_table_rows (proposed)

/// Drive one processing pass over the rows of a global record table.
///
/// The table lives in 15204-byte blocks selected by the counter global: the
/// row count is the dword at `COUNT_BASE + counter * STRIDE`, and row `i` of
/// that block holds its fields at `ROW_BASE + counter * STRIDE + i * ROW_STRIDE`.
/// Each row carries four floats (`F0..F3`), an angle, an id dword, a flag byte
/// and a kind selector. For every row whose id equals `arg0` and whose flag
/// bit 0 equals the low byte of `arg1`, the kind selector (bits 1..4 of the
/// flag byte, minus one) dispatches to one of nine handlers that shuffle the
/// row floats through a scratch frame, run two cosine/sine helpers over the
/// angle on two of the paths, and report the frame words to a family of
/// callee helpers. One handler publishes its floats to a shared output row
/// and raises a sticky flag that unlocks an extra setup sequence on later
/// rows. The return value is the answer of the final reporting callee.
///
/// Counts and the dispatch index are compared UNSIGNED (`jbe`/`jb`/`ja`); the
/// two callee answers that steer branches are tested for zero only. Float
/// arithmetic is SSE scalar in the original's operand order (pinned with
/// `black_box`); every float equality test is "not equal" including NaN.
///
/// Original: 0x00949d70 (cdecl, two stack words; only the low byte of the
/// second is read).
lf_checker_rt::export!(cdecl, rw_00949d70(arg0: u32, arg1: u32) -> u32 {
    unsafe {
        // Global layout (file VAs).
        const COUNTER: u32 = 0x1174794;
        const STRIDE: u32 = 0x3b64; // 15204 bytes per counter block
        const COUNT_BASE: u32 = 0x11ea720;
        const ROW_BASE: u32 = 0x11e6bc0;
        const ROW_STRIDE: u32 = 0x4c;
        const STICKY_FLAG: u32 = 0x11e61cb;
        const TLS_SLOT_IDX: u32 = 0x17aba14;
        const HOOK_THIS: u32 = 0x128e94c;
        // Row field offsets from ROW_BASE.
        const F_F0: u32 = 0x04; // float 0
        const F_F1: u32 = 0x08; // float 1
        const F_F2: u32 = 0x0c; // float 2
        const F_F3: u32 = 0x10; // float 3
        const F_ANGLE: u32 = 0x14; // cos/sin angle
        const F_ID: u32 = 0x18; // id dword matched against arg0
        const F_BDC: u32 = 0x1c; // dword passed to the fixup helper
        const F_BE0: u32 = 0x20; // dword passed to reporting helpers
        const F_BE3: u32 = 0x23; // byte shifted into a report word
        const F_BE4: u32 = 0x24; // string for the 12- and 8-arg reporters
        const F_BEE: u32 = 0x2e; // second string for those reporters
        const F_BC0: u32 = 0x00; // dword passed to the one-arg helper
        const F_BF8: u32 = 0x38; // signed byte for those reporters
        const F_BFA: u32 = 0x3a; // flag/kind byte
        const F_BFC: u32 = 0x3c; // float tested against 0
        const F_C00: u32 = 0x40; // float tested against 0
        const F_C04: u32 = 0x44; // float tested against 1
        const F_C08: u32 = 0x48; // float tested against 1
        const HALF: f32 = 0.5; // bits 0x3f000000, as the original's constant
        const ONE: f32 = 1.0; // bits 0x3f800000, as the original's constant
        // Callee ids (see the contract).
        const C_NOP0: u32 = 1; // 0x432be0 cdecl/0
        const C_FIXUP: u32 = 2; // 0x8fc260 cdecl/4, writes one float via arg1
        const C_PAIR: u32 = 3; // 0x432c20 cdecl/2
        const C_ONE: u32 = 4; // 0x432b70 cdecl/1
        const C_NOP1: u32 = 5; // 0x8d3830 cdecl/0
        const C_REP5A: u32 = 6; // 0x8d41d0 cdecl/5, five frame pointers
        const C_REP5B: u32 = 7; // 0x8d41d0 cdecl/5, four globals + frame
        const C_REP5C: u32 = 8; // 0x8d41d0 cdecl/5, four frames + global
        const C_NOP2: u32 = 9; // 0x8d47a0 cdecl/0
        const C_REP12: u32 = 10; // 0x8e2c30 cdecl/12
        const C_COS: u32 = 11; // cos helper, float in/out of xmm0
        const C_SIN: u32 = 12; // sin helper, float in/out of xmm0
        const C_HOOKQ: u32 = 13; // 0x9bb6e0 thiscall/0, byte answer used
        const C_HOOKF: u32 = 14; // 0x9bb760 thiscall/0
        const C_REP9: u32 = 15; // 0x8d4040 cdecl/9
        const C_REP8: u32 = 16; // 0x8e27d0 cdecl/8
        const C_ALLOC: u32 = 17; // 0x8dc3a0 cdecl/2, nullness tested
        const C_BUILD: u32 = 18; // 0x8dabe0 thiscall/4
        const C_FIN: u32 = 19; // 0x499e30 cdecl/1
        const C_REP5D: u32 = 20; // 0x8e1f90 cdecl/5, all by value
        const C_DONE: u32 = 21; // 0xac7320 cdecl/1, its answer is returned
        const C_COOKIE: u32 = 22; // security-cookie check (preserves regs)

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
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
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
        /// The original's `ucomiss` + `lahf` + `(an instruction of the original)` + `jp` idiom:
        /// taken exactly when the operands compare not-equal, unordered
        /// (NaN) included. Rust `!=` has the same truth table.
        #[inline(always)]
        fn fne(a: f32, b: f32) -> bool {
            core::hint::black_box(a) != core::hint::black_box(b)
        }

        let g = |va: u32| lf_checker_rt::relocated(va);
        // Scratch frame: the original's S+0x00..S+0x1af as 108 words.
        let mut st = [0u32; 0x1b0 / 4];
        let stp = |st: &mut [u32; 0x1b0 / 4], off: usize| -> u32 {
            (st.as_mut_ptr() as u32).wrapping_add(off as u32)
        };
        let arg1b = (arg1 & 0xff) as u8;

        let counter = rd32(g(COUNTER));
        let blk = counter.wrapping_mul(STRIDE);
        if rd32(g(COUNT_BASE).wrapping_add(blk)) > 0 {
            lf_checker_rt::callee_cdecl!(C_NOP0, u32,);
        }
        let mut ebx: u32 = 0;
        let mut esi: u32 = 0;
        // Latched loop: re-reads the counter and count every pass.
        loop {
            let counter = rd32(g(COUNTER));
            let blk = counter.wrapping_mul(STRIDE);
            let count = rd32(g(COUNT_BASE).wrapping_add(blk));
            if ebx >= count {
                break;
            }
            let row = g(ROW_BASE).wrapping_add(blk).wrapping_add(esi);
            // Per-row float loads.
            st[0x04 / 4] = rd32(row.wrapping_add(F_F0));
            st[0x08 / 4] = rd32(row.wrapping_add(F_F1));
            st[0x10 / 4] = rd32(row.wrapping_add(F_F2));
            st[0x14 / 4] = rd32(row.wrapping_add(F_F3));
            let bdc = rd32(row.wrapping_add(F_BDC));
            lf_checker_rt::callee_cdecl!(C_FIXUP, u32, bdc, stp(&mut st, 0x04), 0, 0);
            lf_checker_rt::callee_cdecl!(C_FIXUP, u32, bdc, stp(&mut st, 0x10), 0, 0);
            lf_checker_rt::callee_cdecl!(C_NOP0, u32,);
            lf_checker_rt::callee_cdecl!(C_PAIR, u32, 6, 0);
            // Row gate: id match (full dword) and flag-bit match (low bit).
            if arg0 != rd32(row.wrapping_add(F_ID)) {
                ebx = ebx.wrapping_add(1);
                esi = esi.wrapping_add(ROW_STRIDE);
                continue;
            }
            let flag = rd8(row.wrapping_add(F_BFA));
            if (flag & 1) != arg1b {
                ebx = ebx.wrapping_add(1);
                esi = esi.wrapping_add(ROW_STRIDE);
                continue;
            }
            if (flag & 0x20) != 0 && rd8(g(STICKY_FLAG)) != 0 {
                lf_checker_rt::callee_cdecl!(C_PAIR, u32, 2, 5);
                lf_checker_rt::callee_cdecl!(C_PAIR, u32, 0xf, 8);
                st[0xb0 / 4] = 0x3f800000;
                st[0x170 / 4] = st[0xb0 / 4];
                st[0xb4 / 4] = 0x3f800000;
                st[0x174 / 4] = st[0xb4 / 4];
                st[0x150 / 4] = 0x3f800000;
                st[0x178 / 4] = st[0x150 / 4];
                st[0x154 / 4] = 0;
                st[0x17c / 4] = st[0x154 / 4];
                st[0x40 / 4] = 0;
                st[0x180 / 4] = st[0x40 / 4];
                st[0x44 / 4] = 0x3f800000;
                st[0x184 / 4] = st[0x44 / 4];
                st[0xe0 / 4] = 0;
                st[0x188 / 4] = st[0xe0 / 4];
                st[0xe4 / 4] = 0;
                st[0x18c / 4] = st[0xe4 / 4];
                lf_checker_rt::callee_cdecl!(C_ONE, u32, 0);
                lf_checker_rt::callee_cdecl!(C_NOP1, u32,);
                st[0x18 / 4] = (rd8(row.wrapping_add(F_BE3)) as u32) << 24;
                lf_checker_rt::callee_cdecl!(C_REP5A, u32, stp(&mut st, 0x170), stp(&mut st, 0x178), stp(&mut st, 0x180), stp(&mut st, 0x188), stp(&mut st, 0x18));
                lf_checker_rt::callee_cdecl!(C_PAIR, u32, 2, 8);
                lf_checker_rt::callee_cdecl!(C_PAIR, u32, 0xf, 8);
                st[0x18 / 4] = (rd8(row.wrapping_add(F_BE3)) as u32) << 24;
                lf_checker_rt::callee_cdecl!(C_REP5B, u32, g(0x11ee28c), g(0x11ee294), g(0x11ee29c), g(0x11ee2a4), stp(&mut st, 0x18));
                lf_checker_rt::callee_cdecl!(C_PAIR, u32, 2, 6);
                lf_checker_rt::callee_cdecl!(C_PAIR, u32, 0xf, 0xf);
                lf_checker_rt::callee_cdecl!(C_NOP2, u32,);
            }
            // Dispatch on bits 1..4 of the flag byte, minus one.
            let sw = rd8(row.wrapping_add(F_BFA));
            let mut sw_out = sw;
            let idx = (((sw as u32) >> 1) & 0xf).wrapping_sub(1);
            if idx <= 8 {
                match idx {
                    // Kinds 0 and 1 share the 12-word report tail; kind 1
                    // passes a null second string.
                    0 | 1 => {
                        let sbyte = rd8(row.wrapping_add(F_BF8)) as i8 as i32 as u32;
                        let p7 = if idx == 0 { row.wrapping_add(F_BEE) } else { 0 };
                        lf_checker_rt::callee_cdecl!(C_REP12, u32, st[0x04 / 4], st[0x08 / 4], st[0x10 / 4], row.wrapping_add(F_BE4), p7, sbyte, 0, 0xffffffff, 0, 0xffffffff, 0, 0);
                    }
                    // Kind 2: 8-word report with one float difference.
                    2 => {
                        let sbyte = rd8(row.wrapping_add(F_BF8)) as i8 as i32 as u32;
                        let byte = rd8(row.wrapping_add(F_BE3)) as u32;
                        let d = sub(f32::from_bits(st[0x10 / 4]), f32::from_bits(st[0x04 / 4]));
                        lf_checker_rt::callee_cdecl!(C_REP8, u32, st[0x04 / 4], st[0x08 / 4], d.to_bits(), byte, byte, row.wrapping_add(F_BE4), row.wrapping_add(F_BEE), sbyte);
                    }
                    // Kind 3: thread-state branch; the slot index comes from
                    // a global and the pointed-to flag picks the report.
                    3 => {
                        let d0 = sub(f32::from_bits(st[0x10 / 4]), f32::from_bits(st[0x04 / 4]));
                        let d1 = sub(f32::from_bits(st[0x14 / 4]), f32::from_bits(st[0x08 / 4]));
                        let slot = rd32(g(TLS_SLOT_IDX));
                        let tls = lf_checker_rt::tls_slot(slot as usize);
                        st[0x20 / 4] = d0.to_bits();
                        st[0x1c / 4] = d1.to_bits();
                        if rd32(tls.wrapping_add(0x8cc)) == 0 {
                            lf_checker_rt::callee_cdecl!(C_REP5D, u32, st[0x04 / 4], st[0x08 / 4], d0.to_bits(), d1.to_bits(), rd32(row.wrapping_add(F_BE0)));
                        } else {
                            st[0x168 / 4] = st[0x04 / 4];
                            st[0x16c / 4] = st[0x08 / 4];
                            let ans: u32 = lf_checker_rt::callee_cdecl!(C_ALLOC, u32, 0x1c, 0);
                            if ans == 0 {
                                lf_checker_rt::callee_cdecl!(C_FIN, u32, 0);
                            } else {
                                let ans2: u32 = lf_checker_rt::callee_thiscall!(C_BUILD, u32, ans, stp(&mut st, 0x168), st[0x20 / 4], st[0x1c / 4], rd32(row.wrapping_add(F_BE0)));
                                lf_checker_rt::callee_cdecl!(C_FIN, u32, ans2);
                            }
                        }
                    }
                    // Kind 4: copy the row floats into the report slots.
                    4 => {
                        st[0x50 / 4] = st[0x04 / 4];
                        st[0x170 / 4] = st[0x50 / 4];
                        st[0x54 / 4] = st[0x14 / 4];
                        st[0x174 / 4] = st[0x54 / 4];
                        st[0x130 / 4] = st[0x04 / 4];
                        st[0x178 / 4] = st[0x130 / 4];
                        st[0x134 / 4] = st[0x08 / 4];
                        st[0x17c / 4] = st[0x134 / 4];
                        st[0x60 / 4] = st[0x10 / 4];
                        st[0x180 / 4] = st[0x60 / 4];
                        st[0x64 / 4] = st[0x14 / 4];
                        st[0x184 / 4] = st[0x64 / 4];
                        st[0xf0 / 4] = st[0x10 / 4];
                        st[0xf4 / 4] = st[0x08 / 4];
                        st[0x188 / 4] = st[0xf0 / 4];
                        st[0x18c / 4] = st[0xf4 / 4];
                        lf_checker_rt::callee_cdecl!(C_ONE, u32, 0);
                        lf_checker_rt::callee_cdecl!(C_NOP1, u32,);
                        lf_checker_rt::callee_cdecl!(C_REP5C, u32, stp(&mut st, 0x170), stp(&mut st, 0x178), stp(&mut st, 0x180), stp(&mut st, 0x188), row.wrapping_add(F_BE0));
                        lf_checker_rt::callee_cdecl!(C_NOP2, u32,);
                    }
                    // Kinds 5 and 8: angle-driven kernel or plain copy.
                    5 | 8 => {
                        let angle = f32::from_bits(rd32(row.wrapping_add(F_ANGLE)));
                        let tail: u32;
                        if fne(angle, 0.0) {
                            // Kernel A: halve the summed pairs, rotate by the
                            // cosine/sine answers, then store through one of
                            // two layouts picked by bits 1..4 of the flag.
                            let x2 = f32::from_bits(st[0x04 / 4]);
                            let mut x3 = f32::from_bits(st[0x14 / 4]);
                            x3 = add(x3, f32::from_bits(st[0x08 / 4]));
                            let mut x0 = x2;
                            x0 = add(x0, f32::from_bits(st[0x10 / 4]));
                            x3 = mul(x3, HALF);
                            x0 = mul(x0, HALF);
                            st[0x0c / 4] = x3.to_bits();
                            st[0x18 / 4] = x0.to_bits();
                            x0 = sub(x0, x2);
                            st[0x20 / 4] = x0.to_bits();
                            x0 = x3;
                            x0 = sub(x0, f32::from_bits(st[0x08 / 4]));
                            st[0x1c / 4] = x0.to_bits();
                            let cos = f32::from_bits(lf_checker_rt::callee_cdecl!(C_COS, u32, angle.to_bits()));
                            st[0x24 / 4] = cos.to_bits();
                            let sin = f32::from_bits(lf_checker_rt::callee_cdecl!(C_SIN, u32, angle.to_bits()));
                            let mut x6 = cos;
                            x3 = x6;
                            x3 = mul(x3, f32::from_bits(st[0x20 / 4]));
                            x6 = mul(x6, f32::from_bits(st[0x1c / 4]));
                            let mut x2b = f32::from_bits(st[0x18 / 4]);
                            x2b = sub(x2b, x3);
                            x3 = add(x3, f32::from_bits(st[0x18 / 4]));
                            let mut x4 = sin;
                            let mut x5 = x4;
                            x5 = mul(x5, f32::from_bits(st[0x20 / 4]));
                            x4 = mul(x4, f32::from_bits(st[0x1c / 4]));
                            let mut x1 = f32::from_bits(st[0x0c / 4]);
                            x0 = x2b;
                            if (sw & 0x1e) == 0x12 {
                                x0 = sub(x0, x5);
                                x1 = sub(x1, x4);
                                x4 = add(x4, f32::from_bits(st[0x0c / 4]));
                                x2b = add(x2b, x5);
                                st[0x28 / 4] = x0.to_bits();
                                st[0x170 / 4] = st[0x28 / 4];
                                x0 = x1;
                                x0 = add(x0, x6);
                                st[0x2c / 4] = x0.to_bits();
                                st[0x174 / 4] = st[0x2c / 4];
                                x1 = sub(x1, x6);
                                st[0x158 / 4] = x2b.to_bits();
                                st[0x178 / 4] = st[0x158 / 4];
                                st[0x15c / 4] = x1.to_bits();
                                st[0x17c / 4] = st[0x15c / 4];
                                x0 = x3;
                                x0 = sub(x0, x5);
                                st[0xc0 / 4] = x0.to_bits();
                                st[0x180 / 4] = st[0xc0 / 4];
                                x0 = x4;
                                x0 = add(x0, x6);
                                x3 = add(x3, x5);
                                st[0xc4 / 4] = x0.to_bits();
                                st[0x184 / 4] = st[0xc4 / 4];
                                x4 = sub(x4, x6);
                                st[0x120 / 4] = x3.to_bits();
                                st[0x188 / 4] = st[0x120 / 4];
                                st[0x124 / 4] = x4.to_bits();
                                tail = st[0x124 / 4];
                            } else {
                                x0 = sub(x0, x4);
                                x1 = sub(x1, x5);
                                x5 = add(x5, f32::from_bits(st[0x0c / 4]));
                                x2b = add(x2b, x4);
                                st[0xd0 / 4] = x0.to_bits();
                                st[0x170 / 4] = st[0xd0 / 4];
                                x0 = x1;
                                x0 = add(x0, x6);
                                st[0xd4 / 4] = x0.to_bits();
                                st[0x174 / 4] = st[0xd4 / 4];
                                x1 = sub(x1, x6);
                                st[0x30 / 4] = x2b.to_bits();
                                st[0x178 / 4] = st[0x30 / 4];
                                st[0x34 / 4] = x1.to_bits();
                                st[0x17c / 4] = st[0x34 / 4];
                                x0 = x3;
                                x0 = sub(x0, x4);
                                st[0x38 / 4] = x0.to_bits();
                                st[0x180 / 4] = st[0x38 / 4];
                                x0 = x5;
                                x0 = add(x0, x6);
                                x3 = add(x3, x4);
                                st[0x3c / 4] = x0.to_bits();
                                st[0x184 / 4] = st[0x3c / 4];
                                x5 = sub(x5, x6);
                                st[0x48 / 4] = x3.to_bits();
                                st[0x188 / 4] = st[0x48 / 4];
                                st[0x4c / 4] = x5.to_bits();
                                tail = st[0x4c / 4];
                            }
                        } else {
                            st[0x90 / 4] = st[0x04 / 4];
                            st[0x170 / 4] = st[0x90 / 4];
                            st[0x94 / 4] = st[0x14 / 4];
                            st[0x174 / 4] = st[0x94 / 4];
                            st[0x140 / 4] = st[0x04 / 4];
                            st[0x178 / 4] = st[0x140 / 4];
                            st[0x144 / 4] = st[0x08 / 4];
                            st[0x17c / 4] = st[0x144 / 4];
                            st[0xa0 / 4] = st[0x10 / 4];
                            st[0x180 / 4] = st[0xa0 / 4];
                            st[0xa4 / 4] = st[0x14 / 4];
                            st[0x184 / 4] = st[0xa4 / 4];
                            st[0x110 / 4] = st[0x10 / 4];
                            st[0x114 / 4] = st[0x08 / 4];
                            st[0x188 / 4] = st[0x110 / 4];
                            tail = st[0x114 / 4];
                        }
                        st[0x18c / 4] = tail;
                        sw_out = 0;
                        if (sw & 0x40) != 0 {
                            sw_out = lf_checker_rt::callee_thiscall!(C_HOOKQ, u32, g(HOOK_THIS)) as u8;
                        } else {
                            lf_checker_rt::callee_cdecl!(C_ONE, u32, rd32(row.wrapping_add(F_BC0)));
                        }
                        lf_checker_rt::callee_cdecl!(C_REP5C, u32, stp(&mut st, 0x170), stp(&mut st, 0x178), stp(&mut st, 0x180), stp(&mut st, 0x188), row.wrapping_add(F_BE0));
                        if (sw & 0x40) == 0 {
                            // Latch via the loop tail.
                        } else if sw_out != 0 {
                            lf_checker_rt::callee_thiscall!(C_HOOKF, u32, g(HOOK_THIS));
                        }
                    }
                    // Kind 6: constant or copied report words, then a second
                    // angle-driven kernel with a nine-word report.
                    6 => {
                        let f_bfc = f32::from_bits(rd32(row.wrapping_add(F_BFC)));
                        let mut eax: u32;
                        if fne(f_bfc, 0.0)
                            || fne(f32::from_bits(rd32(row.wrapping_add(F_C00))), 0.0)
                            || fne(f32::from_bits(rd32(row.wrapping_add(F_C04))), ONE)
                            || fne(f32::from_bits(rd32(row.wrapping_add(F_C08))), ONE)
                        {
                            let x2 = f32::from_bits(rd32(row.wrapping_add(F_C08)));
                            let x0 = f32::from_bits(rd32(row.wrapping_add(F_C04)));
                            st[0x98 / 4] = f_bfc.to_bits();
                            st[0x190 / 4] = st[0x98 / 4];
                            st[0x9c / 4] = x2.to_bits();
                            st[0x194 / 4] = st[0x9c / 4];
                            st[0xa8 / 4] = f_bfc.to_bits();
                            let x1 = f32::from_bits(rd32(row.wrapping_add(F_C00)));
                            st[0x198 / 4] = st[0xa8 / 4];
                            st[0xac / 4] = x1.to_bits();
                            st[0x19c / 4] = st[0xac / 4];
                            st[0xb8 / 4] = x0.to_bits();
                            st[0x1a0 / 4] = st[0xb8 / 4];
                            st[0xbc / 4] = x2.to_bits();
                            st[0x1a4 / 4] = st[0xbc / 4];
                            st[0xc8 / 4] = x0.to_bits();
                            st[0xcc / 4] = x1.to_bits();
                            st[0x1a8 / 4] = st[0xc8 / 4];
                            eax = st[0xcc / 4];
                        } else {
                            st[0x58 / 4] = 0x3ba3d70a;
                            st[0x190 / 4] = st[0x58 / 4];
                            st[0x5c / 4] = 0x3f7eb852;
                            st[0x194 / 4] = st[0x5c / 4];
                            st[0x68 / 4] = 0x3ba3d70a;
                            st[0x198 / 4] = st[0x68 / 4];
                            st[0x6c / 4] = 0x3ba3d70a;
                            st[0x19c / 4] = st[0x6c / 4];
                            st[0x78 / 4] = 0x3f7eb852;
                            st[0x1a0 / 4] = st[0x78 / 4];
                            st[0x7c / 4] = 0x3f7eb852;
                            st[0x1a4 / 4] = st[0x7c / 4];
                            st[0x88 / 4] = 0x3f7eb852;
                            st[0x8c / 4] = 0x3ba3d70a;
                            st[0x1a8 / 4] = st[0x88 / 4];
                            eax = st[0x8c / 4];
                        }
                        st[0x1ac / 4] = eax;
                        let angle = f32::from_bits(rd32(row.wrapping_add(F_ANGLE)));
                        let tail: u32;
                        if fne(angle, 0.0) {
                            // Kernel B: same halved-pair prologue as kernel A,
                            // a differently wired rotation, one store layout.
                            let x2 = f32::from_bits(st[0x04 / 4]);
                            let mut x3 = f32::from_bits(st[0x14 / 4]);
                            x3 = add(x3, f32::from_bits(st[0x08 / 4]));
                            let mut x0 = x2;
                            x0 = add(x0, f32::from_bits(st[0x10 / 4]));
                            x3 = mul(x3, HALF);
                            x0 = mul(x0, HALF);
                            st[0x18 / 4] = x3.to_bits();
                            st[0x24 / 4] = x0.to_bits();
                            x0 = sub(x0, x2);
                            st[0x1c / 4] = x0.to_bits();
                            x0 = x3;
                            x0 = sub(x0, f32::from_bits(st[0x08 / 4]));
                            st[0x20 / 4] = x0.to_bits();
                            let cos = f32::from_bits(lf_checker_rt::callee_cdecl!(C_COS, u32, angle.to_bits()));
                            st[0x0c / 4] = cos.to_bits();
                            let sin = f32::from_bits(lf_checker_rt::callee_cdecl!(C_SIN, u32, angle.to_bits()));
                            let mut x2b = f32::from_bits(st[0x24 / 4]);
                            let x7 = f32::from_bits(st[0x18 / 4]);
                            let mut x5 = sin;
                            x0 = cos;
                            x3 = x0;
                            x0 = mul(x0, f32::from_bits(st[0x20 / 4]));
                            x3 = mul(x3, f32::from_bits(st[0x1c / 4]));
                            let mut x4 = x5;
                            x4 = mul(x4, f32::from_bits(st[0x1c / 4]));
                            x5 = mul(x5, f32::from_bits(st[0x20 / 4]));
                            x2b = sub(x2b, x3);
                            x3 = add(x3, f32::from_bits(st[0x24 / 4]));
                            st[0x0c / 4] = x0.to_bits();
                            let mut x1 = x7;
                            x1 = sub(x1, x4);
                            x0 = x2b;
                            x0 = sub(x0, x5);
                            x2b = add(x2b, x5);
                            x4 = add(x4, x7);
                            st[0x118 / 4] = x0.to_bits();
                            st[0x170 / 4] = st[0x118 / 4];
                            st[0x128 / 4] = x2b.to_bits();
                            x2b = f32::from_bits(st[0x0c / 4]);
                            x0 = x1;
                            x0 = add(x0, f32::from_bits(st[0x0c / 4]));
                            x1 = sub(x1, x2b);
                            st[0x11c / 4] = x0.to_bits();
                            st[0x174 / 4] = st[0x11c / 4];
                            x0 = x3;
                            x0 = sub(x0, x5);
                            st[0x178 / 4] = st[0x128 / 4];
                            st[0x12c / 4] = x1.to_bits();
                            st[0x17c / 4] = st[0x12c / 4];
                            st[0x138 / 4] = x0.to_bits();
                            st[0x180 / 4] = st[0x138 / 4];
                            x0 = x4;
                            x0 = add(x0, x2b);
                            x3 = add(x3, x5);
                            st[0x13c / 4] = x0.to_bits();
                            st[0x184 / 4] = st[0x13c / 4];
                            x4 = sub(x4, x2b);
                            st[0x148 / 4] = x3.to_bits();
                            st[0x188 / 4] = st[0x148 / 4];
                            st[0x14c / 4] = x4.to_bits();
                            tail = st[0x14c / 4];
                        } else {
                            st[0xd8 / 4] = st[0x04 / 4];
                            st[0x170 / 4] = st[0xd8 / 4];
                            st[0xdc / 4] = st[0x14 / 4];
                            st[0x174 / 4] = st[0xdc / 4];
                            st[0xe8 / 4] = st[0x04 / 4];
                            st[0x178 / 4] = st[0xe8 / 4];
                            st[0xec / 4] = st[0x08 / 4];
                            st[0x17c / 4] = st[0xec / 4];
                            st[0xf8 / 4] = st[0x10 / 4];
                            st[0x180 / 4] = st[0xf8 / 4];
                            st[0xfc / 4] = st[0x14 / 4];
                            st[0x184 / 4] = st[0xfc / 4];
                            st[0x108 / 4] = st[0x10 / 4];
                            st[0x10c / 4] = st[0x08 / 4];
                            st[0x188 / 4] = st[0x108 / 4];
                            tail = st[0x10c / 4];
                        }
                        st[0x18c / 4] = tail;
                        let mut flag13: u8 = 0;
                        if (sw & 0x40) != 0 {
                            flag13 = lf_checker_rt::callee_thiscall!(C_HOOKQ, u32, g(HOOK_THIS)) as u8;
                        } else {
                            lf_checker_rt::callee_cdecl!(C_ONE, u32, rd32(row.wrapping_add(F_BC0)));
                        }
                        lf_checker_rt::callee_cdecl!(C_REP9, u32, stp(&mut st, 0x170), stp(&mut st, 0x178), stp(&mut st, 0x180), stp(&mut st, 0x188), stp(&mut st, 0x190), stp(&mut st, 0x198), stp(&mut st, 0x1a0), stp(&mut st, 0x1a8), row.wrapping_add(F_BE0));
                        // The original falls through to a shared conditional
                        // jump; the effect is the join below.
                        if (sw & 0x40) == 0 {
                            // Latch via the loop tail.
                        } else if flag13 != 0 {
                            lf_checker_rt::callee_thiscall!(C_HOOKF, u32, g(HOOK_THIS));
                        }
                    }
                    // Kind 7: publish the row floats and raise the sticky flag.
                    7 => {
                        st[0x70 / 4] = st[0x04 / 4];
                        wr32(g(0x11ee28c), st[0x70 / 4]);
                        st[0x74 / 4] = st[0x14 / 4];
                        wr32(g(0x11ee290), st[0x74 / 4]);
                        st[0x160 / 4] = st[0x04 / 4];
                        wr32(g(0x11ee294), st[0x160 / 4]);
                        st[0x164 / 4] = st[0x08 / 4];
                        wr32(g(0x11ee298), st[0x164 / 4]);
                        st[0x80 / 4] = st[0x10 / 4];
                        wr32(g(0x11ee29c), st[0x80 / 4]);
                        st[0x84 / 4] = st[0x14 / 4];
                        wr32(g(0x11ee2a0), st[0x84 / 4]);
                        st[0x100 / 4] = st[0x10 / 4];
                        st[0x104 / 4] = st[0x08 / 4];
                        wr32(g(0x11ee2a4), st[0x100 / 4]);
                        wr32(g(0x11ee2a8), st[0x104 / 4]);
                        wr8(g(STICKY_FLAG), 1);
                    }
                    _ => {}
                }
            }
            ebx = ebx.wrapping_add(1);
            esi = esi.wrapping_add(ROW_STRIDE);
        }
        let ans: u32 = lf_checker_rt::callee_cdecl!(C_DONE, u32, 0x3f800000);
        lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
        ans
    }
});
