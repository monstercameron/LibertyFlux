// original: 0x0091AAB0 kbd_scancode_to_char (proposed)

/// Translate a PC AT scancode into a virtual-key code and a character.
///
/// Cdecl of two words: `scancode` (1-based key number) and `out`, a caller
/// buffer for up to two wide characters plus a terminator, or null. Returns
/// a virtual-key code, a fallback character, or an extended-key id.
///
/// The function first asks the OS for the current layout and the keyboard
/// type: `layout_us` is whether `GetKeyboardLayout(0)` equals 0x04090409
/// (US English in both words) and `type_jp` is whether `GetKeyboardType(0)`
/// equals 7. A jump table on `scancode - 1` (bound compared UNSIGNED: values
/// 0 and above 220 fall through with both results zero) then yields a
/// display character `ch` and a virtual-key code `vk` per key: digits,
/// letters, punctuation, each with its own pair; four extended keys return
/// 0x129-0x12c at once unless the game is in kana mode (a global holding
/// 0x45) or an input flag is set; three bracket keys pick their code from
/// `type_jp`; comma and period consult a helper and `layout_us` to choose
/// between the plain and shifted glyph. Thirty-six codes yield `ch` 1.
/// `vk` zero means "no key": with a null buffer the function returns `ch`;
/// otherwise it continues to the tail below.
///
/// Translation (only when `vk` is nonzero and `out` is non-null) zero-fills
/// a 256-byte key-state area, reads the layout handle, and, outside
/// Japanese mode (mode byte 0x6a or the layout flag set) with an empty
/// layout cache, registers the "00000409" layout through
/// `GetKeyboardLayoutNameA` and `LoadKeyboardLayoutA` and caches the
/// handle. It maps `vk` through `MapVirtualKeyExA` twice on the cached
/// path (code-to-key then key-to-code, the second with map type 1) and
/// once on the Japanese path, then converts through `ToAsciiEx` or
/// `ToUnicodeEx` (buffer length 2) into a small character buffer. A zero
/// from any step ends translation with status 0.
/// The returned count is compared SIGNED (`cmp/jle` against 1): a helper
/// decides the character width, the character is reduced to 8 or 16 bits
/// accordingly, and 0x7e takes a dedicated path storing the 0x7e007e pair.
/// Otherwise the one or two characters are stored to `out` with a zero
/// terminator and two helpers post-process the buffer.
///
/// The tail returns `ch` when nothing was produced, `vk` when translation
/// succeeded, and runs a formatting helper over `out` when translation
/// failed but a character exists, unless the game state selects the
/// angle-bracket path (only for `ch` 0x3c/0x3e with a non-US layout),
/// which stores `ch` alone. All other comparisons are equality or
/// zero-ness tests; the only other ordered comparison is the unsigned
/// switch bound.
///
/// Original: 0x0091AAB0 (cdecl, two stack words). All OS and helper calls
/// are intercepted and scripted by the checker; frame buffers are mirrored
/// in one zeroed array whose overlaps match the original's layout.
lf_checker_rt::export!(cdecl, rw_0091AAB0(scancode: u32, out: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn g8(va: u32) -> u8 {
            unsafe { lf_checker_rt::global::<u8>(va).read() }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read() }
        }
        #[inline(always)]
        unsafe fn iat1(slot: u32, a0: u32) -> u32 {
            unsafe {
                let f: extern "stdcall" fn(u32) -> u32 =
                    core::mem::transmute(lf_checker_rt::global::<u32>(slot).read() as usize);
                f(a0)
            }
        }
        #[inline(always)]
        unsafe fn iat2(slot: u32, a0: u32, a1: u32) -> u32 {
            unsafe {
                let f: extern "stdcall" fn(u32, u32) -> u32 =
                    core::mem::transmute(lf_checker_rt::global::<u32>(slot).read() as usize);
                f(a0, a1)
            }
        }
        #[inline(always)]
        unsafe fn iat3(slot: u32, a0: u32, a1: u32, a2: u32) -> u32 {
            unsafe {
                let f: extern "stdcall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(lf_checker_rt::global::<u32>(slot).read() as usize);
                f(a0, a1, a2)
            }
        }
        #[inline(always)]
        unsafe fn iat6(slot: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
            unsafe {
                let f: extern "stdcall" fn(u32, u32, u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(lf_checker_rt::global::<u32>(slot).read() as usize);
                f(a0, a1, a2, a3, a4, a5)
            }
        }
        #[inline(always)]
        unsafe fn iat7(
            slot: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32,
        ) -> u32 {
            unsafe {
                let f: extern "stdcall" fn(u32, u32, u32, u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(lf_checker_rt::global::<u32>(slot).read() as usize);
                f(a0, a1, a2, a3, a4, a5, a6)
            }
        }

        const IAT_LAYOUT: u32 = 0x00E7340C;
        const IAT_TYPE: u32 = 0x00E73414;
        const IAT_LAYOUT_NAME: u32 = 0x00E73408;
        const IAT_LOAD_LAYOUT: u32 = 0x00E73404;
        const IAT_MAP_VK: u32 = 0x00E7341C;
        const IAT_TO_ASCII: u32 = 0x00E73418;
        const IAT_TO_UNICODE: u32 = 0x00E73420;

        const C_HELPER: u32 = 8;
        const C_MEMSET: u32 = 9;
        const C_WIDTH: u32 = 10;
        const C_NARROW: u32 = 11;
        const C_POST: u32 = 12;
        const C_POST_ALT: u32 = 13;
        const C_FORMAT: u32 = 14;
        const C_ENCODE: u32 = 15;
        const C_EMIT: u32 = 16;
        const C_COOKIE: u32 = 17;

        const G_KANA: u32 = 0x01160C40;
        const G_INPUT_FLAG: u32 = 0x016334C0;
        const G_MODE: u32 = 0x0116C250;
        const G_LAYOUT_FLAG: u32 = 0x0116C253;
        const G_LAYOUT_CACHE: u32 = 0x0117E6C8;
        const G_STATE: u32 = 0x011F7060;
        const G_CMP_A: u32 = 0x012088B4;
        const G_CMP_B: u32 = 0x00F1C040;
        const G_SUBSTATE: u32 = 0x01037720;

        const US_LAYOUT: u32 = 0x04090409;
        const JP_TYPE: u32 = 7;
        const MODE_JP: u8 = 0x6a;
        const MODE_RU: u8 = 0x72;
        const KANA_ON: u32 = 0x45;
        const FMT: u32 = 0x00E85E7C;
        const NARROW_OBJ: u32 = 0x0118D110;
        const WIDTH_OBJ: u32 = 0x0116BFF0;
        const KEYSTATE_LEN: u32 = 0x100;
        const UNICODE_LEN: u32 = 2;

        // One zeroed mirror of the original's frame area around its
        // character buffer: char words at [0..4], layout words at [2..6],
        // the name buffer at [7..], the key-state buffer at [15..].
        // The whole body runs in a closure so the cookie-check stub call
        // below fires on every return path, exactly like the original's
        // epilogue (the stub preserves registers and only logs the call).
        let result = (|| -> u32 {
        let mut frame = [0u32; 24];
        let char_buf = frame.as_mut_ptr() as u32;
        let layout_buf = unsafe { char_buf.wrapping_add(8) };
        let name_buf = unsafe { char_buf.wrapping_add(28) };
        let keystate = unsafe { char_buf.wrapping_add(60) };

        let layout = iat1(IAT_LAYOUT, 0);
        let layout_us = layout == US_LAYOUT;
        let kb_type = iat1(IAT_TYPE, 0);
        let type_jp = kb_type == JP_TYPE;

        // Scancode switch. `ch` is the display character, `vk` the key code.
        let mut ch: u32 = 0;
        let mut vk: u32 = 0;
        match scancode {
            // Extended keys: return at once unless kana mode or input flag.
            200 => {
                if g32(G_KANA) != KANA_ON && g8(G_INPUT_FLAG) == 0 {
                    return 0x129;
                }
                ch = 1;
            }
            208 => {
                if g32(G_KANA) != KANA_ON && g8(G_INPUT_FLAG) == 0 {
                    return 0x12a;
                }
                ch = 1;
            }
            203 => {
                if g32(G_KANA) != KANA_ON && g8(G_INPUT_FLAG) == 0 {
                    return 0x12b;
                }
                ch = 1;
            }
            205 => {
                if g32(G_KANA) != KANA_ON && g8(G_INPUT_FLAG) == 0 {
                    return 0x12c;
                }
                ch = 1;
            }
            // Character only (no key code).
            57 => ch = 0x20,
            79 => ch = 0x31,
            80 => ch = 0x32,
            81 => ch = 0x33,
            75 => ch = 0x34,
            76 => ch = 0x35,
            77 => ch = 0x36,
            71 => ch = 0x37,
            72 => ch = 0x38,
            73 => ch = 0x39,
            82 => ch = 0x30,
            74 => ch = 0x2d,
            83 => ch = 0x2e,
            181 => ch = 0x2f,
            55 => ch = 0x2a,
            78 => ch = 0x2b,
            145 => ch = 0x40,
            146 => ch = 0x3a,
            147 => ch = 0x5f,
            // Digit and letter rows: character and key code agree.
            2 => {
                ch = 0x31;
                vk = ch;
            }
            3 => {
                ch = 0x32;
                vk = ch;
            }
            4 => {
                ch = 0x33;
                vk = ch;
            }
            5 => {
                ch = 0x34;
                vk = ch;
            }
            6 => {
                ch = 0x35;
                vk = ch;
            }
            7 => {
                ch = 0x36;
                vk = ch;
            }
            8 => {
                ch = 0x37;
                vk = ch;
            }
            9 => {
                ch = 0x38;
                vk = ch;
            }
            10 => {
                ch = 0x39;
                vk = ch;
            }
            11 => {
                ch = 0x30;
                vk = ch;
            }
            16 => {
                ch = 0x51;
                vk = ch;
            }
            17 => {
                ch = 0x57;
                vk = ch;
            }
            18 => {
                ch = 0x45;
                vk = ch;
            }
            19 => {
                ch = 0x52;
                vk = ch;
            }
            20 => {
                ch = 0x54;
                vk = ch;
            }
            21 => {
                ch = 0x59;
                vk = ch;
            }
            22 => {
                ch = 0x55;
                vk = ch;
            }
            23 => {
                ch = 0x49;
                vk = ch;
            }
            24 => {
                ch = 0x4f;
                vk = ch;
            }
            25 => {
                ch = 0x50;
                vk = ch;
            }
            30 => {
                ch = 0x41;
                vk = ch;
            }
            31 => {
                ch = 0x53;
                vk = ch;
            }
            32 => {
                ch = 0x44;
                vk = ch;
            }
            33 => {
                ch = 0x46;
                vk = ch;
            }
            34 => {
                ch = 0x47;
                vk = ch;
            }
            35 => {
                ch = 0x48;
                vk = ch;
            }
            36 => {
                ch = 0x4a;
                vk = ch;
            }
            37 => {
                ch = 0x4b;
                vk = ch;
            }
            38 => {
                ch = 0x4c;
                vk = ch;
            }
            44 => {
                ch = 0x5a;
                vk = ch;
            }
            45 => {
                ch = 0x58;
                vk = ch;
            }
            46 => {
                ch = 0x43;
                vk = ch;
            }
            47 => {
                ch = 0x56;
                vk = ch;
            }
            48 => {
                ch = 0x42;
                vk = ch;
            }
            49 => {
                ch = 0x4e;
                vk = ch;
            }
            50 => {
                ch = 0x4d;
                vk = ch;
            }
            // Split pairs.
            12 => {
                vk = 0xbd;
                ch = 0x2d;
            }
            40 => {
                vk = 0xde;
                ch = 0x27;
            }
            53 => {
                vk = 0xbf;
                ch = 0x2f;
            }
            86 => {
                vk = 0xe2;
                ch = 0x5c;
            }
            // Character derived from the key code.
            13 => {
                vk = 0xbb;
                ch = vk.wrapping_sub(0x7e);
            }
            39 => {
                vk = 0xba;
                ch = vk.wrapping_sub(0x7f);
            }
            41 => {
                vk = 0xc0;
                ch = vk.wrapping_sub(0x60);
            }
            // Bracket keys depend on the keyboard type.
            26 => {
                let flag = u32::from(type_jp);
                ch = 0x5b;
                vk = flag.wrapping_mul(2).wrapping_add(0xdb);
            }
            27 => {
                let flag = u32::from(!type_jp);
                ch = 0x5d;
                vk = flag.wrapping_add(0xdc);
            }
            43 => {
                ch = 0x5c;
                if !type_jp {
                    vk = 0xdc;
                }
            }
            // Comma and period consult the helper and the layout.
            51 => {
                let ok: u32 = lf_checker_rt::callee_cdecl!(C_HELPER, u32,);
                if ok & 0xFF != 0 && layout_us {
                    ch = 0x3c;
                } else {
                    vk = 0xbc;
                    ch = 0x2c;
                }
            }
            52 => {
                let ok: u32 = lf_checker_rt::callee_cdecl!(C_HELPER, u32,);
                if ok & 0xFF != 0 && layout_us {
                    ch = 0x3e;
                } else {
                    vk = 0xbe;
                    ch = 0x2e;
                }
            }
            // Codes that yield character 1.
            1 | 14 | 15 | 28 | 29 | 42 | 54 | 56 | 58 | 59 | 60 | 61 | 62 | 63 | 64 | 65 | 66
            | 67 | 68 | 70 | 87 | 88 | 100 | 101 | 102 | 156 | 157 | 184 | 197 | 201 | 207
            | 209 | 210 | 211 | 219 | 220 => ch = 1,
            _ => {}
        }

        // No buffer, or no key: skip translation.
        if out == 0 {
            return ch;
        }
        let mode = g8(G_MODE);
        let jp_mode = mode == MODE_JP || g8(G_LAYOUT_FLAG) != 0;
        let mut status: u32 = 0;
        if vk != 0 {
            lf_checker_rt::callee_cdecl!(C_MEMSET, u32, keystate, 0u32, KEYSTATE_LEN);
            let hkl0 = iat1(IAT_LAYOUT, 0);
            frame[0] = hkl0;
            let vkey: u32;
            if !jp_mode {
                let mut hkl = g32(G_LAYOUT_CACHE);
                if hkl == 0 {
                    frame[6] &= 0xFFFF0000;
                    frame[2] = 0x00300030;
                    frame[3] = 0x00300030;
                    frame[4] = 0x00340030;
                    frame[5] = 0x00390030;
                    iat1(IAT_LAYOUT_NAME, name_buf);
                    hkl = iat2(IAT_LOAD_LAYOUT, layout_buf, 2);
                    lf_checker_rt::global::<u32>(G_LAYOUT_CACHE).write(hkl);
                }
                vkey = iat3(IAT_MAP_VK, vk, 0, hkl);
                frame[1] = vkey;
                let scode = iat3(IAT_MAP_VK, vkey, 1, hkl0);
                vk = scode;
            } else {
                vkey = iat3(IAT_MAP_VK, vk, 0, hkl0);
            }
            frame[1] = hkl0 & 0x3ff;
            if vkey != 0 && vk != 0 {
                frame[0] = 0;
                let count = if jp_mode {
                    iat7(IAT_TO_UNICODE, vk, vkey, keystate, char_buf, UNICODE_LEN, 0, hkl0)
                } else {
                    iat6(IAT_TO_ASCII, vk, vkey, keystate, char_buf, 0, hkl0)
                };
                if count != 0 {
                    let wide: u32 = lf_checker_rt::callee_thiscall!(
                        C_WIDTH, u32, lf_checker_rt::relocated(WIDTH_OBJ),
                    );
                    let mut c = frame[0];
                    // Signed comparison of the callee-returned count (jle).
                    if wide & 0xFF == 0 {
                        if (count as i32) > 1 {
                            c >>= 8;
                        }
                    } else if (count as i32) > 1 {
                        c >>= 16;
                    } else {
                        c &= 0xffff;
                    }
                    frame[0] = c;
                    if c == 0x7e {
                        wr32(out, 0x7e007e);
                        wr16(out.wrapping_add(count.wrapping_mul(2)), 0);
                        status = 1;
                    } else {
                        if wide & 0xFF == 0 {
                            if mode == MODE_RU {
                                lf_checker_rt::callee_thiscall!(
                                    C_NARROW, u32, lf_checker_rt::relocated(NARROW_OBJ), c,
                                    char_buf, 1u32,
                                );
                                c = frame[0];
                            }
                            wr16(out, (c & 0xFF) as u16);
                            wr16(out.wrapping_add(2), ((c >> 8) & 0xFF) as u16);
                            wr16(out.wrapping_add(4), 0);
                        } else {
                            wr16(out.wrapping_add(2), (frame[1] >> 16) as u16);
                            wr16(out, (c & 0xFFFF) as u16);
                            wr16(out.wrapping_add(count.wrapping_mul(2)), 0);
                        }
                        let w0 = rd16(out) as u32;
                        let t: u32 =
                            lf_checker_rt::callee_cdecl!(C_POST, u32, frame[1], w0);
                        if t & 0xFF == 0 {
                            lf_checker_rt::callee_cdecl!(C_POST_ALT, u32, out, out);
                        }
                        status = 1;
                    }
                }
            }
        }

        // Tail: pick the result.
        if ch == 0 {
            if status == 0 {
                return 0;
            }
            return if vk != 0 { vk } else { 0 };
        }
        if status != 0 {
            return if vk != 0 { vk } else { ch };
        }
        let use_brackets = if g32(G_STATE) == 1 {
            true
        } else if g32(G_CMP_A) != g32(G_CMP_B) {
            true
        } else {
            g32(G_SUBSTATE) == 0x12
        };
        if use_brackets && (ch == 0x3c || ch == 0x3e) && layout_us {
            wr16(out, ch as u16);
            wr16(out.wrapping_add(2), 0);
            return ch;
        }
        lf_checker_rt::callee_cdecl!(C_FORMAT, u32, name_buf, lf_checker_rt::relocated(FMT), scancode);
        let encoded: u32 = lf_checker_rt::callee_thiscall!(
            C_ENCODE, u32, lf_checker_rt::relocated(WIDTH_OBJ), name_buf,
        );
        lf_checker_rt::callee_cdecl!(C_EMIT, u32, out, encoded, 0x40u32);
        ch
        })();
        lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
        result
    }
});
