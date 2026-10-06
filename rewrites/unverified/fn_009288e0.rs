// original: 0x009288E0 cascade_shadow_phase_teardown_tail (proposed)

/// Tear down a cascade-shadow render phase, then tail-call the base teardown.
///
/// Installs the phase vtable on `this`, runs the sub-object teardown
/// (callee 1, thiscall) on `this + SUB_OFF`, and tail-calls the base
/// teardown (callee 2, thiscall) with `this`, returning its result.
///
/// Original: 0x009288E0 (thiscall, no stack arguments). One direct call
/// plus an E9 tail jump (patched as a tail call).
lf_checker_rt::export!(thiscall, rw_009288E0(this: u32) -> u32 {
    unsafe {
        const SUB_OFF: u32 = 0x940;
        const VTABLE: u32 = 0x00E8_66AC;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(1, u32, this.wrapping_add(SUB_OFF));
        lf_checker_rt::callee_thiscall!(2, u32, this)
    }
});
