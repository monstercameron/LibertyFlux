// original: 0x00bea910 event_ctor_tag83_cond (proposed)

/// Initialise an event/task record of kind 0x83, with one conditional field.
///
/// `this` points to the record, `src` to a descriptor. Always writes the kind
/// tag at `+0x00`, the shared tick counter (file address `0x11735A4`) at
/// `+0x04` and the first argument at `+0x08`. Only when bit 0x10 of the byte
/// at `src+0x2d` is set, copies the dword at `src+0x20` to `+0x0c`. No calls;
/// checked with `ret: none`.
///
/// Original: thiscall, two stack words, the callee pops 8 bytes.
lf_checker_rt::export!(thiscall, rw_00bea910(this: u32, first: u32, src: u32) -> u32 {
    unsafe {
        const KIND_TAG: u8 = 0x83;
        const TICK_GLOBAL: u32 = 0x11735A4;
        const OFF_KIND: u32 = 0x00;
        const OFF_TICK: u32 = 0x04;
        const OFF_FIRST: u32 = 0x08;
        const OFF_COND: u32 = 0x0c;
        const SRC_FLAGS: u32 = 0x2d;
        const SRC_VALUE: u32 = 0x20;
        const FLAG_MASK: u8 = 0x10;

        let tick = lf_checker_rt::global::<u32>(TICK_GLOBAL).read_unaligned();
        ((this + OFF_KIND) as *mut u8).write(KIND_TAG);
        ((this + OFF_TICK) as *mut u32).write_unaligned(tick);
        ((this + OFF_FIRST) as *mut u32).write_unaligned(first);
        if ((src + SRC_FLAGS) as *const u8).read() & FLAG_MASK != 0 {
            let v = ((src + SRC_VALUE) as *const u32).read_unaligned();
            ((this + OFF_COND) as *mut u32).write_unaligned(v);
        }
        0
    }
});
