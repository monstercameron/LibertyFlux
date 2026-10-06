// original: 0x005ef0c0 cb_6arg_ctor

/// Construct a six-argument render callback holder in place.
///
/// `this` receives the holder's vtable pointer at `+0x00` and a masked link word
/// at `+0x04` (the old word xored with the low 14 bits of itself xored with the
/// global callback counter, which is then incremented). The render-state
/// sub-object at `this+0x0C` is initialised by callee 1 (no stack arguments),
/// then the holder takes the callback pointer `cb` at `+0x08`, 49 words copied
/// from `*state_src` over `+0x0C`, and the dereferenced words `*f0`..`*f3` and
/// `*u` at `+0xD0`..`+0xE0`. Returns `this`.
///
/// Original: thiscall, seven stack arguments, one direct callee, callee pops 0x1C.
lf_checker_rt::export!(thiscall, rw_005ef0c0(this: u32, cb: u32, state_src: u32, f0: u32, f1: u32, f2: u32, f3: u32, u: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00FE156C;
        const LINK: u32 = 0x04;
        const CB: u32 = 0x08;
        const STATE: u32 = 0x0C;
        const STATE_WORDS: usize = 0x31;
        const TAIL: u32 = 0xD0;
        const COUNTER: u32 = 0x010327A0;
        const LINK_MASK: u32 = 0x3FFF;
        let counter = (lf_checker_rt::global::<u32>(COUNTER)).read_unaligned();
        let link = ((this + LINK) as *const u32).read_unaligned();
        ((this) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((this + LINK) as *mut u32).write_unaligned(link ^ ((link ^ counter) & LINK_MASK));
        (lf_checker_rt::global::<u32>(COUNTER)).write_unaligned(counter.wrapping_add(1));
        lf_checker_rt::callee_thiscall!(1, u32, this + STATE);
        ((this + CB) as *mut u32).write_unaligned(cb);
        core::ptr::copy_nonoverlapping(state_src as *const u32, (this + STATE) as *mut u32, STATE_WORDS);
        ((this + TAIL) as *mut u32).write_unaligned((f0 as *const u32).read_unaligned());
        ((this + TAIL + 4) as *mut u32).write_unaligned((f1 as *const u32).read_unaligned());
        ((this + TAIL + 8) as *mut u32).write_unaligned((f2 as *const u32).read_unaligned());
        ((this + TAIL + 12) as *mut u32).write_unaligned((f3 as *const u32).read_unaligned());
        ((this + TAIL + 16) as *mut u32).write_unaligned((u as *const u32).read_unaligned());
    }
    this
});
