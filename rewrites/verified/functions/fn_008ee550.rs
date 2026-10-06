// original: 0x008EE550 tls_logged_table_append (proposed)

/// Log out-of-range keys, then append a record to the shared table.
///
/// Arguments are eight stack words: `key` (a0), `lo` (a1), `hi` (a2), two
/// flag bytes (a3, a4), a float level (a5) and two more flag bytes (a6,
/// a7). When `lo` or `hi` exceeds 100000 (both compared SIGNED: `jg` /
/// `jle`), the triple is reported through the log callee (callee 1) with
/// the thread's log buffer. When `lo == hi`, `key` is hashed with the
/// Joaat callee (callee 2), the hash is logged, and the match table (count
/// at its global, rows of 48 bytes) is scanned for rows whose word at
/// `+0x1c` equals the hash; each match decrements a counter seeded with
/// `lo`, and when it goes negative the three floats just past the table
/// end are widened to doubles and logged through callee 3, ending the
/// scan. The scan bound and the counter step are SIGNED (`jl`, `js`).
///
/// Finally, unless the append-table pointer global is null, one 20-byte
/// record is appended at the append-index global (which is then
/// incremented): hash of `key`, `lo`, `hi`, the two flag bytes at `+0xe`
/// and `+0xf`, the level clamped to a maximum of 15.0 (an unordered
/// compare keeps 15.0) and truncated toward zero into the byte at `+0x10`,
/// and the low bits of the last two flag bytes into bits 1 and 3 of the
/// half-word at `+0xc` (read-modify-write).
///
/// Original: 0x008EE550 (stdcall, eight stack words, no return value).
lf_checker_rt::export!(stdcall, rw_008EE550(
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
) -> u32 {
    unsafe {
        const BOUND: i32 = 0x186a0;
        const LOG_BUF_OFF: u32 = 0x4c8;
        // Format pointers: relocated immediates, derived from the file VAs.
        const LOG_RANGE_FMT: u32 = 0xe83344;
        const LOG_HASH_FMT: u32 = 0xe8336c;
        const LOG_VEC_FMT: u32 = 0xe833a4;
        const ROW_STRIDE: u32 = 0x30;
        const ROW_HASH: u32 = 0x1c;
        const REC_STRIDE: u32 = 20;
        const CALLEE_LOG: u32 = 1;
        const CALLEE_HASH: u32 = 2;
        const CALLEE_LOGVEC: u32 = 3;
        let level_max = *lf_checker_rt::global::<f32>(0xfe8b20);

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn log_buf() -> u32 {
            unsafe {
                let slot = *lf_checker_rt::global::<u32>(0x17aba14);
                lf_checker_rt::tls_slot(slot as usize).wrapping_add(LOG_BUF_OFF)
            }
        }

        // SIGNED range check: log when either key exceeds the bound.
        if (a1 as i32) > BOUND || (a2 as i32) > BOUND {
            lf_checker_rt::callee_cdecl!(CALLEE_LOG, u32, log_buf(), lf_checker_rt::relocated(LOG_RANGE_FMT), a1, a2, a0);
        }
        if a1 == a2 {
            let hash: u32 = lf_checker_rt::callee_cdecl!(CALLEE_HASH, u32, a0, 0);
            lf_checker_rt::callee_cdecl!(CALLEE_LOG, u32, log_buf(), lf_checker_rt::relocated(LOG_HASH_FMT), a0, a0, 0);
            let mut left = a1;
            let n = *lf_checker_rt::global::<i32>(0x1176e38);
            if n > 0 {
                let base = *lf_checker_rt::global::<u32>(0x1176e40);
                let mut off = 0u32;
                let mut count = 0i32;
                while count < n {
                    if (left as i32) < 0 {
                        break;
                    }
                    if hash == rd32(base.wrapping_add(off).wrapping_add(ROW_HASH)) {
                        left = left.wrapping_sub(1);
                        if (left as i32) < 0 {
                            let fbase =
                                base.wrapping_add((n as u32).wrapping_mul(ROW_STRIDE));
                            let d0 = f32::from_bits(rd32(fbase)) as f64;
                            let d1 = f32::from_bits(rd32(fbase.wrapping_add(4))) as f64;
                            let d2 = f32::from_bits(rd32(fbase.wrapping_add(8))) as f64;
                            let (b0, b1, b2) = (d0.to_bits(), d1.to_bits(), d2.to_bits());
                            lf_checker_rt::callee_cdecl!(
                                CALLEE_LOGVEC,
                                u32,
                                log_buf(),
                                lf_checker_rt::relocated(LOG_VEC_FMT),
                                b0 as u32,
                                (b0 >> 32) as u32,
                                b1 as u32,
                                (b1 >> 32) as u32,
                                b2 as u32,
                                (b2 >> 32) as u32
                            );
                        }
                    }
                    count += 1;
                    off = off.wrapping_add(ROW_STRIDE);
                }
            }
        }
        let t = *lf_checker_rt::global::<u32>(0x1176e44);
        if t != 0 {
            let hash: u32 = lf_checker_rt::callee_cdecl!(CALLEE_HASH, u32, a0, 0);
            let w = *lf_checker_rt::global::<u32>(0x1176e3c);
            let e = t.wrapping_add(w.wrapping_mul(REC_STRIDE));
            wr32(e, hash);
            wr32(e.wrapping_add(4), a1);
            wr32(e.wrapping_add(8), a2);
            ((e.wrapping_add(0xe)) as *mut u8).write(a3 as u8);
            ((e.wrapping_add(0xf)) as *mut u8).write(a4 as u8);
            let f = f32::from_bits(a5);
            let fc = if level_max > f { f } else { level_max };
            ((e.wrapping_add(0x10)) as *mut u8).write((fc as i32) as u8);
            let u = ((e.wrapping_add(0xc)) as *const u16).read_unaligned();
            ((e.wrapping_add(0xc)) as *mut u16)
                .write_unaligned((u & !2) | ((((a6 as u8) & 1) as u16) << 1));
            let u = ((e.wrapping_add(0xc)) as *const u16).read_unaligned();
            ((e.wrapping_add(0xc)) as *mut u16)
                .write_unaligned((u & !8) | ((((a7 as u8) & 1) as u16) << 3));
            *lf_checker_rt::global::<u32>(0x1176e3c) = w.wrapping_add(1);
        }
        0
    }
});
