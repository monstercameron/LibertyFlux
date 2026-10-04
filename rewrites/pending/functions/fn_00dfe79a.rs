// original: 0x00dfe79a uint_format_base
// rs03f20: unsigned integer formatter with selectable base (stdcall/4).
//
// Writes `v` into `buf` in the given `base` (digits past 9 use lowercase
// letters), prefixing a minus sign and negating first when `neg` is nonzero,
// and NUL-terminates. Digits are emitted least-significant first and then
// reversed in place.
export!(stdcall, rw_rs03f20(v: u32, buf: *mut u8, base: u32, neg: u32) -> () {
    unsafe {
        let mut p = buf;
        let mut x = v;
        if neg != 0 {
            *p = b'-';
            p = p.add(1);
            x = x.wrapping_neg();
        }
        let start = p;
        loop {
            let digit = x % base;
            *p = if digit > 9 {
                (digit as u8).wrapping_add(0x57)
            } else {
                (digit as u8).wrapping_add(0x30)
            };
            p = p.add(1);
            x /= base;
            if x == 0 {
                break;
            }
        }
        *p = 0;
        let mut lo = start;
        let mut hi = p.sub(1);
        while lo < hi {
            let tmp = *lo;
            *lo = *hi;
            *hi = tmp;
            lo = lo.add(1);
            hi = hi.sub(1);
        }
    }
});
