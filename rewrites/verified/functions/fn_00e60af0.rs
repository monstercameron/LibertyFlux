// original: 0x00E60AF0 timer_table_clear_128 (proposed)

/// Fixed-table clear: resets 128 eight-byte records and one flag byte.
///
/// Behaviour: for 128 records starting at `TABLE`, writes -1 to the head
/// dword and 0 to the following 16-bit halfword, advancing 8 bytes per
/// record; then writes 0 to the flag byte at `FLAG`. No calls. Returns
/// the address one past the last record (the loop cursor at exit).
///
/// Original: no stack arguments; the loop counter runs 127 down to -1
/// (128 iterations: the body runs once more after the counter reaches 0,
/// the sign flag only ends the loop). Only EAX/ECX/EDX are clobbered.
lf_checker_rt::export!(cdecl, rw_00e60af0() -> u32 {
    unsafe {
        const RECORDS: u32 = 128;
        const EMPTY_ID: u32 = 0xFFFF_FFFF;
        let mut slot: u32 = lf_checker_rt::relocated(0x001BB3C24);
        let mut left = RECORDS;
        while left > 0 {
            (slot as *mut u32).write_unaligned(EMPTY_ID);
            ((slot + 4) as *mut u16).write_unaligned(0);
            slot = slot.wrapping_add(8);
            left -= 1;
        }
        (lf_checker_rt::relocated(0x001BB3C1D) as *mut u8).write_unaligned(0);
        slot
    }
});

