// original: 0x00c077a0 stream_name_table_assign (proposed)

/// Assign a value to every table slot whose name matches the given string.
///
/// `this` points to the table (`NAMES` holds the array of name pointers,
/// `COUNT` its 16-bit length, `SLOTS` the parallel value array). Each entry
/// whose name compares equal to `name` (byte-wise, two bytes at a time, up to
/// the terminator) has its slot set to `value`. Returns the entry count.
///
/// Original: 0x00c077a0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00c077a0(this: u32, name: u32, value: u32) -> u32 {
    unsafe {
        const NAMES: u32 = 0x10;
        const COUNT: u32 = 0x14;
        const SLOTS: u32 = 0x18;
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
        let names = (this.wrapping_add(NAMES) as *const u32).read_unaligned();
        let slots = (this.wrapping_add(SLOTS) as *const u32).read_unaligned();
        let mut i = 0u32;
        while i < count {
            let s = (names.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            if equal(s, name) {
                (slots.wrapping_add(i.wrapping_mul(4)) as *mut u32).write_unaligned(value);
            }
            i += 1;
        }
        count
    }
});
