// original: 0x0091d2e0 input_name_to_code (proposed) — STAGE 1 of 2

/// Map an input name to its code. STAGE 1: entry, name chain, 30 constant
/// arms. The call arms, numeric-prefix path and no-match tail are stage 2
/// (taking one returns a loud wrong constant, never a silent pass).
///
/// `wstr` points to wide characters terminated by 0x7e or 0x807e (a null
/// pointer returns `NONE`). The low byte of each word is copied into a
/// 40-byte buffer; input with no terminator in the first 40 words returns
/// `NONE` without matching. `out`, when non-null, is initialised (dwords at
/// `+0`..`+0xc` to `NONE`, dword at `+0x110` and words at `+0x10`, `+0x50`,
/// `+0x90`, `+0xd0` to zero). A second copy of the name is truncated after
/// 5 characters by an unconditional zero store; the first comparison uses
/// that copy, the rest use the full buffer.
///
/// The chain compares the buffer against 68 names in order through the
/// scripted comparer (one stub id per site, so each trial takes the arm its
/// scripts select). Four constant arms are `bl`-gated (0 means the second
/// constant); `bl` is set when the pad object reports present, the layout
/// override byte is clear and the helper answers zero. Every path ends by
/// checking the stack cookie, which preserves registers.
///
/// Names and the cookie are image constants relocated at load; the override
/// byte is writable game data. The abort past the length check is dead (the
/// length guard always fires first) and is not implemented. Cdecl, three
/// stack words, result in `eax`.
lf_checker_rt::export!(cdecl, rw_0091d2e0(wstr: u32, _arg1: u32, out: u32) -> u32 {
    unsafe {
        const NONE: u32 = 0xFF;
        const COOKIE: u32 = 0x01057FB4;
        const GLOB: u32 = 0x01160C3D;
        const TERM_A: u16 = 0x7E;
        const TERM_B: u16 = 0x807E;
        const MAXLEN: usize = 0x28;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
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
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn cookie(ck: u32) {
            unsafe { lf_checker_rt::callee_thiscall!(3, u32, ck) };
        }

        // (strcmp stub id, name file VA, action) in chain order.
        // Action: 0 = stage-1 todo (wrong constant), 1 = const C1,
        // 2 = bl-gated (bl==0 -> C2 else C1).
        const TABLE: [(u32, u32, u8, u32, u32); 68] = [
            (100, 0x00E85F1C, 0, 0, 0), // site 0: stage 2+
            (101, 0x00E84D3C, 2, 0x100, 0x129), // site 1
            (102, 0x00E84D54, 2, 0x101, 0x12A), // site 2
            (103, 0x00E84D94, 2, 0x102, 0x12B), // site 3
            (104, 0x00E84DC4, 2, 0x103, 0x12C), // site 4
            (105, 0x00E84DF0, 1, 0x11C, 0x0), // site 5
            (106, 0x00E84E10, 1, 0x11D, 0x0), // site 6
            (107, 0x00E84E38, 0, 0, 0), // site 7: stage 2+
            (108, 0x00E84E98, 0, 0, 0), // site 8: stage 2+
            (109, 0x00E84F00, 0, 0, 0), // site 9: stage 2+
            (110, 0x00E84F78, 0, 0, 0), // site 10: stage 2+
            (111, 0x00E84FC8, 0, 0, 0), // site 11: stage 2+
            (112, 0x00E8501C, 1, 0x121, 0x0), // site 12
            (113, 0x00E85044, 0, 0, 0), // site 13: stage 2+
            (114, 0x00E8508C, 1, 0x123, 0x0), // site 14
            (115, 0x00E85094, 0, 0, 0), // site 15: stage 2+
            (116, 0x00E850B4, 0, 0, 0), // site 16: stage 2+
            (117, 0x00E850D8, 0, 0, 0), // site 17: stage 2+
            (118, 0x00E850FC, 0, 0, 0), // site 18: stage 2+
            (119, 0x00E85124, 1, 0x108, 0x0), // site 19
            (120, 0x00E85134, 0, 0, 0), // site 20: stage 2+
            (121, 0x00E85218, 0, 0, 0), // site 21: stage 2+
            (122, 0x00E852C4, 0, 0, 0), // site 22: stage 2+
            (123, 0x00E85364, 0, 0, 0), // site 23: stage 2+
            (124, 0x00E853BC, 0, 0, 0), // site 24: stage 2+
            (125, 0x00E85400, 0, 0, 0), // site 25: stage 2+
            (126, 0x00E85444, 0, 0, 0), // site 26: stage 2+
            (127, 0x00E854C0, 0, 0, 0), // site 27: stage 2+
            (128, 0x00E85618, 0, 0, 0), // site 28: stage 2+
            (129, 0x00E85858, 0, 0, 0), // site 29: stage 2+
            (130, 0x00E85904, 0, 0, 0), // site 30: stage 2+
            (131, 0x00E859B4, 0, 0, 0), // site 31: stage 2+
            (132, 0x00E85A2C, 0, 0, 0), // site 32: stage 2+
            (133, 0x00E85A94, 1, 0x114, 0x0), // site 33
            (134, 0x00E85AA4, 1, 0x115, 0x0), // site 34
            (135, 0x00E85AB8, 1, 0x116, 0x0), // site 35
            (136, 0x00E85AD0, 1, 0x117, 0x0), // site 36
            (137, 0x00E85AE4, 0, 0, 0), // site 37: stage 2+
            (138, 0x00E85B04, 0, 0, 0), // site 38: stage 2+
            (139, 0x00E85B38, 0, 0, 0), // site 39: stage 2+
            (140, 0x00E85B50, 0, 0, 0), // site 40: stage 2+
            (141, 0x00E85B74, 0, 0, 0), // site 41: stage 2+
            (142, 0x00E85B9C, 0, 0, 0), // site 42: stage 2+
            (143, 0x00E85BBC, 0, 0, 0), // site 43: stage 2+
            (144, 0x00E85BDC, 0, 0, 0), // site 44: stage 2+
            (145, 0x00E85BF0, 0, 0, 0), // site 45: stage 2+
            (146, 0x00E85C04, 0, 0, 0), // site 46: stage 2+
            (147, 0x00E85C1C, 0, 0, 0), // site 47: stage 2+
            (148, 0x00E85C30, 0, 0, 0), // site 48: stage 2+
            (149, 0x00E85C50, 0, 0, 0), // site 49: stage 2+
            (150, 0x00E85C70, 0, 0, 0), // site 50: stage 2+
            (151, 0x00E85C98, 1, 0x128, 0x0), // site 51
            (152, 0x00E85CAC, 1, 0x12D, 0x0), // site 52
            (153, 0x00E85CB8, 1, 0x12E, 0x0), // site 53
            (154, 0x00E85CC4, 1, 0x12F, 0x0), // site 54
            (155, 0x00E85CD0, 1, 0x130, 0x0), // site 55
            (156, 0x00E85CDC, 1, 0x131, 0x0), // site 56
            (157, 0x00E85CE8, 1, 0x132, 0x0), // site 57
            (158, 0x00E85CF4, 1, 0x133, 0x0), // site 58
            (159, 0x00E85D00, 1, 0x134, 0x0), // site 59
            (160, 0x00E85D0C, 1, 0x135, 0x0), // site 60
            (161, 0x00E85D18, 1, 0x136, 0x0), // site 61
            (162, 0x00E85D24, 1, 0x137, 0x0), // site 62
            (163, 0x00E85D30, 1, 0x138, 0x0), // site 63
            (164, 0x00E85D3C, 1, 0x139, 0x0), // site 64
            (165, 0x00E85D48, 1, 0x13A, 0x0), // site 65
            (166, 0x00E85D54, 1, 0x13B, 0x0), // site 66
            (167, 0x00E85D60, 1, 0x13C, 0x0), // site 67
        ];

        let ck = rd32(lf_checker_rt::relocated(COOKIE));
        if wstr == 0 {
            cookie(ck);
            return NONE;
        }
        if out != 0 {
            wr32(out + 0x110, 0);
            wr32(out, NONE);
            wr32(out + 4, NONE);
            wr32(out + 8, NONE);
            wr32(out + 0xC, NONE);
            wr16(out + 0x10, 0);
            wr16(out + 0x50, 0);
            wr16(out + 0x90, 0);
            wr16(out + 0xD0, 0);
        }
        let mut buf1 = [0u8; 40];
        let mut len = 0usize;
        let mut wi = 0u32;
        let overlong = loop {
            let w = rd16(wstr.wrapping_add(wi * 2));
            if w == TERM_A || w == TERM_B {
                break false;
            }
            buf1[len] = w as u8;
            len += 1;
            wi += 1;
            if len >= MAXLEN {
                break true;
            }
        };
        if overlong {
            cookie(ck);
            return NONE;
        }
        buf1[len] = 0;
        let mut buf2 = [0u8; 44];
        buf2[..len + 1].copy_from_slice(&buf1[..len + 1]);
        buf2[5] = 0;
        // Site 0 compares the truncated copy and runs before bl is known.
        let eq0: u32 = lf_checker_rt::callee_cdecl!(
            TABLE[0].0,
            u32,
            buf2.as_ptr() as u32,
            lf_checker_rt::relocated(TABLE[0].1)
        );
        if eq0 == 0 {
            cookie(ck);
            return 0xDEAD0000 | TABLE[0].0;
        }
        // Short-circuit like the original: the helper runs only when the pad
        // byte is set and the override byte is clear.
        let pad: u32 = lf_checker_rt::callee_cdecl!(1, u32, 0u32);
        let mut bl = false;
        if rd8(pad.wrapping_add(0x328C)) != 0 && rd8(lf_checker_rt::relocated(GLOB)) == 0 {
            let s59: u32 = lf_checker_rt::callee_cdecl!(2, u32,);
            bl = (s59 & 0xFF) == 0;
        }
        let mut ans: Option<u32> = None;
        for &(id, strva, act, c1, c2) in TABLE[1..].iter() {
            let eq: u32 = lf_checker_rt::callee_cdecl!(
                id,
                u32,
                buf1.as_ptr() as u32,
                lf_checker_rt::relocated(strva)
            );
            if eq == 0 {
                ans = Some(match act {
                    1 => c1,
                    2 => {
                        if bl {
                            c1
                        } else {
                            c2
                        }
                    }
                    _ => 0xDEAD0000 | id,
                });
                break;
            }
        }
        cookie(ck);
        ans.unwrap_or(0xDEAD00EE)
    }
});
