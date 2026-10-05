// original: 0x008BF180 input_row_rekey (proposed)

/// Re-resolve one input row's key byte from the shared tables and latch it.
///
/// `arg` selects a directory entry (24 bytes: row pointer, 16-bit count).
/// Two callee lookups resolve codes `0x1E`/`0x1D` against `arg` (`NONE`
/// answers default to `0x2F`/`0x2E`). The entry's rows (22 bytes each) are
/// scanned for the first whose signed word (`+0x12`) equals the first
/// lookup; a miss returns that scan's last word. A second scan finds the
/// first row matching the second lookup (0 when absent) and takes its key
/// byte (`+0x15`). The key selects one of 62 key-table entries (12 bytes:
/// key, value-table pointer, 16-bit limit); on a hit whose value-array
/// entry is at or above the limit (signed), that entry is cleared and the
/// value treated as 0, and the output dword comes from the value-table at
/// `+0x10` of the value-selected element (0 on a key miss). A second
/// key-table scan turns the output dword back into a limit byte (0 on miss).
/// Finally, unless the current-index row already holds the output dword, the
/// first lookup's value slot is cleared, and the row's `+0x15`/`+0x14` bytes
/// are set to the low bytes of the output dword and the limit.
///
/// The function reuses its incoming stack-argument slot as scratch (it holds
/// the row pointer, then the key); a Rust rewrite cannot reproduce that
/// store, so the proof narrows the stack check. Three comparisons are dead:
/// both lookups can never be `NONE` past their defaults, and the second
/// scan's empty check runs only when the count is already known positive.
///
/// Original: 0x008BF180 (cdecl, one stack argument; callee `F` takes
/// (handle, code); early return is `arg * 3`, main return the row pointer).
lf_checker_rt::export!(cdecl, rw_008BF180(arg: u32) -> u32 {
    unsafe {
        const INDEX: u32 = 0x01160C40;
        const VALS: u32 = 0x01160C48;
        const DIR: u32 = 0x019D33A0;
        const KEYTAB: u32 = 0x019D30C0;
        const KEYTAB_END: u32 = 0x019D3390;
        const NONE: u32 = 0x7FFF_FFFF;
        const ROW_STRIDE: u32 = 0x16;
        const ROW_WORD_OFF: u32 = 0x12;
        const ROW_LIM_OFF: u32 = 0x14;
        const ROW_KEY_OFF: u32 = 0x15;
        const CAL_F: u32 = 0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(a).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { lf_checker_rt::global::<u16>(a).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { lf_checker_rt::global::<u32>(a).write_unaligned(v) }
        }
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
        unsafe fn mwr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        let mut first: u32 = lf_checker_rt::callee_cdecl!(CAL_F, u32, arg, 0x1Eu32);
        if first == NONE {
            first = 0x2F;
        }
        let mut second: u32 = lf_checker_rt::callee_cdecl!(CAL_F, u32, arg, 0x1Du32);
        if second == NONE {
            second = 0x2E;
        }
        let entry: u32 = arg.wrapping_mul(3).wrapping_mul(8);
        let count: i32 = rd16(DIR.wrapping_add(entry).wrapping_add(4)) as i32;
        if count <= 0 {
            return arg.wrapping_mul(3);
        }
        let rows: u32 = rd32(DIR.wrapping_add(entry));
        // Scan 1: first row whose word equals the first lookup.
        let mut edi: u32 = 0;
        let mut cur: u32 = rows.wrapping_add(ROW_WORD_OFF);
        let mut w: i32;
        loop {
            w = m16(cur) as i16 as i32;
            if first == w as u32 {
                break;
            }
            edi += 1;
            cur = cur.wrapping_add(ROW_STRIDE);
            if edi >= count as u32 {
                return w as u32;
            }
        }
        // Scan 2: first row whose word equals the second lookup (0 if none).
        let mut idx2: u32 = 0;
        let mut c: u32 = 0;
        let mut d: u32 = rows.wrapping_add(ROW_WORD_OFF);
        if count > 0 {
            loop {
                let x: i32 = m16(d) as i16 as i32;
                if second == x as u32 {
                    idx2 = c;
                    break;
                }
                c += 1;
                d = d.wrapping_add(ROW_STRIDE);
                if c >= count as u32 {
                    idx2 = 0;
                    break;
                }
            }
        }
        let key: u8 = m8(rows.wrapping_add(idx2.wrapping_mul(ROW_STRIDE)).wrapping_add(ROW_KEY_OFF));
        let mut v: u32 = rd32(VALS.wrapping_add(second.wrapping_mul(4)));
        // Scan 3: key-table entry for the key.
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
        let mut out: u32 = 0;
        if hit {
            let lim: u32 = rd16(KEYTAB.wrapping_add(8).wrapping_add(j.wrapping_mul(12))) as u32;
            if (v as i32) < lim as i32 {
                // Below the limit: keep the value.
            } else {
                if second != NONE {
                    wr32(VALS.wrapping_add(second.wrapping_mul(4)), 0);
                }
                v = 0;
            }
            let pt: u32 = rd32(KEYTAB.wrapping_add(4).wrapping_add(j.wrapping_mul(12)));
            out = m32(pt.wrapping_add(v.wrapping_mul(24)).wrapping_add(0x10));
        }
        // Scan 4: key-table entry for the output dword; its limit byte.
        let mut q: u32 = KEYTAB;
        let mut k: u32 = 0;
        let mut hit2: bool = false;
        while q < KEYTAB_END {
            if rd32(q) == out {
                hit2 = true;
                break;
            }
            q += 0x0C;
            k += 1;
        }
        let mut limbyte: u32 = 0;
        if hit2 {
            limbyte = rd16(KEYTAB.wrapping_add(8).wrapping_add(k.wrapping_mul(12))) as u32;
        }
        let curix: u32 = rd32(INDEX);
        let rows2: u32 = rd32(DIR.wrapping_add(curix.wrapping_mul(3).wrapping_mul(8)));
        let slot: u32 = rows2.wrapping_add(edi.wrapping_mul(ROW_STRIDE));
        if m8(slot.wrapping_add(ROW_KEY_OFF)) as u32 != out {
            if first != NONE {
                wr32(VALS.wrapping_add(first.wrapping_mul(4)), 0);
            }
        }
        mwr8(slot.wrapping_add(ROW_KEY_OFF), out as u8);
        let rows3: u32 = rd32(DIR.wrapping_add(rd32(INDEX).wrapping_mul(3).wrapping_mul(8)));
        mwr8(rows3.wrapping_add(edi.wrapping_mul(ROW_STRIDE)).wrapping_add(ROW_LIM_OFF), limbyte as u8);
        rows3
    }
});
