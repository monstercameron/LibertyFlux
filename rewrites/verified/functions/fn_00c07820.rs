// original: 0x00c07820 stream_record_find_by_key (proposed)

/// Search fixed-size records for one whose leading key matches the argument.
///
/// `this` points to the set (`RECORDS` holds the array, `COUNT` its 16-bit
/// length, each record `STRIDE` bytes starting with a NUL-terminated key).
/// The wanted key is the string at `KEY_OFF` past `arg`. Returns 1 on the
/// first match, 0 when the set is empty, and otherwise the low byte cleared
/// from the last comparison's sign (0, or all high bits set when the last
/// record's key sorted below the wanted one). A count at or above 0x8000 is
/// treated as empty and returns the count with its low byte cleared.
///
/// Original: 0x00c07820 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c07820(this: u32, arg: u32) -> u32 {
    unsafe {
        const RECORDS: u32 = 0x00;
        const COUNT: u32 = 0x04;
        const STRIDE: u32 = 0x28;
        const KEY_OFF: u32 = 0x2c;
        /// Signed three-way compare in the original's read order: 0 when
        /// equal, else -1 when the record byte sorts below, else 1.
        unsafe fn compare(mut a: u32, mut b: u32) -> i32 {
            unsafe {
                loop {
                    let c1 = (a as *const u8).read();
                    let d1 = (b as *const u8).read();
                    if c1 != d1 {
                        return if c1 < d1 { -1 } else { 1 };
                    }
                    if c1 == 0 {
                        return 0;
                    }
                    let c2 = (a.wrapping_add(1) as *const u8).read();
                    let d2 = (b.wrapping_add(1) as *const u8).read();
                    if c2 != d2 {
                        return if c2 < d2 { -1 } else { 1 };
                    }
                    a = a.wrapping_add(2);
                    b = b.wrapping_add(2);
                    if c2 == 0 {
                        return 0;
                    }
                }
            }
        }
        let count = (this.wrapping_add(COUNT) as *const u16).read_unaligned() as u32;
        if count == 0 {
            return 0;
        }
        if (count as i32) <= 0 {
            return count & 0xffff_ff00;
        }
        let base = (this.wrapping_add(RECORDS) as *const u32).read_unaligned();
        let want = arg.wrapping_add(KEY_OFF);
        let mut last = 0i32;
        let mut i = 0u32;
        while i < count {
            last = compare(base.wrapping_add(i.wrapping_mul(STRIDE)), want);
            if last == 0 {
                return 1;
            }
            i += 1;
        }
        (last as u32) & 0xffff_ff00
    }
});
