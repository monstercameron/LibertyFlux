// original: 0x00D4F7A0 clear_task_link_and_set_flag
/// Clear the record's link word at `+0x1c`, set its one-byte state at `+0x2c`
/// to one, and return the record pointer. The first cdecl argument is unused;
/// the record pointer is the second stack argument.
lf_checker_rt::export!(cdecl, rw_00d4f7a0(_context: u32, record: u32) -> u32 {
    const LINK_WORD: u32 = 0x1c;
    const STATE_BYTE: u32 = 0x2c;

    unsafe {
        ((record + LINK_WORD) as *mut u32).write_unaligned(0);
        ((record + STATE_BYTE) as *mut u8).write(1);
        record
    }
});
