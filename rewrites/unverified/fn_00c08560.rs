// original: 0x00c08560 stream_record_activate (proposed)

/// Run the membership test for the argument, then activate its record.
///
/// `this` points to the set (records at `RECORDS`, 16-bit length at `COUNT`,
/// each record `STRIDE` bytes starting with a key) and the wanted key is the
/// string at `KEY_OFF` past `arg`. The tester (callee 1) runs first: a zero
/// low byte in its answer returns that whole answer at once. Otherwise the
/// set is searched the same way the tester searches it; an empty set returns
/// the tester's answer with its low byte cleared, a record found at index `i`
/// is handed to the activator (callee 2) with that index and the activator's
/// answer comes back with its low byte forced to 1, and a set with no match
/// returns the last comparison's sign with its low byte cleared (0, or all
/// high bits set when the last record's key sorted below).
///
/// Original: 0x00c08560 (thiscall, one stack word; both callees thiscall).
lf_checker_rt::export!(thiscall, rw_00c08560(this: u32, arg: u32) -> u32 {
    unsafe {
        const RECORDS: u32 = 0x00;
        const COUNT: u32 = 0x04;
        const STRIDE: u32 = 0x28;
        const KEY_OFF: u32 = 0x2c;
        const TEST: u32 = 1;
        const ACTIVATE: u32 = 2;
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
        let t: u32 = lf_checker_rt::callee_thiscall!(TEST, u32, this, arg);
        if t & 0xff == 0 {
            return t;
        }
        let count = (this.wrapping_add(COUNT) as *const u16).read_unaligned() as u32;
        if (count as i32) <= 0 {
            return t & 0xffff_ff00;
        }
        let base = (this.wrapping_add(RECORDS) as *const u32).read_unaligned();
        let want = arg.wrapping_add(KEY_OFF);
        let mut last = 0i32;
        let mut i = 0u32;
        while i < count {
            last = compare(base.wrapping_add(i.wrapping_mul(STRIDE)), want);
            if last == 0 {
                let r: u32 = lf_checker_rt::callee_thiscall!(ACTIVATE, u32, this, i);
                return (r & 0xffff_ff00) | 1;
            }
            i += 1;
        }
        (last as u32) & 0xffff_ff00
    }
});
