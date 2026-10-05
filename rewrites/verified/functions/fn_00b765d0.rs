// original: 0x00b765d0 ptr_pair_invalidate (proposed)

/// Invalidate two slots by writing -1 through two pointers.
///
/// `second` (second stack word) is written first, then `first`. Either write
/// faults when its pointer is null or unmapped. Returns the first pointer
/// with its low byte cleared: the original reloads `first` into eax and then
/// zeroes only al, so the upper 24 bits of the return value are the upper 24
/// bits of `first`.
///
/// Original: 0x00b765d0 (stdcall, two stack words).
lf_checker_rt::export!(stdcall, rw_00b765d0(first: u32, second: u32) -> u32 {
    unsafe {
        const INVALID: u32 = 0xffff_ffff;
        (second as *mut u32).write_unaligned(INVALID);
        (first as *mut u32).write_unaligned(INVALID);
        first & 0xffff_ff00
    }
});
