// original: 0x00a9cff0 stream_claim_slot_and_notify (proposed)

/// Claim the first free word of a 128-word run table and tail-call the
/// change handler, unless the subject's gate flag is set.
///
/// `obj` points to a record with a flag word at `+0x24`; when bit 26 is set
/// the original returns its incoming `eax` untouched, which a Rust rewrite
/// cannot observe, so the contract pins the bit clear and that path never
/// runs (see the `narrowed` note in `results.json`). Otherwise the words at
/// `this + 0x8ecd8` are scanned for the first zero. When one is found at
/// index `k`, `obj` is stored there and the handler is tail-called with the
/// slot address (the object pointer travels in ECX); its answer is the
/// answer. When all 128 words are non-zero the result is 0x80 and nothing
/// is stored or called.
///
/// Original: 0x00a9cff0 (thiscall, one stack word; zero-found path ends in
/// a tail jump).
lf_checker_rt::export!(thiscall, rw_00a9cff0(this: u32, obj: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0x24;
        const GATE_BIT: u32 = 1 << 26;
        const WORDS_OFF: u32 = 0x8ecd8;
        const WORDS_MAX: u32 = 0x80;
        const TABLE_FULL: u32 = 0x80;
        const HANDLER: u32 = 1;
        if ((obj + FLAG_OFF) as *const u32).read_unaligned() & GATE_BIT != 0 {
            // Dead in the contract (gate bit pinned clear): the original
            // returns entry eax here. A defined wrong value, never a panic
            // (a panicking rewrite wedges the worker instead of failing).
            return 0xdead_beef;
        }
        let mut n = 0u32;
        while n < WORDS_MAX {
            let slot = this + WORDS_OFF + n.wrapping_mul(4);
            if ((slot) as *const u32).read_unaligned() == 0 {
                (slot as *mut u32).write_unaligned(obj);
                return lf_checker_rt::callee_thiscall!(HANDLER, u32, obj, slot);
            }
            n += 1;
        }
        TABLE_FULL
    }
});
