// original: 0x00c078c0 stream_record_lookup (proposed)

/// Find the record whose leading key matches the given string.
///
/// `this` points to the set (`RECORDS` holds the array, `COUNT` its 16-bit
/// length, each record `STRIDE` bytes starting with a NUL-terminated key).
/// Returns a pointer to the first matching record, or null when none matches
/// or the count is zero or at/above 0x8000.
///
/// Original: 0x00c078c0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c078c0(this: u32, key: u32) -> u32 {
    unsafe {
        const RECORDS: u32 = 0x00;
        const COUNT: u32 = 0x04;
        const STRIDE: u32 = 0x50;
        /// Byte-wise equality in the original's read order and width.
        unsafe fn equal(mut a: u32, mut b: u32) -> bool {
            unsafe {
                loop {
                    let c1 = (a as *const u8).read();
                    let d1 = (b as *const u8).read();
                    if c1 != d1 {
                        return false;
                    }
                    if c1 == 0 {
                        return true;
                    }
                    let c2 = (a.wrapping_add(1) as *const u8).read();
                    let d2 = (b.wrapping_add(1) as *const u8).read();
                    if c2 != d2 {
                        return false;
                    }
                    a = a.wrapping_add(2);
                    b = b.wrapping_add(2);
                    if c2 == 0 {
                        return true;
                    }
                }
            }
        }
        let count = (this.wrapping_add(COUNT) as *const u16).read_unaligned() as u32;
        if (count as i32) <= 0 {
            return 0;
        }
        let base = (this.wrapping_add(RECORDS) as *const u32).read_unaligned();
        let mut i = 0u32;
        while i < count {
            let rec = base.wrapping_add(i.wrapping_mul(STRIDE));
            if equal(rec, key) {
                return rec;
            }
            i += 1;
        }
        0
    }
});
