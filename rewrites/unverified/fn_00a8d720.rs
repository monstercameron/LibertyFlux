// original: 0x00a8d720 pool_alloc_array (proposed)

/// Allocate an array of `count` 0xA0-byte entries, zeroing each header.
///
/// `count` is signed: a non-positive count allocates anyway (the byte size
/// goes negative, which the allocator callee just observes) and skips the
/// fill. Each entry's first twelve bytes are zeroed unless the buffer is
/// null, in which case the fill is skipped entry by entry. Returns the
/// buffer.
///
/// Original: 0x00A8D720 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00a8d720(count: u32) -> u32 {
    unsafe {
        const CALLEE_ALLOC: u32 = 1;
        const ENTRY_SIZE: u32 = 0xa0;
        let n = count as i32;
        let bytes = (n.wrapping_mul(ENTRY_SIZE as i32)) as u32;
        let buf = lf_checker_rt::callee_cdecl!(CALLEE_ALLOC, u32, bytes);
        if n > 0 {
            let mut i = 0i32;
            while i < n {
                let entry =
                    buf.wrapping_add((i as u32).wrapping_mul(ENTRY_SIZE));
                if entry != 0 {
                    (entry as *mut u32).write_unaligned(0);
                    ((entry + 4) as *mut u32).write_unaligned(0);
                    ((entry + 8) as *mut u32).write_unaligned(0);
                }
                i += 1;
            }
        }
        buf
    }
});
