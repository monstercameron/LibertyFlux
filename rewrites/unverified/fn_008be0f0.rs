// original: 0x008be0f0 input_apply_device_bindings (proposed)

/// Read the current device's key record and commit its binding state to the slots.
///
/// Takes no arguments; the device index comes from `G_CUR_INDEX` and incoming
/// ECX is ignored (the original pushes it and reuses its stack slot for a
/// local before any read). The per-device table `T_DEVS` has rows of 24
/// bytes (record-array pointer at `+0`, record count as a 16-bit word at
/// `+4`); each record is `REC_LEN` bytes with a tag byte at `+0`, a signed
/// 16-bit key at `+0x12` and a binding id byte at `+0x15`.
///
/// The first record tagged `TAG` selects a key whose slot value seeds a scan
/// of the 60-row binding table `T_DEFS` (rows of 12 bytes: id at `+0`,
/// value-list pointer at `+4`); on a hit the mode word is read from the
/// value list at `g*24+0x14`. The dispatch callee then runs with the found
/// index, the first answer's slot value and the device index. Mode 3, 4 or
/// 15 takes the main path (three key-callee answers committed as 1/0/0,
/// then slots `0x77..=0x88` cleared, returning `0x89`); any other mode, or
/// no tagged record, takes the alternate path (two answers committed as 1/0
/// with early `NO_LINK` returns). `NO_LINK` answers are replaced by fixed
/// alternates except on the alternate path, where they return early.
///
/// Original: 0x008be0f0 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_008be0f0() -> u32 {
    unsafe {
        const G_CUR_INDEX: u32 = 0x01160C40;
        const G_SLOTS: u32 = 0x01160C48;
        const T_DEFS: u32 = 0x019D30C0;
        const T_DEF_END: u32 = 0x019D3390;
        const T_DEVS: u32 = 0x019D33A0;
        const REC_LEN: u32 = 0x16;
        const REC_TAG: u32 = 0x0;
        const REC_KEY: u32 = 0x12;
        const REC_ID: u32 = 0x15;
        const NO_LINK: u32 = 0x7FFF_FFFF;
        const TAG: u8 = 0x19;
        const ALT_PAIR: u32 = 0x2b;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let slots = lf_checker_rt::relocated(G_SLOTS);
        let ebx = rd32(lf_checker_rt::relocated(G_CUR_INDEX));
        let mut pair = lf_checker_rt::callee_cdecl!(1, u32, ebx, 0x19_u32);
        if pair == NO_LINK {
            pair = ALT_PAIR;
        }
        let dev_row = lf_checker_rt::relocated(T_DEVS).wrapping_add(ebx.wrapping_mul(24));
        let n = rd16(dev_row.wrapping_add(4)) as i32;
        let mut found = 0u32;
        let mut mode = 0i32;
        if n > 0 {
            let recs = rd32(dev_row);
            let mut c = 0i32;
            loop {
                if rd8(recs.wrapping_add((c as u32).wrapping_mul(REC_LEN).wrapping_add(REC_TAG))) == TAG {
                    found = c as u32;
                    let key = rd16(recs.wrapping_add((c as u32).wrapping_mul(REC_LEN).wrapping_add(REC_KEY))) as i16 as i32;
                    let id = rd8(recs.wrapping_add((c as u32).wrapping_mul(REC_LEN).wrapping_add(REC_ID)));
                    let g = rd32(slots.wrapping_add((key as u32).wrapping_mul(4)));
                    let mut t = lf_checker_rt::relocated(T_DEFS);
                    let end = lf_checker_rt::relocated(T_DEF_END);
                    loop {
                        if rd32(t) == id as u32 {
                            let q = rd32(t.wrapping_add(4));
                            mode = rd32(q.wrapping_add(g.wrapping_mul(24).wrapping_add(0x14))) as i32;
                            break;
                        }
                        t = t.wrapping_add(12);
                        if t >= end {
                            break;
                        }
                    }
                    break;
                }
                c += 1;
                if c >= n {
                    break;
                }
            }
        }
        let gpair = rd32(slots.wrapping_add(pair.wrapping_mul(4)));
        let esi = lf_checker_rt::callee_cdecl!(2, u32, found, gpair, ebx);
        if mode >= 3 && (mode <= 4 || mode == 0x0f) {
            let mut x = lf_checker_rt::callee_cdecl!(3, u32, esi, 0x1a_u32);
            if x == NO_LINK {
                x = 0x55;
            }
            wr32(slots.wrapping_add(x.wrapping_mul(4)), 1);
            let mut y = lf_checker_rt::callee_cdecl!(4, u32, esi, 0x1d_u32);
            if y == NO_LINK {
                y = 0x2e;
            }
            wr32(slots.wrapping_add(y.wrapping_mul(4)), 0);
            let mut z = lf_checker_rt::callee_cdecl!(5, u32, esi, 0x1e_u32);
            if z == NO_LINK {
                z = 0x2f;
            }
            wr32(slots.wrapping_add(z.wrapping_mul(4)), 0);
            let mut w = 0x77u32;
            loop {
                // The original skips NO_LINK here; w never reaches it.
                if w != NO_LINK {
                    wr32(slots.wrapping_add(w.wrapping_mul(4)), 0);
                }
                w = w.wrapping_add(1);
                if w > 0x88 {
                    break;
                }
            }
            return w;
        }
        let x = lf_checker_rt::callee_cdecl!(6, u32, esi, 0x1a_u32);
        if x == NO_LINK {
            return x;
        }
        wr32(slots.wrapping_add(x.wrapping_mul(4)), 1);
        let y = lf_checker_rt::callee_cdecl!(7, u32, esi, 0x1c_u32);
        if y == NO_LINK {
            return y;
        }
        wr32(slots.wrapping_add(y.wrapping_mul(4)), 0);
        y
    }
});
