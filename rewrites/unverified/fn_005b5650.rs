// original: 0x005B5650 hud_marker_emit (proposed)

/// Emit one marker quad into the vertex arena after a chain of state gates.
///
/// Gate chain (any failure skips to the cookie check and returns): a signed
/// counter must exceed 1; a two-word query (callee 0) seeds a handle that must
/// later equal 0x4b; two polled flags (callees 1, 2, zero arguments) must answer
/// zero; a mode word must differ from 10; an optional object (null skips the
/// check, otherwise a flag word must be set or the word at +0x50 must lie
/// outside 2..=3, SIGNED) must pass; a state word must equal 7; a sub-mode word
/// must be 2 or 3.
///
/// Past the gates, two indexed reads (callee 3) and a four-word quad call
/// (callee 4) fill four frame floats; the quad's stub writes are the values the
/// computation reads. From inputs (a, b, c, d) it forms t1 = a - c, t5 = c + a,
/// t2 = d + b and t4 = b - d in exactly this instruction order. A third indexed
/// read (callee 3) returns a pointer whose float is converted with truncate
/// toward zero (cvttss2si semantics, including 0x80000000 for NaN, infinities
/// and out-of-range values); when a flag byte is set, a one-word query (callee
/// 5, thiscall) replaces the result's low byte.
///
/// TLS slot 0 points at a small array whose first word points at a block; a
/// nonzero word at +0x8cc of that block takes the handle path (a two-word
/// lookup, callee 6; null finishes through callee 8 with 0, otherwise a
/// three-word thiscall, callee 7, whose answer goes to callee 8), while zero
/// takes the emit path: a prepare thiscall (callee 10), a two-word call
/// (callee 11), then four guarded vertex stores. Each store runs only when both
/// a flag global and the arena cursor are nonzero; it appends a 0x24-byte
/// vertex (two computed floats, zeros, -1.0, an (alpha << 24) | white colour,
/// and two per-vertex 0/1 constants), advances the cursor global and bumps a
/// count global. A trailing call (callee 12) runs when the final cursor is
/// nonzero, two globals are cleared, and a close call (callee 13) runs. Every
/// exit path, taken or not, ends in the cookie check (callee 9, thiscall,
/// argument is always the pure cookie word) and returns nothing (cdecl, 0 args).
lf_checker_rt::export!(cdecl, rw_005B5650() -> u32 {
    unsafe {
        const GATE_COUNT: u32 = 0x103008C;
        const COOKIE: u32 = 0x1057FB4;
        const GATE_MODE: u32 = 0x17F5EA4;
        const GATE_MODE_BAD: u32 = 0x0A;
        const GATE_PTR: u32 = 0x19888A4;
        const GATE_PTR_FLAG: u32 = 0x19888A0;
        const GATE_FIELD: u32 = 0x50;
        const GATE_LO: i32 = 2;
        const GATE_HI: i32 = 3;
        const GATE_STATE: u32 = 0x1160C40;
        const GATE_STATE_OK: u32 = 7;
        const HANDLE_WANT: u32 = 0x4B;
        const SUB_MODE: u32 = 0x1160D74;
        const FLAG_BYTE: u32 = 0x11616F8;
        const TABLE2: u32 = 0x19D2F30;
        const V_FLAG: u32 = 0x1B4F71C;
        const V_CURSOR: u32 = 0x1B4F724;
        const V_COUNT: u32 = 0x1B4F718;
        const TLS_BLOCK_WORD: u32 = 0x8CC;
        const VERTEX_LEN: u32 = 0x24;
        const NEG_ONE_BITS: u32 = 0xBF80_0000;
        const ONE_BITS: u32 = 0x3F80_0000;
        const WHITE_RGB: u32 = 0x00FF_FFFF;
        const C_QUERY: u32 = 0;
        const C_POLL_A: u32 = 1;
        const C_POLL_B: u32 = 2;
        const C_INDEXED: u32 = 3;
        const C_QUAD: u32 = 4;
        const C_FLAGFN: u32 = 5;
        const C_HANDLE: u32 = 6;
        const C_APPLY: u32 = 7;
        const C_FINISH: u32 = 8;
        const C_COOKIE: u32 = 9;
        const C_PREP: u32 = 10;
        const C_PAIR: u32 = 11;
        const C_TAIL: u32 = 12;
        const C_CLOSE: u32 = 13;

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
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        /// Truncate a float toward zero exactly like cvttss2si (same
        /// instruction; out-of-range, NaN and infinities give 0x80000000).
        #[inline(always)]
        unsafe fn cvtt(fbits: u32) -> u32 {
            unsafe {
                let v = core::arch::x86::_mm_set_ss(f32::from_bits(fbits));
                core::arch::x86::_mm_cvttss_si32(v) as u32
            }
        }

        // Every break below lands on the shared cookie check, like the
        // original's jumps to its single exit block.
        'done: {
            if (rd32(lf_checker_rt::relocated(GATE_COUNT)) as i32) <= 1 {
                break 'done;
            }
            let handle = lf_checker_rt::callee_cdecl!(C_QUERY, u32, 0, 0xFFFF_FFFF);
            if (lf_checker_rt::callee_cdecl!(C_POLL_A, u32,) as u8) != 0 {
                break 'done;
            }
            if rd32(lf_checker_rt::relocated(GATE_MODE)) == GATE_MODE_BAD {
                break 'done;
            }
            let obj = rd32(lf_checker_rt::relocated(GATE_PTR));
            if obj != 0 && rd32(lf_checker_rt::relocated(GATE_PTR_FLAG)) == 0 {
                let field = rd32(obj.wrapping_add(GATE_FIELD)) as i32;
                // SIGNED range: values 2 and 3 exit, everything else passes.
                if field >= GATE_LO && field <= GATE_HI {
                    break 'done;
                }
            }
            if (lf_checker_rt::callee_cdecl!(C_POLL_B, u32,) as u8) != 0 {
                break 'done;
            }
            if rd32(lf_checker_rt::relocated(GATE_STATE)) != GATE_STATE_OK {
                break 'done;
            }
            if handle != HANDLE_WANT {
                break 'done;
            }
            let submode = rd32(lf_checker_rt::relocated(SUB_MODE));
            if submode != 2 && sub != 3 {
                break 'done;
            }

            // Quad floats: the stub writes them into our frame through the
            // two pointers, exactly as it does for the original's frame.
            let mut quad = [0u32; 4];
            let pq = (&mut quad as *mut u32) as u32;
            lf_checker_rt::callee_cdecl!(C_INDEXED, u32, pq, 0xA9);
            lf_checker_rt::callee_cdecl!(C_INDEXED, u32, pq.wrapping_add(8), 0xAA);
            lf_checker_rt::callee_cdecl!(C_QUAD, u32, 2, pq, pq.wrapping_add(8), 0);
            let a = f32::from_bits(quad[0]);
            let b = f32::from_bits(quad[1]);
            let c = f32::from_bits(quad[2]);
            let d = f32::from_bits(quad[3]);
            let t1 = sub(a, c);
            let t5 = add(c, a);
            let t2 = add(d, b);
            let t4 = sub(b, d);

            let mut frame_tmp = [0u32; 1];
            let pt = (&mut frame_tmp as *mut u32) as u32;
            let cvt_ptr = lf_checker_rt::callee_cdecl!(C_INDEXED, u32, pt, 0xAB);
            let mut col = cvtt(rd32(cvt_ptr));
            if rd8(lf_checker_rt::relocated(FLAG_BYTE)) != 0 {
                let r = lf_checker_rt::callee_thiscall!(
                    C_FLAGFN, u32,
                    lf_checker_rt::relocated(FLAG_BYTE)
                );
                col = (col & 0xFFFF_FF00) | (r & 0xFF);
            }
            let colour = ((col & 0xFF) << 24) | WHITE_RGB;

            let tls_block = rd32(lf_checker_rt::tls_slot(0));
            let alt = if submode != 2 { 1u32 } else { 0u32 };
            if rd32(tls_block.wrapping_add(TLS_BLOCK_WORD)) != 0 {
                let h = lf_checker_rt::callee_cdecl!(C_HANDLE, u32, 0x30, 0);
                if h == 0 {
                    lf_checker_rt::callee_cdecl!(C_FINISH, u32, 0);
                    break 'done;
                }
                let slot = rd32(lf_checker_rt::relocated(TABLE2).wrapping_add(alt * 4));
                let mut out_param = [0u32; 4];
                let po = (&mut out_param as *mut u32) as u32;
                let r = lf_checker_rt::callee_thiscall!(C_APPLY, u32, h, colour, slot, po);
                lf_checker_rt::callee_cdecl!(C_FINISH, u32, r);
                break 'done;
            }

            lf_checker_rt::callee_thiscall!(
                C_PREP, u32,
                lf_checker_rt::relocated(TABLE2).wrapping_add(alt * 4)
            );
            lf_checker_rt::callee_cdecl!(C_PAIR, u32, 4, 4);
            let flag = rd32(lf_checker_rt::relocated(V_FLAG));
            let mut cursor = rd32(lf_checker_rt::relocated(V_CURSOR));
            let mut count = rd32(lf_checker_rt::relocated(V_COUNT));
            // (fx, fy, word at +0x1c, word at +0x20) per vertex.
            let verts: [(f32, f32, u32, u32); 4] = [
                (t1, t4, 0, ONE_BITS),
                (t1, t4, 0, 0),
                (t5, t2, ONE_BITS, ONE_BITS),
                (t5, t4, ONE_BITS, 0),
            ];
            for (i, &(fx, fy, c1c, c20)) in verts.iter().enumerate() {
                let last = i == 3;
                if flag != 0 && cursor != 0 {
                    let vp = cursor;
                    cursor = cursor.wrapping_add(VERTEX_LEN);
                    wr32(lf_checker_rt::relocated(V_CURSOR), cursor);
                    count = count.wrapping_add(1);
                    wr32(lf_checker_rt::relocated(V_COUNT), count);
                    wr32(vp, fx.to_bits());
                    wr32(vp.wrapping_add(4), fy.to_bits());
                    wr32(vp.wrapping_add(8), 0);
                    wr32(vp.wrapping_add(0x0C), 0);
                    wr32(vp.wrapping_add(0x10), 0);
                    wr32(vp.wrapping_add(0x14), NEG_ONE_BITS);
                    wr32(vp.wrapping_add(0x18), colour);
                    wr32(vp.wrapping_add(0x1C), c1c);
                    wr32(vp.wrapping_add(0x20), c20);
                } else {
                    count = count.wrapping_add(1);
                    if last {
                        wr32(lf_checker_rt::relocated(V_COUNT), count);
                    }
                }
            }
            if cursor != 0 {
                lf_checker_rt::callee_cdecl!(C_TAIL, u32,);
                wr32(lf_checker_rt::relocated(V_CURSOR), 0);
            }
            wr32(lf_checker_rt::relocated(V_FLAG), 0);
            lf_checker_rt::callee_cdecl!(C_CLOSE, u32,);
        }
        // Every original path ends here with the pure cookie word: the stored
        // check value is cookie ^ entry_esp and esp at the xor equals entry
        // esp on all three code paths, so the xors cancel.
        let cookie = rd32(lf_checker_rt::relocated(COOKIE));
        lf_checker_rt::callee_thiscall!(C_COOKIE, u32, cookie);
    }
    0
});
