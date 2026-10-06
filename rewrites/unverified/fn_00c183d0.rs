// original: 0x00c183d0 init_wrapper_set_bit1_clear_bit0

/// Fill a record through the two-helper filler, then mark it mode 1.
///
/// Calls the filler at 0x00C18380 (stdcall: destination `a0`, source `a1`),
/// then sets bit 1 and clears bit 0 of the mode byte at `a0+3`. Returns the
/// new mode byte in `al`.
///
/// Original: 0x00C183D0 (stdcall, two stack words).
lf_checker_rt::export!(stdcall, rw_00c183d0(a0: u32, a1: u32) -> u32 {
    unsafe {
        const FILLER: u32 = 1;
        const MODE_OFF: u32 = 3;
        lf_checker_rt::callee_stdcall!(FILLER, u32, a0, a1);
        let mode = ((a0 + MODE_OFF) as *const u8).read();
        let new_mode = (mode & 0xfe) | 2;
        ((a0 + MODE_OFF) as *mut u8).write(new_mode);
        new_mode as u32
    }
});
