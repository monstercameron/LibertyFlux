// original: 0x00CC5F90 list_append_link (proposed)

/// Link `value` onto the tail of a small list header and bump its count.
///
/// `this` points to a 12-byte header: word `+0` is the first value (filled in
/// when still zero), word `+4` is the previous tail node or null, word `+8`
/// is the entry count. The previous tail, when non-null, has its own `+4`
/// word overwritten with `value`. Returns `value` (the original leaves it in
/// the return register).
///
/// Original: 0x00CC5F90 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00cc5f90(this: u32, value: u32) -> u32 {
    unsafe {
        const FIRST: u32 = 0;
        const PREV_TAIL: u32 = 4;
        const COUNT: u32 = 8;
        let prev = (this.wrapping_add(PREV_TAIL) as *const u32).read_unaligned();
        (this.wrapping_add(PREV_TAIL) as *mut u32).write_unaligned(value);
        if prev != 0 {
            (prev.wrapping_add(PREV_TAIL) as *mut u32).write_unaligned(value);
        }
        let count = (this.wrapping_add(COUNT) as *const u32).read_unaligned();
        (this.wrapping_add(COUNT) as *mut u32).write_unaligned(count.wrapping_add(1));
        if (this.wrapping_add(FIRST) as *const u32).read_unaligned() == 0 {
            (this.wrapping_add(FIRST) as *mut u32).write_unaligned(value);
        }
        value
    }
});
