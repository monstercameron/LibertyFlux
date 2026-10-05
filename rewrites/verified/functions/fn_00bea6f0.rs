// original: 0x00bea6f0 event_ctor_tag73 (proposed)

/// Initialise an event/task record of kind 0x73.
///
/// `this` points to the record. Writes the kind tag at `+0x00`, the shared
/// tick counter (dword at file address `0x11735A4`) at `+0x04`, the first
/// argument at `+0x08`, the third at `+0x0c` and the second argument's bits
/// (moved as a float, value untouched) at `+0x10`. No branches, no calls;
/// checked with `ret: none`.
///
/// Original: thiscall, three stack words, the callee pops 0xc bytes.
lf_checker_rt::export!(thiscall, rw_00bea6f0(this: u32, first: u32, second_bits: u32, third: u32) -> u32 {
    unsafe {
        const KIND_TAG: u8 = 0x73;
        const TICK_GLOBAL: u32 = 0x11735A4;
        const OFF_KIND: u32 = 0x00;
        const OFF_TICK: u32 = 0x04;
        const OFF_FIRST: u32 = 0x08;
        const OFF_THIRD: u32 = 0x0c;
        const OFF_SECOND: u32 = 0x10;

        let tick = lf_checker_rt::global::<u32>(TICK_GLOBAL).read_unaligned();
        ((this + OFF_KIND) as *mut u8).write(KIND_TAG);
        ((this + OFF_TICK) as *mut u32).write_unaligned(tick);
        ((this + OFF_FIRST) as *mut u32).write_unaligned(first);
        ((this + OFF_THIRD) as *mut u32).write_unaligned(third);
        ((this + OFF_SECOND) as *mut u32).write_unaligned(second_bits);
        0
    }
});
