// original: 0x006f50e0 validate_packed_header
/// Validate a packed header against a limit (fastcall: `this` in ECX,
/// `limit` in EDX).
///
/// Interprets bytes 0..2 as a big-endian value, derives a quotient and a
/// bit-dependent adjustment from it, and range-checks both against `limit`
/// and fixed bounds. Returns 1 when everything is consistent, else 0.
rt::export!(fastcall, rw_006f50e0(this: *const u8, limit: u32) -> u8 {
    unsafe {
        if limit < 4 {
            return 0;
        }
        let b0 = *this as u32;
        let b1 = *this.add(1) as u32;
        let b2 = *this.add(2) as u32;
        let raw = (b0 << 8) | b1;
        let bit = (b1 >> 5) & 1;
        let adjust = (bit | 2) * 2;
        let quot = raw >> 6;
        let diff = quot.wrapping_sub(adjust);
        if quot > limit {
            return 0;
        }
        if quot > 0x39A {
            return 0;
        }
        if diff > 0x394 {
            return 0;
        }
        let check = bit | ((b2 | 0x4000) >> 13);
        if quot != diff.wrapping_add(check * 2) {
            return 0;
        }
        1
    }
});
