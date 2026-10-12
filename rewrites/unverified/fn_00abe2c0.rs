// original: 0x00ABE2C0 bounds_collector_init

/// Allocate and clear a contiguous array of three word-sized bounds values.
///
/// `this` arrives in ECX but is unused. The signed item count is the sole
/// stack argument. The initializer requests 12 bytes per item from the
/// allocator. For a positive count and non-null allocation it clears each
/// item's three words to zero; otherwise it performs no stores. It returns
/// the allocator's pointer and uses the thiscall convention, whose callee
/// removes the count argument.
lf_checker_rt::export!(thiscall, rw_00abe2c0(_this: u32, count: i32) -> u32 {
    const ALLOCATOR_ID: u32 = 1;
    const WORDS_PER_ITEM: u32 = 3;
    const BYTES_PER_WORD: u32 = 4;

    let allocation_bytes = (count as u32).wrapping_mul(WORDS_PER_ITEM * BYTES_PER_WORD);
    let allocation = lf_checker_rt::callee_cdecl!(ALLOCATOR_ID, u32, allocation_bytes);

    if count > 0 && allocation != 0 {
        let base = allocation as *mut u32;
        for index in 0..count as u32 {
            let item = unsafe { base.add((index * WORDS_PER_ITEM) as usize) };
            unsafe {
                item.write_unaligned(0);
                item.add(1).write_unaligned(0);
                item.add(2).write_unaligned(0);
            }
        }
    }

    allocation
});
