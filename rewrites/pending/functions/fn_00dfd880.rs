// original: 0x00dfd880 memswap_bytes
// rs03f08: byte-block swap between two buffers (cdecl/3).
//
// Exchanges `n` bytes between `a` and `b`, one byte at a time. Returns the
// address one past the last byte swapped on the `b` side (that is, `b + n`,
// or `b` when the buffers are identical or the count is zero).
export!(cdecl, rw_rs03f08(a: *mut u8, b: *mut u8, n: u32) -> u32 {
    unsafe {
        if a == b {
            return b as u32;
        }
        let mut p = b;
        let mut q = a;
        let mut left = n;
        while left != 0 {
            let tmp = *p;
            *p = *q;
            *q = tmp;
            p = p.add(1);
            q = q.add(1);
            left -= 1;
        }
        p as u32
    }
});
