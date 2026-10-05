// original: 0x00a532b0 vehicle_find_int (proposed)

/// Linear search: 1 if `value` occurs in the first `count` words of `array`.
///
/// `count` is signed: zero or negative means not found without reading.
/// Scans words in order and returns 1 on the first match, else 0. Cdecl,
/// three stack words, result in al (upper bytes zeroed by the xor).
lf_checker_rt::export!(cdecl, rw_00a532b0(value: u32, array: u32, count: u32) -> u32 {
    unsafe {
        if (count as i32) <= 0 {
            return 0;
        }
        let mut i: u32 = 0;
        loop {
            let w = ((array.wrapping_add(i.wrapping_mul(4))) as *const u32).read_unaligned();
            if w == value {
                return 1;
            }
            i = i.wrapping_add(1);
            if (i as i32) >= (count as i32) {
                return 0;
            }
        }
    }
});
