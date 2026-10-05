// original: 0x00aa07e0 stream_pair_table_append (proposed)

/// Append a word pair to a 128-entry table unless it is full.
///
/// `this` points to an object with a signed count at `+0x70c0` and pairs of
/// words at `+0x70c4`, one pair per 8 bytes. When the count is already 128
/// or more the original returns its incoming `eax` untouched, which a Rust
/// rewrite cannot observe; the contract pins the count below capacity so
/// that path never runs (see the `narrowed` note in `results.json`).
/// Otherwise `first` is stored at slot `count`, the count is re-read,
/// `second` is stored beside it, the count is incremented, and `second` is
/// returned.
///
/// Original: 0x00aa07e0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00aa07e0(this: u32, first: u32, second: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0x70c0;
        const PAIRS_OFF: u32 = 0x70c4;
        const PAIR_STRIDE: u32 = 8;
        const CAPACITY: i32 = 0x80;
        let count_ptr = (this + COUNT_OFF) as *mut u32;
        if count_ptr.read_unaligned() as i32 >= CAPACITY {
            // Dead in the contract (count pinned below capacity): the
            // original returns entry eax here. A defined wrong value, never
            // a panic (a panicking rewrite wedges the worker instead of
            // failing the trial).
            return 0xdead_beef;
        }
        let i0 = count_ptr.read_unaligned();
        ((this + PAIRS_OFF + i0.wrapping_mul(PAIR_STRIDE)) as *mut u32).write_unaligned(first);
        let i1 = count_ptr.read_unaligned();
        ((this + PAIRS_OFF + 4 + i1.wrapping_mul(PAIR_STRIDE)) as *mut u32).write_unaligned(second);
        count_ptr.write_unaligned(count_ptr.read_unaligned().wrapping_add(1));
        second
    }
});
