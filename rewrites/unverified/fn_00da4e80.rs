// original: 0x00DA4E80 CTaskComplexShockingEventWatch::ctor (proposed)

/// Constructor of the Watch task: run the family base constructor on
/// `this` with the caller's argument, install this class's virtual table,
/// then clear the trailing state fields (+0x70, +0x74, +0x78, +0x7C). Returns `this`.
/// Original: thiscall, one stack word, callee cleanup.
lf_checker_rt::export!(thiscall, rw_00da4e80(this: u32, init: u32) -> u32 {
    unsafe {
        const BASE_CTOR: u32 = 1;
        const VTABLE: u32 = 0x00EEF694;

        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this, init);
        (this as *mut u32).write(lf_checker_rt::relocated(VTABLE));
        ((this + 0x70) as *mut u32).write(0);
        ((this + 0x74) as *mut u32).write(0);
        ((this + 0x78) as *mut u32).write(0);
        ((this + 0x7C) as *mut u16).write(0);
        this
    }
});
