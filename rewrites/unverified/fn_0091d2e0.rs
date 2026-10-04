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

// __TABLE__

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
        let pad: u32 = lf_checker_rt::callee_cdecl!(1, u32, 0u32);
        let present = rd8(pad.wrapping_add(0x328C)) != 0;
        let g = rd8(lf_checker_rt::relocated(GLOB));
        let s59: u32 = lf_checker_rt::callee_cdecl!(2, u32,);
        let bl = present && g == 0 && (s59 & 0xFF) == 0;
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
