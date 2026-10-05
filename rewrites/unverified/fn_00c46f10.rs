// original: 0x00c46f10 ccam_indexed_getter (proposed)
/// Return one word from the embedded table, bounds-checked.
///
/// Reads index `i` from `this + INDEX` (signed) and limit `max` from
/// `this + LIMIT` (signed compare). Returns 0 when `i` is negative or
/// above `max`, otherwise the word at `this + i * 4`.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c46f10(this: u32) -> u32 {
    const INDEX: u32 = 0x78;
    const LIMIT: u32 = 0x7c;
    unsafe {
        let i = ((this + INDEX) as *const i32).read_unaligned();
        let max = ((this + LIMIT) as *const i32).read_unaligned();
        if i < 0 || i > max {
            0
        } else {
            ((this + (i as u32).wrapping_mul(4)) as *const u32).read_unaligned()
        }
    }
});
