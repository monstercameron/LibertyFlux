// original: 0x00DA2E60 move_16_byte_records

/// Copies a positive signed count of 16-byte blocks from the first pointer to
/// the third pointer. The signed block count is `(end - start) >> 4`; values
/// whose result is zero or negative do nothing. Each block is transferred as
/// two ordered 64-bit volatile loads and stores, preserving the forward-copy
/// behavior when source and destination ranges overlap.
lf_checker_rt::export!(cdecl, rw_00da2e60(start: u32, end: u32, destination: u32) -> () {
    const BLOCK_SHIFT: u32 = 4;
    const BLOCK_BYTES: u32 = 1 << BLOCK_SHIFT;
    const QWORD_BYTES: u32 = 8;

    let signed_span = end.wrapping_sub(start) as i32;
    let block_count = signed_span >> BLOCK_SHIFT;
    if block_count <= 0 { return; }

    let mut offset = 0u32;
    for _ in 0..block_count as u32 {
        let first_source = start.wrapping_add(offset);
        let first_destination = destination.wrapping_add(offset);
        let first_word = unsafe { (first_source as *const u64).read_volatile() };
        unsafe { (first_destination as *mut u64).write_volatile(first_word); }

        let second_source = first_source.wrapping_add(QWORD_BYTES);
        let second_destination = first_destination.wrapping_add(QWORD_BYTES);
        let second_word = unsafe { (second_source as *const u64).read_volatile() };
        unsafe { (second_destination as *mut u64).write_volatile(second_word); }

        offset = offset.wrapping_add(BLOCK_BYTES);
    }
});
