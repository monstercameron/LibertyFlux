// original: 0x00b51040 event_list_init (proposed)

/// Initialise an event-list header for a new generation.
///
/// `this` points to the header: generation tag at `+0x04` (cleared),
/// generation value at `+0x08` (set to `gen`), state at `+0x10` (set to 1,
/// meaning populated). Returns `gen` (the original leaves its argument in
/// the return register).
///
/// Original: 0x00b51040 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b51040(this: u32, gen: u32) -> u32 {
    unsafe {
        const TAG: u32 = 0x04;
        const GENERATION: u32 = 0x08;
        const STATE: u32 = 0x10;
        const STATE_POPULATED: u32 = 1;
        ((this + TAG) as *mut u32).write_unaligned(0);
        ((this + GENERATION) as *mut u32).write_unaligned(gen);
        ((this + STATE) as *mut u32).write_unaligned(STATE_POPULATED);
        gen
    }
});
