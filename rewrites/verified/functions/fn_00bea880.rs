// original: 0x00bea880 event_ctor_tag81 (proposed)

/// Initialise an event/task record of kind 0x81.
///
/// `this` points to the record. Writes the kind tag at `+0x00`, the shared
/// tick counter (file address `0x11735A4`) at `+0x04` and the argument at
/// `+0x08`. No branches, no calls; checked with `ret: none`.
///
/// Original: thiscall, one stack word, the callee pops 4 bytes.
lf_checker_rt::export!(thiscall, rw_00bea880(this: u32, first: u32) -> u32 {
    unsafe {
        const KIND_TAG: u8 = 0x81;
        const TICK_GLOBAL: u32 = 0x11735A4;
        const OFF_KIND: u32 = 0x00;
        const OFF_TICK: u32 = 0x04;
        const OFF_FIRST: u32 = 0x08;

        let tick = lf_checker_rt::global::<u32>(TICK_GLOBAL).read_unaligned();
        ((this + OFF_KIND) as *mut u8).write(KIND_TAG);
        ((this + OFF_TICK) as *mut u32).write_unaligned(tick);
        ((this + OFF_FIRST) as *mut u32).write_unaligned(first);
        0
    }
});
