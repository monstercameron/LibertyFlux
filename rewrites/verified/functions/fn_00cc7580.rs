// original: 0x00CC7580 euphoria_controller_ctor (proposed)

/// Controller constructor: base plus two embedded sub-objects, then state.
///
/// Calls the base constructor on `this` with (`b1`, `b2`, `b3`), plants the
/// primary and secondary vtables, constructs the sub-object at `+0x100` and
/// the zero-block at `+0x2a0`, then stores `a0` at `+0xb0`, -1 at `+0xb4` and
/// `+0xb8`, zero at `+0xc`, a back-pointer to `this` at `+0x290`, and a zero
/// half-word at `+0x2e0`. Returns `this`.
///
/// Original: 0x00CC7580 (thiscall, four stack words).
lf_checker_rt::export!(thiscall, rw_00cc7580(this: u32, a0: u32, b1: u32, b2: u32, b3: u32) -> u32 {
    unsafe {
        const BASE_CALLEE: u32 = 1;
        const SUB1_CALLEE: u32 = 2;
        const SUB2_CALLEE: u32 = 3;
        const VTABLE_MAIN: u32 = 0x00ED994C;
        const VTABLE_SUB: u32 = 0x00ED9A58;
        const SUB_VTABLE_AT: u32 = 0x50;
        const SUB1_AT: u32 = 0x100;
        const SUB2_AT: u32 = 0x2a0;
        const ARG_AT: u32 = 0xb0;
        const INVALID_AT: [u32; 2] = [0xb4, 0xb8];
        const TAG_AT: u32 = 0x0c;
        const BACKLINK_AT: u32 = 0x290;
        const TAIL_HALF_AT: u32 = 0x2e0;
        lf_checker_rt::callee_thiscall!(BASE_CALLEE, u32, this, b1, b2, b3);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_MAIN));
        (this.wrapping_add(SUB_VTABLE_AT) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(VTABLE_SUB));
        lf_checker_rt::callee_thiscall!(SUB1_CALLEE, u32, this.wrapping_add(SUB1_AT));
        lf_checker_rt::callee_thiscall!(SUB2_CALLEE, u32, this.wrapping_add(SUB2_AT));
        (this.wrapping_add(ARG_AT) as *mut u32).write_unaligned(a0);
        for off in INVALID_AT {
            (this.wrapping_add(off) as *mut u32).write_unaligned(0xFFFF_FFFF);
        }
        (this.wrapping_add(TAG_AT) as *mut u32).write_unaligned(0);
        (this.wrapping_add(BACKLINK_AT) as *mut u32).write_unaligned(this);
        (this.wrapping_add(TAIL_HALF_AT) as *mut u16).write_unaligned(0);
        this
    }
});
