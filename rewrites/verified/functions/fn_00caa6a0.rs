// original: 0x00CAA6A0 slot_first_nonnull (proposed)

/// Return the first non-null slot of a small task record.
///
/// `this` points to a record holding candidate pointers at `+0x08`, `+0x04`
/// and `+0x10`, checked in that order. Returns the first candidate that is
/// non-zero, or the third slot's value (possibly null) when both earlier
/// slots are null. Reads only; no calls, no stores.
///
/// Original: 0x00CAA6A0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00caa6a0(this: u32) -> u32 {
    unsafe {
        const SLOT_FIRST: u32 = 0x08;
        const SLOT_SECOND: u32 = 0x04;
        const SLOT_THIRD: u32 = 0x10;
        let first = (this.wrapping_add(SLOT_FIRST) as *const u32).read_unaligned();
        if first != 0 {
            return first;
        }
        let second = (this.wrapping_add(SLOT_SECOND) as *const u32).read_unaligned();
        if second != 0 {
            return second;
        }
        (this.wrapping_add(SLOT_THIRD) as *const u32).read_unaligned()
    }
});
