// original: 0x00DA4EC0 CTaskComplexShockingEvent::dtor_base (proposed)

/// Base destructor of the shocking-event task family: install this class's
/// virtual table, destroy the embedded member at `+0x20`, then tail-jump to
/// the task-header destructor with `this` still in ECX.
///
/// The tail jump is expressed as a call whose answer is returned: the
/// checker intercepts the jump target the same way. Original: thiscall, no
/// stack arguments.
lf_checker_rt::export!(thiscall, rw_00da4ec0(this: u32) -> u32 {
    unsafe {
        const MEMBER_DTOR: u32 = 1;
        const HEADER_DTOR: u32 = 2;
        const VTABLE: u32 = 0x00EEF634;
        const MEMBER_OFF: u32 = 0x20;

        (this as *mut u32).write(lf_checker_rt::relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(MEMBER_DTOR, u32, this + MEMBER_OFF);
        lf_checker_rt::callee_thiscall!(HEADER_DTOR, u32, this)
    }
});
