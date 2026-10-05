// original: 0x00942840 streaming_is_state_one (proposed)

/// Report whether the streaming state word holds the value 1.
///
/// Returns 1 in `eax` when the state word reads 1, else 0. Reads one global,
/// writes nothing.
///
/// Original: 0x00942840 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_00942840() -> u32 {
    unsafe {
        const STATE: u32 = 0x011D6FC4;
        const READY: u32 = 1;
        u32::from(*lf_checker_rt::global::<u32>(STATE) == READY)
    }
});
