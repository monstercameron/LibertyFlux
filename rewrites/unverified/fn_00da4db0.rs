// original: 0x00DA4DB0 CTaskComplexShockingEvent::ctor_base (proposed)

/// Base constructor of the shocking-event task family: build the task
/// header, install this class's virtual table, build the embedded member
/// at `+0x20` and initialise it from the caller's argument.
///
/// `this` is the task storage, `init` an opaque word forwarded to the
/// member initialiser. The three callees (header constructor, member
/// constructor, member initialiser) are intercepted and answered by script.
/// Returns `this`. Original: thiscall, one stack word, callee cleanup.
lf_checker_rt::export!(thiscall, rw_00da4db0(this: u32, init: u32) -> u32 {
    unsafe {
        const HEADER_CTOR: u32 = 1;
        const MEMBER_CTOR: u32 = 2;
        const MEMBER_INIT: u32 = 3;
        const VTABLE: u32 = 0x00EEF634;
        const MEMBER_OFF: u32 = 0x20;

        lf_checker_rt::callee_thiscall!(HEADER_CTOR, u32, this);
        (this as *mut u32).write(lf_checker_rt::relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(MEMBER_CTOR, u32, this + MEMBER_OFF);
        lf_checker_rt::callee_thiscall!(MEMBER_INIT, u32, this + MEMBER_OFF, init);
        this
    }
});
