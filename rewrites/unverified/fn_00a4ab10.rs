// original: 0x00A4AB10 vehicle_array_contains (proposed)

/// True when the non-zero argument matches one of eight words at `this + ARRAY`.
///
/// Scans `ARRAY` (0x0F54) for `count` (8) consecutive words and returns 1 on
/// the first equal word, 0 if none match. A zero argument returns 0 without
/// scanning (even if a slot holds zero).
///
/// Original: 0x00A4AB10 (thiscall, one stack word), leaf, no globals.
lf_checker_rt::export!(thiscall, rw_00A4AB10(this: u32, val: u32) -> u32 {
    unsafe {
        const ARRAY: u32 = 0x0F54;
        const COUNT: u32 = 8;
        if val == 0 {
            return 0;
        }
        let mut i = 0;
        while i < COUNT {
            let slot = ((this + ARRAY + i * 4) as *const u32).read_unaligned();
            if slot == val {
                return 1;
            }
            i += 1;
        }
        0
    }
});
