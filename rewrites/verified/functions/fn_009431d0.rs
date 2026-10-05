// original: 0x009431d0 streaming_state_reset (proposed)

/// Reset the streaming mode, state and present flag to idle values.
///
/// Writes 1 to the mode word, 0 to the state word and 0 to the present flag
/// byte. Takes no arguments and leaves `eax` untouched.
///
/// Original: 0x009431d0 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_009431d0() -> u32 {
    unsafe {
        const MODE: u32 = 0x011D6FC8;
        const STATE: u32 = 0x011D6FC4;
        const PRESENT: u32 = 0x011D6FCC;
        lf_checker_rt::global::<u32>(MODE).write(1);
        lf_checker_rt::global::<u32>(STATE).write(0);
        lf_checker_rt::global::<u8>(PRESENT).write(0);
        0
    }
});
