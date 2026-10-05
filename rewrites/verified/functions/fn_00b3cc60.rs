// original: 0x00B3CC60 task_table_scan_and_score (proposed)

/// Scan a global entry table for the first free slot, then score it.
///
/// `obj` points at a 64-byte input record, `flt_c`/`flt_a` are two floats
/// passed through to the scorer, `arg_b` a word passed through, and `flag`
/// (low byte) selects whether entries must also carry bit 1 and whether the
/// record is copied (lane-permuted) into a scratch buffer first. The table
/// holds 48 entries of 0x64 bytes at TABLE; entries whose first byte has bit
/// 0 set (and bit 1 when `flag` is nonzero) are skipped and counted. A full
/// scan returns 0. Otherwise the scorer (callee 1) is invoked with the found
/// entry address in ECX and `(buffer-or-obj, flt_c, flt_a, arg_b, flag)` on
/// the stack, a global saturating counter (capped at 0x30) is bumped, and 1
/// is returned. Both exits verify the frame cookie through callee 2 (which
/// preserves registers). Cdecl, five stack words.
///
/// A set global spin flag would make the original block calling its sleeper;
/// that path is untestable on the single-threaded worker (the flag is BSS
/// zero) and is mirrored here as a plain spin (see `narrowed`).
lf_checker_rt::export!(cdecl, rw_00B3CC60(obj: u32, flt_c: u32, flt_a: u32, arg_b: u32, flag: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x016B_7CB0;
        const TABLE_END: u32 = 0x016B_8F70;
        const ENTRY: u32 = 0x64;
        const COUNTER: u32 = 0x016B_7CAC;
        const SAT: u32 = 0x30;
        const SPIN: u32 = 0x016B_7BF2;
        const COOKIE: u32 = 0x0105_7FB4;
        const CAL_SCORE: u32 = 1;
        const CAL_COOKIE: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn g8(va: u32) -> u8 {
            unsafe { lf_checker_rt::global::<u8>(va).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn setg32(va: u32, v: u32) {
            unsafe { lf_checker_rt::global::<u32>(va).write_unaligned(v) }
        }

        // Spin-wait gate (never taken in proofs; see doc comment).
        if g8(SPIN) != 0 {
            loop {
                core::hint::spin_loop();
            }
        }

        // Table scan.
        let flagb = flag as u8;
        let mut entry = TABLE;
        let mut count = 0u32;
        let found = loop {
            let cl = g8(entry);
            if cl & 1 == 0 {
                break true;
            }
            if flagb != 0 && cl & 2 == 0 {
                break true;
            }
            entry = entry.wrapping_add(ENTRY);
            count = count.wrapping_add(1);
            if !(entry < TABLE_END) {
                break false;
            }
        };
        if !found {
            lf_checker_rt::callee_thiscall!(CAL_COOKIE, u32, g32(COOKIE));
            // The original's scan bound is an absolute immediate, which the
            // loader relocates; EAX holds the relocated end address here.
            return lf_checker_rt::relocated(TABLE_END) & 0xFFFF_FF00;
        }

        // Buffer: permuted copy when flagged, else the record itself.
        let mut buf = [0u32; 16];
        let arg0: u32;
        if flagb != 0 {
            let mut j = 0u32;
            while j < 8 {
                buf[j as usize] = rd32(obj.wrapping_add(j.wrapping_mul(4)));
                j += 1;
            }
            let mut k = 0u32;
            while k < 4 {
                buf[(8 + k) as usize] = rd32(obj.wrapping_add(0x30).wrapping_add(k.wrapping_mul(4)));
                buf[(12 + k) as usize] =
                    rd32(obj.wrapping_add(0x20).wrapping_add(k.wrapping_mul(4)));
                k += 1;
            }
            arg0 = buf.as_ptr() as u32;
        } else {
            arg0 = obj;
        }
        let entry_addr =
            lf_checker_rt::relocated(TABLE.wrapping_add(count.wrapping_mul(ENTRY)));
        lf_checker_rt::callee_thiscall!(CAL_SCORE, u32, entry_addr, arg0, flt_c, flt_a, arg_b, flagb as u32);

        // Saturating counter.
        let c = g32(COUNTER).wrapping_add(1);
        let sat = if (c as i32) < (SAT as i32) { c } else { SAT };
        setg32(COUNTER, sat);
        lf_checker_rt::callee_thiscall!(CAL_COOKIE, u32, g32(COOKIE));
        (sat & 0xFFFF_FF00) | 1
    }
});
