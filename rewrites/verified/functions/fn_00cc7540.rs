// original: 0x00CC7540 euphoria_widget_ctor_default (proposed)

/// Default constructor: run the base constructor, then plant vtables and state.
///
/// Calls the base constructor on `this` first, then writes the primary
/// vtable at `+0`, the secondary vtable at `+0x50`, the mode constant 13 at
/// `+0xa0`, and zero at `+0xa4` and `+0xc`. Returns `this`.
///
/// Original: 0x00CC7540 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00cc7540(this: u32) -> u32 {
    unsafe {
        const BASE_CALLEE: u32 = 1;
        const VTABLE_MAIN: u32 = 0x00ED9814;
        const VTABLE_SUB: u32 = 0x00ED991C;
        const SUB_VTABLE_AT: u32 = 0x50;
        const MODE_AT: u32 = 0xa0;
        const DEFAULT_MODE: u32 = 13;
        const STATE_AT: u32 = 0xa4;
        const TAG_AT: u32 = 0x0c;
        lf_checker_rt::callee_thiscall!(BASE_CALLEE, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_MAIN));
        (this.wrapping_add(SUB_VTABLE_AT) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(VTABLE_SUB));
        (this.wrapping_add(MODE_AT) as *mut u32).write_unaligned(DEFAULT_MODE);
        (this.wrapping_add(STATE_AT) as *mut u32).write_unaligned(0);
        (this.wrapping_add(TAG_AT) as *mut u32).write_unaligned(0);
        this
    }
});
