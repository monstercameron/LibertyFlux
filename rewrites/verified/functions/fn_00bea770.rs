// original: 0x00bea770 event_ctor_tag7a (proposed)

/// Initialise an event/task record of kind 0x7a from inline values and a triple.
///
/// `this` points to the record, `src` to three dwords. Writes the kind tag at
/// `+0x00`, the shared tick counter (file address `0x11735A4`) at `+0x04`,
/// the three plain arguments at `+0x08`/`+0x0c`/`+0x10`, the two float-bit
/// arguments untouched at `+0x14`/`+0x18`, and the pointed-to triple at
/// `+0x1c`/`+0x20`/`+0x24`. No branches, no calls; checked with `ret: none`.
///
/// Original: thiscall, six stack words, the callee pops 0x18 bytes.
lf_checker_rt::export!(thiscall, rw_00bea770(this: u32, first: u32, second: u32, third: u32, fourth_bits: u32, fifth_bits: u32, src: u32) -> u32 {
    unsafe {
        const KIND_TAG: u8 = 0x7a;
        const TICK_GLOBAL: u32 = 0x11735A4;
        const OFF_KIND: u32 = 0x00;
        const OFF_TICK: u32 = 0x04;
        const OFF_FIRST: u32 = 0x08;
        const OFF_SECOND: u32 = 0x0c;
        const OFF_THIRD: u32 = 0x10;
        const OFF_FOURTH: u32 = 0x14;
        const OFF_FIFTH: u32 = 0x18;
        const OFF_TRIPLE: u32 = 0x1c;

        let tick = lf_checker_rt::global::<u32>(TICK_GLOBAL).read_unaligned();
        ((this + OFF_KIND) as *mut u8).write(KIND_TAG);
        ((this + OFF_TICK) as *mut u32).write_unaligned(tick);
        ((this + OFF_FIRST) as *mut u32).write_unaligned(first);
        ((this + OFF_SECOND) as *mut u32).write_unaligned(second);
        ((this + OFF_THIRD) as *mut u32).write_unaligned(third);
        ((this + OFF_FOURTH) as *mut u32).write_unaligned(fourth_bits);
        ((this + OFF_FIFTH) as *mut u32).write_unaligned(fifth_bits);
        for i in 0..3u32 {
            let w = ((src + i * 4) as *const u32).read_unaligned();
            ((this + OFF_TRIPLE + i * 4) as *mut u32).write_unaligned(w);
        }
        0
    }
});
