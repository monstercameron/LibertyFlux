// original: 0x0093fc30 streaming_flags_reset (proposed)

/// Reset the streaming slot state words and flags to their idle values.
///
/// Writes -1 to the two slot state words and clears three flag bytes. Takes
/// no arguments, reads nothing, returns nothing.
///
/// Original: 0x0093fc30 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_0093fc30() -> u32 {
    unsafe {
        const SLOT_A: u32 = 0x01036F14;
        const SLOT_B: u32 = 0x01036F18;
        const FLAG_A: u32 = 0x011A890A;
        const FLAG_B: u32 = 0x011A4FB7;
        const FLAG_C: u32 = 0x011A890B;
        const IDLE: u32 = 0xFFFF_FFFF;
        lf_checker_rt::global::<u32>(SLOT_A).write(IDLE);
        lf_checker_rt::global::<u32>(SLOT_B).write(IDLE);
        lf_checker_rt::global::<u8>(FLAG_A).write(0);
        lf_checker_rt::global::<u8>(FLAG_B).write(0);
        lf_checker_rt::global::<u8>(FLAG_C).write(0);
        0
    }
});
