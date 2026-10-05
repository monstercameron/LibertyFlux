// original: 0x00A4C760 vehicle_counter_dec (proposed)

/// Decrements the counter at `this + COUNT` unless it is already zero.
///
/// Returns the new counter value in `eax` (zero when it was already zero).
/// Wrapping is impossible: the store only happens for non-zero values.
///
/// Original: 0x00A4C760 (thiscall, no stack words), leaf, no globals.
lf_checker_rt::export!(thiscall, rw_00A4C760(this: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x12EC;
        let addr = (this + COUNT) as *mut u32;
        let v = addr.read_unaligned();
        if v == 0 {
            return 0;
        }
        addr.write_unaligned(v - 1);
        v - 1
    }
});
