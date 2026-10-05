// original: 0x00d3baf0 CDummyTask_Stationary::vf3

/// Stationary dummy task operation dispatcher (vtable slot 3): six operations
/// selected by `op` on the task object `task` with a peer object `stream`.
///
/// Layout of `task` (all offsets hex): `+0` vtable (slot `+0x10` used by the
/// write path), `+0x10`/`+0x14` small integers, `+0x18` child object or null,
/// `+0x1c` integer, `+0x48` flag byte, `+0x4a` 16-bit value, `+0x50`/`+0x54`/
/// `+0x58` floats. `stream` is only passed through to the callees.
///
/// Operations: 0 reads fields from `stream` into the task (flag bits 1, 5, 6
/// set from single bits, then depending on them a word, a 19-byte blob, or a
/// scaled registry value); 1 is the same shape with immediate-writing callees;
/// 2 does nothing; 3 writes the task's fields into `stream` (flag bits read
/// back as single bits, integers written with widths 3, 7 and 16); 4 formats
/// the task's state through the log callee (floats widened exactly from 32 to
/// 64 bits, addresses passed as immediates); 5 returns the encoded size of the
/// task (1, 13, 27, a registry base plus 13, or 27 by flag combination).
/// Any other `op` returns 0. All paths except operation 5 return 0.
///
/// Original: 0x00d3baf0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00d3baf0(task: u32, op: u32, stream: u32) -> u32 {
    unsafe {
        const F_INT_A: u32 = 0x10;
        const F_INT_B: u32 = 0x14;
        const F_CHILD: u32 = 0x18;
        const F_INT_C: u32 = 0x1c;
        const F_FLAGS: u32 = 0x48;
        const F_WORD: u32 = 0x4a;
        const F_BLOB: u32 = 0x50;
        const F_FLT0: u32 = 0x50;
        const F_FLT1: u32 = 0x54;
        const F_FLT2: u32 = 0x58;
        const VT_SLOT_WRITE: u32 = 0x10;
        const VT_SLOT_CHILD: u32 = 0x54;
        const INT_B_BIAS: u32 = 0x15e;
        const BLOB_LEN: u32 = 0x13;
        const SIZE_BASE_ADD: u32 = 0x0d;
        const SIZE_WORD_PATH: u32 = 0x1b;
        const SIZE_PLAIN: u32 = 0x0d;
        const SIZE_CHILD: u32 = 0x1d;
        const LOG_FILE: u32 = 0x0198b570;
        const FMT_HEAD: u32 = 0x00ee3508;
        const FMT_SEL_A: u32 = 0x00ee3520;
        const FMT_SEL_B: u32 = 0x00ee3528;
        const FMT_SEL: u32 = 0x00ee3530;
        const FMT_WORD: u32 = 0x00ee3544;
        const FMT_STR: u32 = 0x00ee355c;
        const FMT_VEC: u32 = 0x00ee3574;
        const FMT_QRY: u32 = 0x00ee3598;
        const FMT_NOCHILD: u32 = 0x00ee35bc;
        const FIND_A: u32 = 0x0114e6e4;
        const FIND_B: u32 = 0x01040148;
        const FIND_RES_OFF: u32 = 0x6c;
        const REG_OFF: u32 = 0x08;

        const C_READ_BOOL: u32 = 1;
        const C_READ_BITS: u32 = 2;
        const C_READ_WORD: u32 = 3;
        const C_POST_READ: u32 = 4;
        const C_READ_BLOB: u32 = 5;
        const C_HELPER: u32 = 6;
        const C_REGISTRY: u32 = 7;
        const C_WRITE_IMM: u32 = 8;
        const C_MARK_A: u32 = 9;
        const C_MARK_B: u32 = 10;
        const C_WRITE_BOOL: u32 = 12;
        const C_WRITE_BITS: u32 = 13;
        const C_WRITE_WORD: u32 = 14;
        const C_WRITE_BLOB: u32 = 15;
        const C_LOG4: u32 = 17;
        const C_LOG5: u32 = 18;
        const C_FIND: u32 = 19;
        const C_STRINGIFY: u32 = 20;
        const C_QUERY: u32 = 21;
        const C_SIZE_BASE: u32 = 22;
        const C_LOG10: u32 = 23;

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
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        /// Fold one input bit into a flag bit: the original's
        /// shift/xor/mask/xor sequence, which sets flag bit `mask` to bit 0
        /// of `b` and leaves the rest.
        #[inline(always)]
        unsafe fn fold_bit(task: u32, b: u8, shift: u32, mask: u8) {
            unsafe {
                let mut al = b.wrapping_shl(shift);
                al ^= rd8(task + F_FLAGS);
                al &= mask;
                wr8(task + F_FLAGS, rd8(task + F_FLAGS) ^ al);
            }
        }
        /// The original answers bit reads through one shared scratch slot and
        /// word reads through slots overlapping it; later calls observe
        /// earlier answers there, so the slots keep the same relative layout
        /// (bool at +0x13, words at +0x14, +0x18 and +0x1c).
        #[inline(always)]
        unsafe fn read_bool_at(stream: u32, slot: u32) -> u8 {
            unsafe {
                lf_checker_rt::callee_thiscall!(C_READ_BOOL, u32, stream, slot);
                rd8(slot)
            }
        }
        #[inline(always)]
        unsafe fn read_bits_at(stream: u32, slot: u32, width: u32) -> u32 {
            unsafe {
                lf_checker_rt::callee_thiscall!(C_READ_BITS, u32, stream, slot, width);
                rd32(slot)
            }
        }
        /// Exact 32-to-64-bit float widening, as the original's
        /// load/convert/store sequence does it.
        #[inline(always)]
        fn widen(bits: u32) -> u64 {
            (core::hint::black_box(f32::from_bits(bits)) as f64).to_bits()
        }

        match op {
            0 => {
                let mut scratch = [0u8; 32];
                let sbase = scratch.as_mut_ptr() as u32;
                let (sbool, sbits1, sbits2, sbits3) =
                    (sbase + 0x13, sbase + 0x14, sbase + 0x18, sbase + 0x1c);
                let b = read_bool_at(stream, sbool);
                fold_bit(task, b, 1, 0x02);
                if rd8(task + F_FLAGS) & 0x02 == 0 {
                    return 0;
                }
                wr32(task + F_INT_B, read_bits_at(stream, sbits1, 3).wrapping_add(INT_B_BIAS));
                wr32(task + F_INT_A, read_bits_at(stream, sbits2, 7));
                let b = read_bool_at(stream, sbool);
                fold_bit(task, b, 5, 0x20);
                if rd8(task + F_FLAGS) & 0x20 == 0 {
                    let b = read_bool_at(stream, sbool);
                    wr8(
                        task + F_FLAGS,
                        rd8(task + F_FLAGS) & 0x7f | b.wrapping_shl(7),
                    );
                    if rd8(task + F_FLAGS) & 0x80 == 0 {
                        return 0;
                    }
                    let v = read_bits_at(stream, sbits3, 0x10);
                    let reg: u32 = lf_checker_rt::callee_cdecl!(C_REGISTRY, u32,);
                    let scaled = v.wrapping_shl(5).wrapping_add(rd32(reg + REG_OFF));
                    lf_checker_rt::callee_thiscall!(C_HELPER, u32, task, scaled);
                    return 0;
                }
                let b = read_bool_at(stream, sbool);
                fold_bit(task, b, 6, 0x40);
                if rd8(task + F_FLAGS) & 0x40 == 0 {
                    lf_checker_rt::callee_thiscall!(
                        C_READ_BLOB,
                        u32,
                        stream,
                        task + F_BLOB,
                        BLOB_LEN
                    );
                    return 0;
                }
                lf_checker_rt::callee_thiscall!(C_READ_WORD, u32, stream, task + F_WORD);
                lf_checker_rt::callee_thiscall!(C_POST_READ, u32, task);
                0
            }
            1 => {
                let mut scratch = [0u8; 32];
                let sbool = scratch.as_mut_ptr() as u32 + 0x13;
                let b = read_bool_at(stream, sbool);
                fold_bit(task, b, 1, 0x02);
                if rd8(task + F_FLAGS) & 0x02 == 0 {
                    return 0;
                }
                lf_checker_rt::callee_thiscall!(C_WRITE_IMM, u32, stream, 3, 1);
                lf_checker_rt::callee_thiscall!(C_WRITE_IMM, u32, stream, 7, 1);
                let b = read_bool_at(stream, sbool);
                fold_bit(task, b, 5, 0x20);
                if rd8(task + F_FLAGS) & 0x20 == 0 {
                    let b = read_bool_at(stream, sbool);
                    wr8(
                        task + F_FLAGS,
                        rd8(task + F_FLAGS) & 0x7f | b.wrapping_shl(7),
                    );
                    if rd8(task + F_FLAGS) & 0x80 == 0 {
                        return 0;
                    }
                    lf_checker_rt::callee_thiscall!(C_WRITE_IMM, u32, stream, 0x10, 1);
                    return 0;
                }
                let b = read_bool_at(stream, sbool);
                fold_bit(task, b, 6, 0x40);
                if rd8(task + F_FLAGS) & 0x40 == 0 {
                    lf_checker_rt::callee_thiscall!(C_MARK_B, u32, stream, 1, BLOB_LEN);
                    return 0;
                }
                lf_checker_rt::callee_thiscall!(C_MARK_A, u32, stream, 1);
                0
            }
            3 => {
                let hook: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(task) + VT_SLOT_WRITE) as usize);
                hook(task);
                let flags = rd8(task + F_FLAGS);
                lf_checker_rt::callee_thiscall!(
                    C_WRITE_BOOL,
                    u32,
                    stream,
                    (flags.wrapping_shr(1) & 1) as u32
                );
                if flags & 0x02 == 0 {
                    return 0;
                }
                lf_checker_rt::callee_thiscall!(
                    C_WRITE_BITS,
                    u32,
                    stream,
                    rd32(task + F_INT_B).wrapping_sub(INT_B_BIAS),
                    3
                );
                lf_checker_rt::callee_thiscall!(
                    C_WRITE_BITS,
                    u32,
                    stream,
                    rd32(task + F_INT_A),
                    7
                );
                let flags = rd8(task + F_FLAGS);
                lf_checker_rt::callee_thiscall!(
                    C_WRITE_BOOL,
                    u32,
                    stream,
                    (flags.wrapping_shr(5) & 1) as u32
                );
                if flags & 0x20 == 0 {
                    lf_checker_rt::callee_thiscall!(
                        C_WRITE_BOOL,
                        u32,
                        stream,
                        (flags.wrapping_shr(7) & 1) as u32
                    );
                    if flags & 0x80 == 0 {
                        return 0;
                    }
                    let child = rd32(task + F_CHILD);
                    let child_hook: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(rd32(rd32(child) + VT_SLOT_CHILD) as usize);
                    let r = child_hook(child);
                    let reg: u32 = lf_checker_rt::callee_cdecl!(C_REGISTRY, u32,);
                    let v = (r as i32).wrapping_sub(rd32(reg + REG_OFF) as i32) >> 5;
                    lf_checker_rt::callee_thiscall!(C_WRITE_BITS, u32, stream, v as u32, 0x10);
                    return 0;
                }
                let b = (flags.wrapping_shr(6) & 1) as u32;
                lf_checker_rt::callee_thiscall!(C_WRITE_BOOL, u32, stream, b);
                if flags & 0x40 == 0 {
                    lf_checker_rt::callee_thiscall!(
                        C_WRITE_BLOB,
                        u32,
                        stream,
                        task + F_BLOB,
                        BLOB_LEN
                    );
                    return 0;
                }
                lf_checker_rt::callee_thiscall!(
                    C_WRITE_WORD,
                    u32,
                    stream,
                    rd16(task + F_WORD) as u32
                );
                0
            }
            4 => {
                let file = lf_checker_rt::relocated(LOG_FILE);
                lf_checker_rt::callee_cdecl!(
                    C_LOG4,
                    u32,
                    file,
                    0,
                    0,
                    lf_checker_rt::relocated(FMT_HEAD)
                );
                let flags = rd8(task + F_FLAGS);
                let sel = if flags & 0x02 == 0 { FMT_SEL_B } else { FMT_SEL_A };
                lf_checker_rt::callee_cdecl!(
                    C_LOG5,
                    u32,
                    file,
                    0,
                    1,
                    lf_checker_rt::relocated(FMT_SEL),
                    lf_checker_rt::relocated(sel)
                );
                if flags & 0x02 == 0 {
                    return 0;
                }
                if flags & 0x20 == 0 {
                    if flags & 0x80 == 0 {
                        return 0;
                    }
                    let child = rd32(task + F_CHILD);
                    if child == 0 {
                        lf_checker_rt::callee_cdecl!(
                            C_LOG4,
                            u32,
                            file,
                            0,
                            1,
                            lf_checker_rt::relocated(FMT_NOCHILD)
                        );
                        return 0;
                    }
                    let mut slot: u32 = 0;
                    let q1: u32 = lf_checker_rt::callee_thiscall!(
                        C_QUERY,
                        u32,
                        child,
                        &mut slot as *mut u32 as u32,
                        0
                    );
                    let q2: u32 = lf_checker_rt::callee_thiscall!(
                        C_QUERY,
                        u32,
                        child,
                        &mut slot as *mut u32 as u32,
                        0
                    );
                    let d1 = widen(rd32(q2 + 8));
                    let q3: u32 = lf_checker_rt::callee_thiscall!(
                        C_QUERY,
                        u32,
                        child,
                        &mut slot as *mut u32 as u32,
                        0
                    );
                    let d2 = widen(rd32(q3 + 4));
                    let d3 = widen(rd32(q1));
                    lf_checker_rt::callee_cdecl!(
                        C_LOG10,
                        u32,
                        file,
                        0,
                        1,
                        lf_checker_rt::relocated(FMT_QRY),
                        d3 as u32,
                        (d3 >> 32) as u32,
                        d2 as u32,
                        (d2 >> 32) as u32,
                        d1 as u32,
                        (d1 >> 32) as u32
                    );
                    return 0;
                }
                if flags & 0x40 == 0 {
                    let d3 = widen(rd32(task + F_FLT0));
                    let d2 = widen(rd32(task + F_FLT1));
                    let d1 = widen(rd32(task + F_FLT2));
                    lf_checker_rt::callee_cdecl!(
                        C_LOG10,
                        u32,
                        file,
                        0,
                        1,
                        lf_checker_rt::relocated(FMT_VEC),
                        d3 as u32,
                        (d3 >> 32) as u32,
                        d2 as u32,
                        (d2 >> 32) as u32,
                        d1 as u32,
                        (d1 >> 32) as u32
                    );
                    return 0;
                }
                let found: u32 = lf_checker_rt::callee_cdecl!(
                    C_FIND,
                    u32,
                    rd32(task + F_INT_C),
                    0,
                    lf_checker_rt::relocated(FIND_A),
                    lf_checker_rt::relocated(FIND_B),
                    0
                );
                if found == 0 {
                    let sx = rd16(task + F_WORD) as i16 as i32 as u32;
                    lf_checker_rt::callee_cdecl!(
                        C_LOG5,
                        u32,
                        file,
                        0,
                        1,
                        lf_checker_rt::relocated(FMT_WORD),
                        sx
                    );
                    return 0;
                }
                let s = rd32(found + FIND_RES_OFF);
                if s == 0 {
                    return 0;
                }
                let v: u32 = lf_checker_rt::callee_thiscall!(C_STRINGIFY, u32, s);
                lf_checker_rt::callee_cdecl!(
                    C_LOG5,
                    u32,
                    file,
                    0,
                    1,
                    lf_checker_rt::relocated(FMT_STR),
                    v
                );
                0
            }
            5 => {
                let flags = rd8(task + F_FLAGS);
                if flags & 0x02 == 0 {
                    return 1;
                }
                if flags & 0x20 == 0 {
                    if flags & 0x80 == 0 {
                        return SIZE_PLAIN;
                    }
                    return SIZE_CHILD;
                }
                if flags & 0x40 == 0 {
                    let base: u32 = lf_checker_rt::callee_cdecl!(C_SIZE_BASE, u32,);
                    return base.wrapping_add(SIZE_BASE_ADD);
                }
                SIZE_WORD_PATH
            }
            _ => 0,
        }
    }
});
