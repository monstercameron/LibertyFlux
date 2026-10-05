// original: 0x00c7a560 task_scenario_ctor_copy3

/// Construct a scenario task, then copy a 16-byte block from a linked object.
/// The three arguments forward to the base constructor callee and `VTABLE`
/// is installed. When the link at `+0x1c` is non-null, the source is the
/// alternate object at `[link+0x20]` biased by `+0x30` if that is set, else
/// the link itself biased by `+0x10`; four words are copied to `+0x30`.
/// The moves are bitwise (the original uses vector moves as copies).
/// Returns `this`.
/// Original: 0x00c7a560 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00c7a560(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00ED5C6C;
        const OFF_LINK: u32 = 0x1c;
        const OFF_ALT: u32 = 0x20;
        const ALT_BIAS: u32 = 0x30;
        const LINK_BIAS: u32 = 0x10;
        const OFF_DST: u32 = 0x30;
        const BASE_CTOR: u32 = 1;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this, a0, a1, a2);
        wr32(this, lf_checker_rt::relocated(VTABLE));
        let link = rd32(this.wrapping_add(OFF_LINK));
        if link != 0 {
            let alt = rd32(link.wrapping_add(OFF_ALT));
            let src = if alt != 0 { alt.wrapping_add(ALT_BIAS) } else { link.wrapping_add(LINK_BIAS) };
            let mut i = 0u32;
            while i < 4 {
                wr32(this.wrapping_add(OFF_DST).wrapping_add(i * 4), rd32(src.wrapping_add(i * 4)));
                i += 1;
            }
        }
        this
    }
});
