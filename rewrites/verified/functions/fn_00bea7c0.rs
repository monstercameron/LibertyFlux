// original: 0x00bea7c0 event_ctor_tag7b (proposed)

/// Initialise an event/task record of kind 0x7b.
///
/// `this` points to the record. Writes the kind tag at `+0x00`, the shared
/// tick counter (file address `0x11735A4`) at `+0x04`, the first four
/// arguments at `+0x08`/`+0x0c`/`+0x10`/`+0x14`, and the low byte of the fifth
/// argument at `+0x01`. No branches, no calls; checked with `ret: none`.
///
/// Original: thiscall, five stack words, the callee pops 0x14 bytes.
lf_checker_rt::export!(thiscall, rw_00bea7c0(this: u32, first: u32, second: u32, third: u32, fourth: u32, fifth: u32) -> u32 {
    unsafe {
        const KIND_TAG: u8 = 0x7b;
        const TICK_GLOBAL: u32 = 0x11735A4;
        const OFF_KIND: u32 = 0x00;
        const OFF_FLAG: u32 = 0x01;
        const OFF_TICK: u32 = 0x04;
        const OFF_FIRST: u32 = 0x08;
        const OFF_SECOND: u32 = 0x0c;
        const OFF_THIRD: u32 = 0x10;
        const OFF_FOURTH: u32 = 0x14;

        let tick = lf_checker_rt::global::<u32>(TICK_GLOBAL).read_unaligned();
        ((this + OFF_KIND) as *mut u8).write(KIND_TAG);
        ((this + OFF_TICK) as *mut u32).write_unaligned(tick);
        ((this + OFF_FIRST) as *mut u32).write_unaligned(first);
        ((this + OFF_SECOND) as *mut u32).write_unaligned(second);
        ((this + OFF_THIRD) as *mut u32).write_unaligned(third);
        ((this + OFF_FOURTH) as *mut u32).write_unaligned(fourth);
        ((this + OFF_FLAG) as *mut u8).write(fifth as u8);
        0
    }
});
