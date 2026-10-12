// original: 0x0087F750 rage::miniheapAllocator::get_bytes_remaining

/// Return the allocator's remaining byte capacity through TLS slot zero.
///
/// The context's capacity is the word at offset `0x4c`, and the current
/// cursor is at `0x30`. Subtraction wraps at 32 bits, matching the allocator
/// counters even when synthetic inputs put the cursor beyond its capacity.
lf_checker_rt::export!(thiscall, rw_0087f750(_this: u32) -> u32 {
    unsafe {
        const CURSOR_OFFSET: u32 = 0x30;
        const CAPACITY_OFFSET: u32 = 0x4C;
        let context = lf_checker_rt::tls_slot(0);
        let capacity = (context.wrapping_add(CAPACITY_OFFSET) as *const u32).read_unaligned();
        let cursor = (context.wrapping_add(CURSOR_OFFSET) as *const u32).read_unaligned();
        capacity.wrapping_sub(cursor)
    }
});
