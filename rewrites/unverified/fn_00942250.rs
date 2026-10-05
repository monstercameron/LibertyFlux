// original: 0x00942250 streaming_counter_bump_a (proposed)

/// Advance the object's primary 2-bit round-robin counter.
///
/// Reads the counter word at `this + 0x2040`, adds one and keeps the low two
/// bits, stores it back, and returns the new value in `eax`.
///
/// Original: 0x00942250 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00942250(this: u32) -> u32 {
    unsafe {
        const COUNTER: u32 = 0x2040;
        const MASK: u32 = 3;
        let slot = (this + COUNTER) as *mut u32;
        let next = ((*slot).wrapping_add(1)) & MASK;
        *slot = next;
        next
    }
});
