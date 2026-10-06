// original: 0x00E56DF0 find_key_record_and_refresh (proposed)
/// Look up a key record and refresh every mismatching table row (original 0x00E56DF0).
///
/// `key` is a NUL-terminated string. The key service is asked first; a zero
/// low byte ends the function returning the service's full answer. Otherwise
/// a 16-byte descriptor is built on the frame from four global words and
/// handed, with an open tag, to the record service: a null record ends the
/// function with 0.
///
/// On a live record a handle is opened, a row count is read through it, and
/// the string table is allocated. The record is then bound a second time
/// with a bind tag (null again means 0). When the count is SIGNED-positive
/// (`jle` skips zero and negatives alike) the table is walked in 0x20-byte
/// rows, `(count - 1) >> 5 + 1` of them, each compared against the key with
/// an unsigned-byte strcmp; every mismatch is refreshed with the record and
/// the row size. Finally the record is released, the table is freed, and the
/// function returns the free call's answer with its low byte forced to 1.
///
/// Two frame details of the original are unobservable and not reproduced: the
/// table pointer is stashed in a below-ESP scratch word (only the loop setup
/// and the free call read it back; the rewrite keeps it in a local), and the
/// stack-cookie check calls run for real on the original side (left
/// unpatched; the rewrite has no cookie).
///
/// Original: stdcall, one stack argument, the callee pops 4 bytes.
lf_checker_rt::export!(stdcall, rw_e56df0(key: u32) -> u32 {
    unsafe {
        /// Global descriptor words copied into the frame struct.
        const DESC: u32 = 0x00F1_A320;
        /// Tags passed beside the descriptor.
        const TAG_OPEN: u32 = 0x00F1_A310;
        const TAG_LOOKUP: u32 = 0x00F1_A330;
        const TAG_BIND: u32 = 0x00F1_A334;
        /// Bytes per table row.
        const ROW: u32 = 0x20;
        /// Callee ids, matching the contract.
        const KEY_SVC: u32 = 1;
        const AUX: u32 = 2;
        const RECORD: u32 = 3;
        const HANDLE: u32 = 4;
        const ALLOC: u32 = 5;
        const PREP: u32 = 6;
        const TOUCH: u32 = 7;
        const BIND: u32 = 8;
        const REFRESH: u32 = 9;
        const FREE: u32 = 10;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }

        /// Unsigned-byte strcmp two bytes at a time, as the original's
        /// unrolled loop: -1/0/+1, of which only the zero-ness is used.
        unsafe fn cmp_row(row: u32, k: u32) -> i32 {
            unsafe {
                let mut c = row;
                let mut a = k;
                loop {
                    let d = rd8(c);
                    let e = rd8(a);
                    if d != e {
                        return if d < e { -1 } else { 1 };
                    }
                    if d == 0 {
                        return 0;
                    }
                    let d2 = rd8(c + 1);
                    let e2 = rd8(a + 1);
                    if d2 != e2 {
                        return if d2 < e2 { -1 } else { 1 };
                    }
                    c += 2;
                    a += 2;
                    if d2 == 0 {
                        return 0;
                    }
                }
            }
        }

        let ka = lf_checker_rt::callee_stdcall!(KEY_SVC, u32, key);
        if (ka as u8) == 0 {
            return ka;
        }
        lf_checker_rt::callee_cdecl!(AUX, u32, lf_checker_rt::relocated(TAG_OPEN), 0);
        let desc = [
            rd32(lf_checker_rt::relocated(DESC)),
            rd32(lf_checker_rt::relocated(DESC + 4)),
            rd32(lf_checker_rt::relocated(DESC + 8)),
            rd32(lf_checker_rt::relocated(DESC + 12)),
        ];
        let rec =
            lf_checker_rt::callee_cdecl!(RECORD, u32, desc.as_ptr() as u32, lf_checker_rt::relocated(TAG_LOOKUP));
        if rec == 0 {
            return 0;
        }
        let count = lf_checker_rt::callee_cdecl!(HANDLE, u32, rec);
        let table = lf_checker_rt::callee_cdecl!(ALLOC, u32, count);
        lf_checker_rt::callee_cdecl!(PREP, u32, rec, table, count);
        lf_checker_rt::callee_cdecl!(TOUCH, u32, rec);
        let bound =
            lf_checker_rt::callee_cdecl!(BIND, u32, desc.as_ptr() as u32, lf_checker_rt::relocated(TAG_BIND));
        if bound == 0 {
            return 0;
        }
        if (count as i32) > 0 {
            let mut n = (count.wrapping_sub(1) >> 5).wrapping_add(1);
            let mut row = table;
            while n != 0 {
                if cmp_row(row, key) != 0 {
                    lf_checker_rt::callee_cdecl!(REFRESH, u32, bound, row, ROW);
                }
                row = row.wrapping_add(ROW);
                n -= 1;
            }
        }
        lf_checker_rt::callee_cdecl!(TOUCH, u32, bound);
        let fa = lf_checker_rt::callee_cdecl!(FREE, u32, table);
        (fa & 0xFFFF_FF00) | 1
    }
});
