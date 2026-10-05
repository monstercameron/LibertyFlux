// original: 0x008C7B60 stream_marks_clear
/// Clear the sixteen streaming mark bytes and report how many.
///
/// Zeroes the mark array one byte at a time and returns 16, the loop
/// counter at exit. The bounds-check jump past the loop's `ret` is dead:
/// the counter is zeroed immediately before the check, so it never fires.
/// Makes no calls. Original: cdecl, no stack words.
lf_checker_rt::export!(cdecl, rw_008c7b60() -> u32 {
    unsafe {
        const MARKS: u32 = 0x1173258;
        const MARK_COUNT: u32 = 16;
        let base = lf_checker_rt::relocated(MARKS);
        let mut i: u32 = 0;
        while i < MARK_COUNT {
            ((base + i) as *mut u8).write(0);
            i += 1;
        }
        i
    }
});
