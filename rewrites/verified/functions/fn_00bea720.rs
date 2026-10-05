// original: 0x00bea720 event_ctor_tag79 (proposed)

/// Initialise an event/task record of kind 0x79 from a source descriptor.
///
/// `this` points to the record, `src` to a descriptor whose dword at `+0x14`
/// points at three dwords and whose byte at `+0x24` is a flag. Writes the
/// kind tag at `+0x00`, the shared tick counter (file address `0x11735A4`)
/// at `+0x04`, the three plain arguments at `+0x08`/`+0x0c`/`+0x10`, the flag
/// byte at `+0x01`, and the three pointed-to dwords at `+0x14`/`+0x18`/`+0x1c`.
/// No branches, no calls; checked with `ret: none`.
///
/// Original: thiscall, four stack words, the callee pops 0x10 bytes.
lf_checker_rt::export!(thiscall, rw_00bea720(this: u32, first: u32, second: u32, third: u32, src: u32) -> u32 {
    unsafe {
        const KIND_TAG: u8 = 0x79;
        const TICK_GLOBAL: u32 = 0x11735A4;
        const OFF_KIND: u32 = 0x00;
        const OFF_FLAG: u32 = 0x01;
        const OFF_TICK: u32 = 0x04;
        const OFF_FIRST: u32 = 0x08;
        const OFF_SECOND: u32 = 0x0c;
        const OFF_THIRD: u32 = 0x10;
        const OFF_TRIPLE: u32 = 0x14;
        const SRC_TRIPLE_PTR: u32 = 0x14;
        const SRC_FLAG: u32 = 0x24;

        let tick = lf_checker_rt::global::<u32>(TICK_GLOBAL).read_unaligned();
        ((this + OFF_KIND) as *mut u8).write(KIND_TAG);
        ((this + OFF_TICK) as *mut u32).write_unaligned(tick);
        ((this + OFF_FIRST) as *mut u32).write_unaligned(first);
        ((this + OFF_SECOND) as *mut u32).write_unaligned(second);
        ((this + OFF_THIRD) as *mut u32).write_unaligned(third);
        let flag = ((src + SRC_FLAG) as *const u8).read();
        ((this + OFF_FLAG) as *mut u8).write(flag);
        let triple = ((src + SRC_TRIPLE_PTR) as *const u32).read_unaligned();
        for i in 0..3u32 {
            let w = ((triple + i * 4) as *const u32).read_unaligned();
            ((this + OFF_TRIPLE + i * 4) as *mut u32).write_unaligned(w);
        }
        0
    }
});
