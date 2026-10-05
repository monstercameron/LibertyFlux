// original: 0x00bea6d0 event_ctor_tag72 (proposed)

/// Initialise an event/task record of kind 0x72.
///
/// `this` points to the record. Writes the kind tag at `+0x00`, a snapshot of
/// the shared tick counter (dword at file address `0x11735A4`) at `+0x04`,
/// the first argument at `+0x0c` and the second at `+0x08`. No branches, no
/// calls; the original leaves its last load in `eax` but the value is not a
/// result (checked with `ret: none`).
///
/// Original: thiscall, two stack words, the callee pops 8 bytes.
lf_checker_rt::export!(thiscall, rw_00bea6d0(this: u32, first: u32, second: u32) -> u32 {
    unsafe {
        const KIND_TAG: u8 = 0x72;
        const TICK_GLOBAL: u32 = 0x11735A4;
        const OFF_KIND: u32 = 0x00;
        const OFF_TICK: u32 = 0x04;
        const OFF_SECOND: u32 = 0x08;
        const OFF_FIRST: u32 = 0x0c;

        let tick = lf_checker_rt::global::<u32>(TICK_GLOBAL).read_unaligned();
        ((this + OFF_KIND) as *mut u8).write(KIND_TAG);
        ((this + OFF_TICK) as *mut u32).write_unaligned(tick);
        ((this + OFF_SECOND) as *mut u32).write_unaligned(second);
        ((this + OFF_FIRST) as *mut u32).write_unaligned(first);
        0
    }
});
