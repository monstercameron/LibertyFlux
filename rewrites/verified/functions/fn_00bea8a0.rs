// original: 0x00bea8a0 event_ctor_tag82 (proposed)

/// Initialise an event/task record of kind 0x82 from a 48-byte descriptor.
///
/// `this` points to the record, `src` to six qwords. Writes the kind tag at
/// `+0x00`, the shared tick counter (file address `0x11735A4`) at `+0x04`,
/// the first argument at `+0x08` and the six qwords at `+0x0c`/`+0x14`/
/// `+0x1c`/`+0x24`/`+0x2c`/`+0x34`, then registers the record through callee 1
/// (cdecl, two words: the descriptor's first dword and `this+1`). No
/// branches; checked with `ret: none`.
///
/// Original: thiscall, two stack words, the callee pops 8 bytes.
lf_checker_rt::export!(thiscall, rw_00bea8a0(this: u32, first: u32, src: u32) -> u32 {
    unsafe {
        const KIND_TAG: u8 = 0x82;
        const TICK_GLOBAL: u32 = 0x11735A4;
        const OFF_KIND: u32 = 0x00;
        const OFF_TICK: u32 = 0x04;
        const OFF_FIRST: u32 = 0x08;
        const OFF_BLOB: u32 = 0x0c;
        const QWORDS: u32 = 6;
        const REGISTER: u32 = 1;

        let tick = lf_checker_rt::global::<u32>(TICK_GLOBAL).read_unaligned();
        ((this + OFF_KIND) as *mut u8).write(KIND_TAG);
        ((this + OFF_TICK) as *mut u32).write_unaligned(tick);
        ((this + OFF_FIRST) as *mut u32).write_unaligned(first);
        for i in 0..QWORDS {
            let lo = ((src + i * 8) as *const u32).read_unaligned();
            let hi = ((src + i * 8 + 4) as *const u32).read_unaligned();
            ((this + OFF_BLOB + i * 8) as *mut u32).write_unaligned(lo);
            ((this + OFF_BLOB + i * 8 + 4) as *mut u32).write_unaligned(hi);
        }
        let key = (src as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(REGISTER, u32, key, this.wrapping_add(1));
        0
    }
});
