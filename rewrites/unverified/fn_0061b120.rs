// original: 0x0061B120 net_drain_pair_stack

/// Drain a stack of word pairs, submitting each to the pair handler.
///
/// `this` points at a count word followed by 8-byte pairs. While the
/// count is nonzero it is decremented and the top pair is handed to the
/// handler (second word first on the stack). Afterwards the count is
/// decremented once more, so an empty stack ends at -1. The declared
/// return channel is the full accumulator: the handler's last answer
/// when a pair ran, the untouched entry register otherwise (the
/// contract pins that register to zero).
/// Original: 0x0061B120 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_0061B120(this: u32) -> u32 {
    unsafe {
        const DRAIN_CALLEE: u32 = 1;
        const PAIR_BYTES: u32 = 8;
        let count_at = this as *mut u32;
        let mut answer = 0u32;
        while count_at.read_unaligned() != 0 {
            let left = count_at.read_unaligned().wrapping_sub(1);
            count_at.write_unaligned(left);
            let pair = this + 4 + left.wrapping_mul(PAIR_BYTES);
            let hi = ((pair + 4) as *const u32).read_unaligned();
            let lo = (pair as *const u32).read_unaligned();
            answer = lf_checker_rt::callee_cdecl!(DRAIN_CALLEE, u32, lo, hi);
        }
        count_at.write_unaligned(count_at.read_unaligned().wrapping_sub(1));
        answer
    }
});
