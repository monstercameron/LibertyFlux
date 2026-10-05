// original: 0x00a9cff0 stream_count_leading_live (proposed)

/// Count leading non-zero words of a 128-word run table, unless the
/// subject's gate flag is set.
///
/// `obj` points to a record with a flag word at `+0x24`; when bit 26 is set
/// the original returns its incoming `eax` untouched, which a Rust rewrite
/// cannot observe, so the contract pins the bit clear and that path never
/// runs (see the `narrowed` note in `results.json`). Otherwise the words at
/// `this + 0x8ecd8` are scanned in order and the number of leading non-zero
/// words is returned, at most 128.
///
/// Original: 0x00a9cff0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a9cff0(this: u32, obj: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0x24;
        const GATE_BIT: u32 = 1 << 26;
        const WORDS_OFF: u32 = 0x8ecd8;
        const WORDS_MAX: u32 = 0x80;
        if ((obj + FLAG_OFF) as *const u32).read_unaligned() & GATE_BIT != 0 {
            // Dead in the contract (gate bit pinned clear): the original
            // returns entry eax here. A defined wrong value, never a panic
            // (a panicking rewrite wedges the worker instead of failing).
            return 0xdead_beef;
        }
        let mut n = 0u32;
        while n < WORDS_MAX {
            if ((this + WORDS_OFF + n.wrapping_mul(4)) as *const u32).read_unaligned() == 0 {
                break;
            }
            n += 1;
        }
        n
    }
});
