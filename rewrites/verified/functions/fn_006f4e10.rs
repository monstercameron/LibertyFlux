// original: 0x006f4e10 xor_decode_buffer
/// XOR-decode a length-prefixed buffer in place.
///
/// Bytes 0..2 are a big-endian length `n`. Every byte in `buf[2..n]` is
/// xored with the key stored at `buf[4]`; the key slot itself is restored
/// afterwards, so it is never changed. Returns `max(n, 2)`.
rt::export!(thiscall, rw_006f4e10(this: *mut u8) -> u32 {
    unsafe {
        let n = ((*this as u32) << 8) | (*this.add(1) as u32);
        let key = *this.add(4);
        if n > 2 {
            let mut i = 2u32;
            while i < n {
                *this.add(i as usize) ^= key;
                i += 1;
            }
            *this.add(4) = key;
            n
        } else {
            2
        }
    }
});
