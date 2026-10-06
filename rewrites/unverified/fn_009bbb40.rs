// original: 0x009bbb40 clear_input_slot_flags (proposed)

/// Clear the flag byte of each of the 26 input slots.
///
/// Slot `i` starts at 0x0128E960 + `i` * 0x40; its flag is the first byte.
/// Walks all 26 slots writing 0 and returns the table end 0x0128F5E0.
///
/// Edge cases: none; the table bounds are fixed.
///
/// Original: no arguments, returns the end address in EAX.
lf_checker_rt::export!(cdecl, rw_009bbb40() -> u32 {
    unsafe {
        const TABLE: u32 = 0x0128e960;
        const END: u32 = 0x0128f5e0;
        const STRIDE: u32 = 0x40;
        let end = lf_checker_rt::relocated(END);
        let mut cur = lf_checker_rt::relocated(TABLE);
        while cur < end {
            (cur as *mut u8).write(0);
            cur += STRIDE;
        }
        cur
    }
});
