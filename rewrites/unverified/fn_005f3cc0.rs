// original: 0x005F3CC0 bitarray_test (proposed)

/// Test one bit of a shared bit array selected by an inner object.
///
/// `this + 0x64` points at an inner object whose descriptor at `+ 0x10c`
/// holds the word array base at its first word. Bit `index & 31` of word
/// `index >> 5` (an arithmetic shift, matching the original's `sar`) decides
/// the result. A null inner object reads as clear. The original is thiscall
/// with one stack argument, returning the result in AL.
lf_checker_rt::export!(thiscall, rw_005f3cc0(this: u32, index: u32) -> u32 {
    unsafe {
        const INNER_OFFSET: u32 = 0x64;
        const DESC_OFFSET: u32 = 0x10c;
        let inner =
            (this.wrapping_add(INNER_OFFSET) as *const u32).read_unaligned();
        if inner == 0 {
            return 0;
        }
        let desc =
            (inner.wrapping_add(DESC_OFFSET) as *const u32).read_unaligned();
        let words = (desc as *const u32).read_unaligned();
        let bit = index & 31;
        let word = (index as i32 >> 5) as u32;
        let w = (words.wrapping_add(word.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        u32::from(w & (1u32 << bit) != 0)
    }
});
