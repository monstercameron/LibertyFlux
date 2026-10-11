// original: 0x00AE9100 fill_small_timer_slots

/// Initializes a 30-entry timer-slot block. It stores the entry count in the
/// first and third bytes, then writes `0xff` to every other byte beginning
/// four bytes into the block.
///
/// Calling convention: thiscall with the block pointer in ECX. The block is a
/// byte layout and remains independently addressed from Rust pointer width.
lf_checker_rt::export!(thiscall, rw_00ae9100(block: u32) -> () {
    unsafe {
        const ENTRY_COUNT: u32 = 30;
        (block.wrapping_add(2) as *mut u8).write_unaligned(ENTRY_COUNT as u8);
        (block as *mut u8).write_unaligned(ENTRY_COUNT as u8);
        let mut index = 0u32;
        while index < ENTRY_COUNT {
            (block.wrapping_add(4).wrapping_add(index.wrapping_mul(2)) as *mut u8)
                .write_unaligned(0xff);
            index = index.wrapping_add(1);
        }
    }
});
