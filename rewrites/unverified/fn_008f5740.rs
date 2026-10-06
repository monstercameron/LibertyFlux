// original: 0x008f5740 scancode_text_entry (proposed)

/// Turn a scancode string into output words through the keyboard layout.
///
/// `obj` is the input object (a 256-byte key-state array at `+0x88`, a flags
/// byte at `+0x118` whose bit 0 keeps numpad keys unmapped), `strp` a
/// NUL-terminated scancode string, `out` a word buffer receiving one word per
/// translated key plus a terminating zero word. Each scancode maps to a
/// virtual key (or appends a word directly, or is skipped); the key is mapped
/// through the keyboard layout, translated to characters, and appended, with
/// dead-key state kept in the `G_DEAD_CHAR` global and the layout cached in
/// `G_LAYOUT_CACHE` after the first use. Two classification bytes select an
/// alternate mapping path that skips the layout cache.
///
/// The keyboard-type query at entry is dead: the original compares the answer
/// against 7 and stores the flag, but the only reload lands in a register
/// the next instructions overwrite, so the answer affects nothing. The call
/// itself still happens. The per-scancode switch tests the case-table index,
/// a fixed nonzero byte for the four special scancodes, so their zero sides
/// never run. Counts returned by the translator are compared SIGNED (a
/// negative count means a dead key); the wrong version compares them
/// unsigned. Returns the input pointer left at the terminator.
///
/// Thiscall: object in ECX, two stack words, callee cleans up.
const OBJ_KEYSTATE: u32 = 0x88;
const OBJ_PAD_FLAGS: u32 = 0x118;
const G_CLS0: u32 = 0x116c250;
const G_CLS3: u32 = 0x116c253;
const G_LAYOUT_CACHE: u32 = 0x117e6c8;
const G_DEAD_CHAR: u32 = 0x117e6d0;
const IAT_MAPVK: u32 = 0xe7341c;
const IAT_TOASCII: u32 = 0xe73418;
const IAT_FOLD: u32 = 0xe7329c;

const C_KBTYPE: u32 = 1;
const C_KBLAYOUT: u32 = 2;
const C_KBLNAME: u32 = 3;
const C_KLLOAD: u32 = 4;
const C_MAPVK: u32 = 5;
const C_TOASCII: u32 = 6;
const C_FOLD: u32 = 7;
const C_BYTEMAP: u32 = 8;
const C_COOKIE: u32 = 9;

#[inline(always)]
unsafe fn rd8(a: u32) -> u8 {
    unsafe { (a as *const u8).read() }
}

#[inline(always)]
unsafe fn wr16(a: u32, v: u16) {
    unsafe { (a as *mut u16).write_unaligned(v) }
}

enum T1Act {
    Vk(u32),
    Append(u16),
    AppendDi,
    Skip,
}

#[inline(always)]
fn t1(b: u8) -> T1Act {
    match b {
        0x02 => T1Act::Vk(0x31),
        0x03 => T1Act::Vk(0x32),
        0x04 => T1Act::Vk(0x33),
        0x05 => T1Act::Vk(0x34),
        0x06 => T1Act::Vk(0x35),
        0x07 => T1Act::Vk(0x36),
        0x08 => T1Act::Vk(0x37),
        0x09 => T1Act::Vk(0x38),
        0x0A => T1Act::Vk(0x39),
        0x0B => T1Act::Vk(0x30),
        0x0C => T1Act::Vk(0xBD),
        0x0D => T1Act::Vk(0xBB),
        0x10 => T1Act::Vk(0x51),
        0x11 => T1Act::Vk(0x57),
        0x12 => T1Act::Vk(0x45),
        0x13 => T1Act::Vk(0x52),
        0x14 => T1Act::Vk(0x54),
        0x15 => T1Act::Vk(0x59),
        0x16 => T1Act::Vk(0x55),
        0x17 => T1Act::Vk(0x49),
        0x18 => T1Act::Vk(0x4F),
        0x19 => T1Act::Vk(0x50),
        0x1A => T1Act::Vk(0xDD),
        0x1B => T1Act::Vk(0xDC),
        0x1E => T1Act::Vk(0x41),
        0x1F => T1Act::Vk(0x53),
        0x20 => T1Act::Vk(0x44),
        0x21 => T1Act::Vk(0x46),
        0x22 => T1Act::Vk(0x47),
        0x23 => T1Act::Vk(0x48),
        0x24 => T1Act::Vk(0x4A),
        0x25 => T1Act::Vk(0x4B),
        0x26 => T1Act::Vk(0x4C),
        0x27 => T1Act::Vk(0xBA),
        0x28 => T1Act::Vk(0xDE),
        0x29 => T1Act::Vk(0xC0),
        0x2C => T1Act::Vk(0x5A),
        0x2D => T1Act::Vk(0x58),
        0x2E => T1Act::Vk(0x43),
        0x2F => T1Act::Vk(0x56),
        0x30 => T1Act::Vk(0x42),
        0x31 => T1Act::Vk(0x4E),
        0x32 => T1Act::Vk(0x4D),
        0x33 => T1Act::Vk(0xBC),
        0x34 => T1Act::Vk(0xBE),
        0x35 => T1Act::Vk(0xBF),
        0x37 => T1Act::Vk(0x6A),
        0x39 => T1Act::Vk(0x20),
        0x47 => T1Act::Vk(0x67),
        0x48 => T1Act::Vk(0x68),
        0x49 => T1Act::Vk(0x69),
        0x4A => T1Act::Vk(0x6D),
        0x4B => T1Act::Vk(0x64),
        0x4C => T1Act::Vk(0x65),
        0x4D => T1Act::Vk(0x66),
        0x4E => T1Act::Vk(0x6B),
        0x4F => T1Act::Vk(0x61),
        0x50 => T1Act::Vk(0x62),
        0x51 => T1Act::Vk(0x63),
        0x52 => T1Act::Vk(0x60),
        0x53 => T1Act::Vk(0x6E),
        0x56 => T1Act::Vk(0xE2),
        0x7D => T1Act::Vk(0xE2),
        0xB5 => T1Act::Vk(0x6F),
        0x2B => T1Act::AppendDi,
        0x90 => T1Act::Append(0x5e),
        0x91 => T1Act::Append(0x40),
        0x92 => T1Act::Append(0x3a),
        0x93 => T1Act::Append(0x5f),
        _ => T1Act::Skip,
    }
}

/// Doubled character (or dead-key candidate) to dead-character word.
/// Five values map; the tables share them.
#[inline(always)]
fn dead_word(c: u16) -> Option<u16> {
    match c {
        0x5E => Some(0x302),
        0x60 => Some(0x300),
        0xA8 => Some(0x308),
        0xB4 => Some(0x301),
        0xB8 => Some(0x327),
        _ => None,
    }
}

#[inline(always)]
fn is_numpad(vk: u32) -> bool {
    (0x60..=0x69).contains(&vk) || matches!(vk, 0x6a | 0x6b | 0x6d | 0x6f | 0x6e)
}

lf_checker_rt::export!(thiscall, rw_008f5740(obj: u32, strp: u32, out: u32) -> u32 {
    unsafe {
        use lf_checker_rt as rt;
        rt::callee_stdcall!(C_KBTYPE, u32, 0u32);
        let g_cls0 = rt::global::<u8>(G_CLS0);
        let g_cls3 = rt::global::<u8>(G_CLS3);
        let g_cache = rt::global::<u32>(G_LAYOUT_CACHE);
        let g_dead = rt::global::<u16>(G_DEAD_CHAR);
        let mapvk_slot = rt::relocated(IAT_MAPVK) as *const u32;
        let toascii_slot = rt::relocated(IAT_TOASCII) as *const u32;
        let fold_slot = rt::relocated(IAT_FOLD) as *const u32;
        let mut p = strp;
        let mut esi = out;
        if rd8(p) == 0 {
            wr16(esi, 0);
            rt::callee_cdecl!(C_COOKIE, u32);
            return p;
        }
        let mut ebx: u32 = 0;
        let mut ebp: u32 = mapvk_slot.read();
        let mut edi: u32 = 0x5c;
        loop {
            let run_body = match t1(rd8(p)) {
                T1Act::Skip => {
                    ebx = 0;
                    false
                }
                T1Act::Append(w) => {
                    wr16(esi, w);
                    esi += 2;
                    ebx != 0
                }
                T1Act::AppendDi => {
                    wr16(esi, edi as u16);
                    esi += 2;
                    ebx != 0
                }
                T1Act::Vk(v) => {
                    ebx = v;
                    true
                }
            };
            if run_body {
                'body: {
                    edi = ebx;
                    let layout = rt::callee_stdcall!(C_KBLAYOUT, u32, 0u32);
                    let eq19 = ((layout & 0x3ff) == 0x19) as u8;
                    let use_alt = g_cls0.read() == 0x6a || g_cls3.read() != 0;
                    if !use_alt {
                        let mut cached = g_cache.read();
                        if cached == 0 {
                            let mut namebuf = [0u8; 16];
                            rt::callee_stdcall!(C_KBLNAME, u32, namebuf.as_mut_ptr() as u32);
                            let klid: [u8; 16] = [
                                0x30, 0, 0x30, 0, 0x30, 0, 0x30, 0, 0x34, 0, 0x30, 0, 0x39, 0, 0x30,
                                0,
                            ];
                            cached = rt::callee_stdcall!(C_KLLOAD, u32, klid.as_ptr() as u32, 2u32);
                            g_cache.write(cached);
                        }
                        let via_reg: extern "stdcall" fn(u32, u32, u32) -> u32 =
                            core::mem::transmute(ebp as usize);
                        let a1 = via_reg(ebx, 0, cached);
                        let via_slot: extern "stdcall" fn(u32, u32, u32) -> u32 =
                            core::mem::transmute(mapvk_slot.read() as usize);
                        let a2 = via_slot(a1, 1, layout);
                        ebx = a2;
                        ebp = a1;
                    } else {
                        let via_reg: extern "stdcall" fn(u32, u32, u32) -> u32 =
                            core::mem::transmute(ebp as usize);
                        ebp = via_reg(ebx, 0, layout);
                    }
                    if rd8(obj + OBJ_PAD_FLAGS) & 1 != 0 && is_numpad(edi) {
                        ebx = edi;
                    }
                    if ebp == 0 || ebx == 0 {
                        break 'body;
                    }
                    let mut chbuf = [0u32; 1];
                    let toascii: extern "stdcall" fn(u32, u32, u32, u32, u32, u32) -> u32 =
                        core::mem::transmute(toascii_slot.read() as usize);
                    let cnt = toascii(ebx, ebp, obj + OBJ_KEYSTATE, chbuf.as_mut_ptr() as u32, 0, layout);
                    let mut dx = g_dead.read();
                    if dx != 0 && cnt == 1 {
                        let src = [chbuf[0] as u16, dx, 0u16];
                        let mut dst = [0u16; 3];
                        let fold: extern "stdcall" fn(u32, u32, u32, u32, u32) -> u32 =
                            core::mem::transmute(fold_slot.read() as usize);
                        let fr = fold(0x20, src.as_ptr() as u32, 3, dst.as_mut_ptr() as u32, 3);
                        if (fr as i32) > 0 {
                            chbuf[0] = dst[0] as u32;
                        }
                        dx = 0;
                        g_dead.write(0);
                    } else if (cnt as i32) <= 0 {
                        if !use_alt && (cnt as i32) < 0 {
                            let cx = chbuf[0] as u16;
                            if (cx as u32).wrapping_sub(0x5e) <= 0x5a {
                                match dead_word(cx) {
                                    Some(v) => g_dead.write(v),
                                    None => g_dead.write(cx),
                                }
                            } else {
                                g_dead.write(cx);
                            }
                        }
                        break 'body;
                    }
                    let mut direct_append = false;
                    if !use_alt && (cnt as i32) > 1 {
                        let w = chbuf[0];
                        if (w as u8) == ((w >> 8) as u8) {
                            let lo = w as u8;
                            if (lo as u32).wrapping_sub(0x5e) <= 0x5a {
                                match dead_word(lo as u16) {
                                    Some(v) => {
                                        g_dead.write(v);
                                        break 'body;
                                    }
                                    None => {
                                        g_dead.write(0);
                                        direct_append = true;
                                    }
                                }
                            } else {
                                g_dead.write(0);
                                direct_append = true;
                            }
                        }
                    }
                    if !direct_append && dx != 0 {
                        break 'body;
                    }
                    let mut ax = chbuf[0] as u16;
                    if (cnt as i32) > 1 {
                        ax >>= 8;
                    }
                    if ax == 0x7e {
                        (esi as *mut u32).write_unaligned(0x7e007e);
                        esi += 4;
                    } else {
                        wr16(esi, ax);
                        esi += 2;
                        if eq19 != 0 {
                            let at = esi - 2;
                            rt::callee_stdcall!(C_BYTEMAP, u32, rd8(at) as u32, at, eq19 as u32);
                        }
                    }
                }
                ebp = mapvk_slot.read();
                edi = 0x5c;
            }
            p += 1;
            wr16(esi, 0);
            if rd8(p) == 0 {
                break;
            }
        }
        rt::callee_cdecl!(C_COOKIE, u32);
        p
    }
});
