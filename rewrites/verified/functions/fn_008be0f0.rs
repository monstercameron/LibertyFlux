// original: 0x008BE0F0 mode_table_lookup_and_flag_set (proposed)

/// Resolve the current mode from three tables, notify through a callee, then
/// latch flag slots.
///
/// The global index word selects one entry of a directory table (24 bytes per
/// entry: row pointer, 16-bit row count). The entry's rows (22 bytes each) are
/// scanned for the first whose tag byte is `ROW_TAG`; that row names a key
/// byte and a signed word. The word indexes the value array; the key selects
/// one of 62 key-table entries (12 bytes: key, pointer to a 24-byte-stride
/// table); the mode is the dword at `+0x14` of the value-selected element.
/// A missing row or key leaves the mode at 0.
///
/// The notifier callee then sees (found row or 0, value-selected-by-lookup,
/// index). When the mode is 3, 4 or 15 the wide path latches one slot to 1,
/// two slots to 0 and clears slots `0x77..=0x88`, returning `0x89`;
/// otherwise the narrow path latches one slot to 1 and one to 0 and returns
/// the second lookup. Every lookup answer of `NONE` (`0x7fffffff`, callee for
/// "absent") is replaced by a fixed default slot, except on the narrow path
/// where it returns immediately.
///
/// Original: 0x008BE0F0 (cdecl, no arguments, no register inputs; callee
/// `F` takes (handle, code) and callee `G` takes (row, value, index)).
lf_checker_rt::export!(cdecl, rw_008BE0F0() -> u32 {
    unsafe {
        const INDEX: u32 = 0x01160C40;
        const VALS: u32 = 0x01160C48;
        const DIR: u32 = 0x019D33A0;
        const DIR_COUNT_OFF: u32 = 0x00000004;
        const KEYTAB: u32 = 0x019D30C0;
        const KEYTAB_END: u32 = 0x019D3390;
        const NONE: u32 = 0x7FFF_FFFF;
        const ROW_TAG: u8 = 0x19;
        const ROW_STRIDE: u32 = 0x16;
        const ROW_WORD_OFF: u32 = 0x12;
        const ROW_KEY_OFF: u32 = 0x15;
        const CAL_F: u32 = 0;
        const CAL_G: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(a).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { lf_checker_rt::global::<u16>(a).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { lf_checker_rt::global::<u8>(a).read() }
        }
        // Raw loads for runtime pointers (rows, key-table targets): these are
        // already relocated by the load that produced them, never file VAs.
        #[inline(always)]
        unsafe fn m32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn m16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn m8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { lf_checker_rt::global::<u32>(a).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn val_at(i: u32) -> u32 {
            unsafe { rd32(VALS.wrapping_add(i.wrapping_mul(4))) }
        }
        #[inline(always)]
        unsafe fn val_set(i: u32, v: u32) {
            unsafe { wr32(VALS.wrapping_add(i.wrapping_mul(4)), v) }
        }

        let idx: u32 = rd32(INDEX);
        let mut row: u32 = 0;
        let mut mode: u32 = 0;
        let mut resolved: u32 = lf_checker_rt::callee_cdecl!(CAL_F, u32, idx, 0x19u32);
        if resolved == NONE {
            resolved = 0x2B;
        }
        let entry: u32 = idx.wrapping_mul(3).wrapping_mul(8);
        let count: i32 = rd16(DIR.wrapping_add(entry).wrapping_add(DIR_COUNT_OFF)) as i32;
        if count > 0 {
            let rows: u32 = rd32(DIR.wrapping_add(entry));
            let mut i: u32 = 0;
            let mut found: u32 = count as u32;
            while i < count as u32 {
                if m8(rows.wrapping_add(i.wrapping_mul(ROW_STRIDE))) == ROW_TAG {
                    found = i;
                    break;
                }
                i += 1;
            }
            if found < count as u32 {
                row = found;
                let base: u32 = rows.wrapping_add(found.wrapping_mul(ROW_STRIDE));
                let w: i32 = m16(base.wrapping_add(ROW_WORD_OFF)) as i16 as i32;
                let key: u8 = m8(base.wrapping_add(ROW_KEY_OFF));
                let v: u32 = val_at(w as u32);
                let mut p: u32 = KEYTAB;
                let mut j: u32 = 0;
                let mut hit: bool = false;
                while p < KEYTAB_END {
                    if rd32(p) == key as u32 {
                        hit = true;
                        break;
                    }
                    p += 0x0C;
                    j += 1;
                }
                if hit {
                    let pt: u32 = rd32(KEYTAB.wrapping_add(4).wrapping_add(j.wrapping_mul(12)));
                    mode = m32(pt.wrapping_add(v.wrapping_mul(24)).wrapping_add(0x14));
                }
            }
        }
        let cur: u32 = val_at(resolved);
        let handle: u32 = lf_checker_rt::callee_cdecl!(CAL_G, u32, row, cur, idx);
        let m: i32 = mode as i32;
        if m >= 3 && (m <= 4 || m == 0x0F) {
            let mut r: u32 = lf_checker_rt::callee_cdecl!(CAL_F, u32, handle, 0x1Au32);
            if r == NONE {
                r = 0x55;
            }
            val_set(r, 1);
            r = lf_checker_rt::callee_cdecl!(CAL_F, u32, handle, 0x1Du32);
            if r == NONE {
                r = 0x2E;
            }
            val_set(r, 0);
            r = lf_checker_rt::callee_cdecl!(CAL_F, u32, handle, 0x1Eu32);
            if r == NONE {
                r = 0x2F;
            }
            val_set(r, 0);
            let mut s: u32 = 0x77;
            loop {
                if s != NONE {
                    val_set(s, 0);
                }
                s += 1;
                if s > 0x88 {
                    break;
                }
            }
            0x89
        } else {
            let mut r: u32 = lf_checker_rt::callee_cdecl!(CAL_F, u32, handle, 0x1Au32);
            if r == NONE {
                return NONE;
            }
            val_set(r, 1);
            r = lf_checker_rt::callee_cdecl!(CAL_F, u32, handle, 0x1Cu32);
            if r == NONE {
                return NONE;
            }
            val_set(r, 0);
            r
        }
    }
});
