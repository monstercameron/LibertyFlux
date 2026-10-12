// original: 0x0087F730 rage::miniheapAllocator::get_bytes_allocated

/// Return the allocator's current byte count, read through TLS slot zero.
///
/// The fabricated thread-local allocator context stores the current cursor at
/// offset `0x30` and the baseline at `0x54`; the result is their wrapping
/// 32-bit difference. The unused stack word is retained for the virtual-call
/// ABI and is popped by the original.
lf_checker_rt::export!(thiscall, rw_0087f730(_this: u32, _unused: u32) -> u32 {
    unsafe {
        const CURSOR_OFFSET: u32 = 0x30;
        const BASELINE_OFFSET: u32 = 0x54;
        let context = lf_checker_rt::tls_slot(0);
        let cursor = (context.wrapping_add(CURSOR_OFFSET) as *const u32).read_unaligned();
        let baseline = (context.wrapping_add(BASELINE_OFFSET) as *const u32).read_unaligned();
        cursor.wrapping_sub(baseline)
    }
});
