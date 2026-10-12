// original: 0x005F3C80 nonzero_word_scan (proposed)

/// True when any of the first `count` shared words is nonzero.
///
/// `this + 0x64` points at an inner object whose descriptor at `+ 0x10c`
/// holds the word array base at its first word and a 16-bit count at `+ 4`.
/// Scans at most `count` words (an unsigned bound) for the first nonzero one.
/// A null inner object or a zero count reads as all clear. The original is
/// thiscall with no stack arguments, returning the result in AL.
lf_checker_rt::export!(thiscall, rw_005f3c80(this: u32) -> u32 {
    unsafe {
        const INNER_OFFSET: u32 = 0x64;
        const DESC_OFFSET: u32 = 0x10c;
        const COUNT_OFFSET: u32 = 0x04;
        let inner =
            (this.wrapping_add(INNER_OFFSET) as *const u32).read_unaligned();
        if inner == 0 {
            return 0;
        }
        let desc =
            (inner.wrapping_add(DESC_OFFSET) as *const u32).read_unaligned();
        let count = (desc.wrapping_add(COUNT_OFFSET) as *const u16)
            .read_unaligned() as u32;
        if count == 0 {
            return 0;
        }
        let words = (desc as *const u32).read_unaligned();
        let mut i = 0u32;
        loop {
            let w = (words.wrapping_add(i.wrapping_mul(4)) as *const u32)
                .read_unaligned();
            if w != 0 {
                return 1;
            }
            i = i.wrapping_add(1);
            if i >= count {
                return 0;
            }
        }
    }
});
