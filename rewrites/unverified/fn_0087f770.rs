// original: 0x0087F770 miniheap_allocator_contains (proposed)

/// Test whether an address lies in the allocator's half-open byte range.
///
/// The lower endpoint is the word at TLS-context offset `0x54`, and the upper
/// endpoint is at `0x30`. Both comparisons are unsigned: the address is
/// contained exactly when `lower <= address < upper`. The virtual method
/// returns its boolean in AL and pops its one stack argument.
lf_checker_rt::export!(thiscall, rw_0087f770(_this: u32, address: u32) -> u32 {
    unsafe {
        const LOWER_OFFSET: u32 = 0x54;
        const UPPER_OFFSET: u32 = 0x30;
        let context = lf_checker_rt::tls_slot(0);
        let lower = (context.wrapping_add(LOWER_OFFSET) as *const u32).read_unaligned();
        let upper = (context.wrapping_add(UPPER_OFFSET) as *const u32).read_unaligned();
        u32::from(address >= lower && address < upper)
    }
});
