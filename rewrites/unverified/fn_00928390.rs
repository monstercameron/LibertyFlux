// original: 0x00928390 cascade_shadow_phase_attach (proposed)

/// Attach a cascade-shadow render phase to its owner and initialise it.
///
/// Calls the base attach helper (callee 1, thiscall) with `this` and `arg`,
/// then runs the sub-object constructor (callee 2, thiscall) on
/// `this + SUB_OFF`, installs the phase vtable, and writes the three
/// initial fields: 0 at `+0x8d0`, 3 at `+0x8f4`, 1 at `+0x3c`. Returns `this`.
///
/// Original: 0x00928390 (thiscall, one stack word). Two direct calls.
lf_checker_rt::export!(thiscall, rw_00928390(this: u32, arg: u32) -> u32 {
    unsafe {
        const SUB_OFF: u32 = 0x940;
        const VTABLE: u32 = 0x00E8_66AC;
        lf_checker_rt::callee_thiscall!(1, u32, this, arg);
        let sub = this.wrapping_add(SUB_OFF);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(2, u32, sub);
        (this.wrapping_add(0x8d0) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x8f4) as *mut u32).write_unaligned(3);
        (this.wrapping_add(0x3c) as *mut u32).write_unaligned(1);
        this
    }
});
