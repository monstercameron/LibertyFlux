// original: 0x00dcf5b0 csv_skip_records (proposed)

/// Skip `count` NUL-terminated records, then report whether input remains.
///
/// `this` points to the reader. Each iteration reads one char from the
/// next-char callee and decrements the remaining count on NUL; the end
/// callee (both thiscall) stops the loop early. Afterwards the end callee
/// is polled once more and its answer returned INVERTED: 1 while input
/// remains, 0 at the end.
///
/// Original: 0x00DCF5B0 (thiscall, one stack word, byte result).
lf_checker_rt::export!(thiscall, rw_00dcf5b0(this: u32, count: u32) -> u32 {
    unsafe {
        /// End-of-input predicate callee id.
        const AT_END: u32 = 1;
        /// Next-character callee id.
        const NEXT: u32 = 2;
        let mut left = count;
        let e: u32 = lf_checker_rt::callee_thiscall!(AT_END, u32, this);
        if (e as u8) == 0 {
            loop {
                if left == 0 {
                    break;
                }
                let c: u32 = lf_checker_rt::callee_thiscall!(NEXT, u32, this);
                if (c as u8) == 0 {
                    left = left.wrapping_sub(1);
                }
                let e: u32 = lf_checker_rt::callee_thiscall!(AT_END, u32, this);
                if (e as u8) != 0 {
                    break;
                }
            }
        }
        // Returns whether input remains (inverted end flag).
        let e: u32 = lf_checker_rt::callee_thiscall!(AT_END, u32, this);
        ((e as u8) == 0) as u32
    }
});
