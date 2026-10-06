// original: 0x005eefd0 cb_5arg_ctor

/// Construct a five-argument render callback holder in place.
///
/// `this` receives the holder's vtable pointer at `+0x00` and a masked link word
/// at `+0x04` (the old word xored with the low 14 bits of itself xored with the
/// global callback counter, which is then incremented). The render-state
/// sub-object at `this+0x24` is initialised by callee 1 (which takes no stack
/// arguments and writes its first words), then the holder takes the callback
/// pointer `cb` at `+0x08`, the dereferenced words `*p0`/`*p1` at `+0x0C`/`+0x10`,
/// the 16-byte rect `*rect` at `+0x14` (default rect words are written first,
/// then overwritten), 49 words copied from `*state_src` over `+0x24`, and the
/// flag byte `*flag` at `+0xE8`. Returns `this`.
///
/// Original: thiscall, six stack arguments, one direct callee, callee pops 0x18.
/// Stack order is `(cb, p0, p1, rect, state_src, flag)`: the state pointer comes
/// fifth and the flag pointer sixth.
lf_checker_rt::export!(thiscall, rw_005eefd0(this: u32, cb: u32, p0: u32, p1: u32, rect: u32, state_src: u32, flag: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00FE1588;
        const LINK: u32 = 0x04;
        const CB: u32 = 0x08;
        const P0: u32 = 0x0C;
        const P1: u32 = 0x10;
        const RECT: u32 = 0x14;
        const STATE: u32 = 0x24;
        const FLAG: u32 = 0xE8;
        const STATE_WORDS: usize = 0x31;
        const COUNTER: u32 = 0x010327A0;
        const LINK_MASK: u32 = 0x3FFF;
        const RX0: u32 = 0x49742400;
        const RX1: u32 = 0xC9742400;
        let counter = (lf_checker_rt::global::<u32>(COUNTER)).read_unaligned();
        let link = ((this + LINK) as *const u32).read_unaligned();
        ((this) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((this + LINK) as *mut u32).write_unaligned(link ^ ((link ^ counter) & LINK_MASK));
        (lf_checker_rt::global::<u32>(COUNTER)).write_unaligned(counter.wrapping_add(1));
        ((this + RECT) as *mut u32).write_unaligned(RX0);
        ((this + RECT + 4) as *mut u32).write_unaligned(RX1);
        ((this + RECT + 8) as *mut u32).write_unaligned(RX1);
        ((this + RECT + 12) as *mut u32).write_unaligned(RX0);
        lf_checker_rt::callee_thiscall!(1, u32, this + STATE);
        ((this + CB) as *mut u32).write_unaligned(cb);
        ((this + P0) as *mut u32).write_unaligned((p0 as *const u32).read_unaligned());
        ((this + P1) as *mut u32).write_unaligned((p1 as *const u32).read_unaligned());
        core::ptr::copy_nonoverlapping(rect as *const u8, (this + RECT) as *mut u8, 16);
        core::ptr::copy_nonoverlapping(state_src as *const u32, (this + STATE) as *mut u32, STATE_WORDS);
        ((this + FLAG) as *mut u8).write_unaligned((flag as *const u8).read_unaligned());
    }
    this
});
