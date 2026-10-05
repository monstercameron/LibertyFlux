// original: 0x00942b20 streaming_mode_set (proposed)

/// Store the streaming mode and mark it present.
///
/// Writes the argument to the mode word, sets the present flag byte to 1,
/// and returns the argument in `eax`.
///
/// Original: 0x00942b20 (cdecl, one stack argument).
lf_checker_rt::export!(cdecl, rw_00942b20(mode: u32) -> u32 {
    unsafe {
        const MODE: u32 = 0x011D6FC8;
        const PRESENT: u32 = 0x011D6FCC;
        lf_checker_rt::global::<u32>(MODE).write(mode);
        lf_checker_rt::global::<u8>(PRESENT).write(1);
        mode
    }
});
