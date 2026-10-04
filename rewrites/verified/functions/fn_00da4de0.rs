// original: 0x00DA4DE0 CTaskComplexShockingEventFlee::ctor (proposed)

/// Constructor of the Flee task: run the family base constructor on
/// `this` with the caller's argument, install this class's virtual table,
/// then clear the trailing state fields (+0x70). Returns `this`.
/// Original: thiscall, one stack word, callee cleanup.
lf_checker_rt::export!(thiscall, rw_00da4de0(this: u32, init: u32) -> u32 {
    unsafe {
        const BASE_CTOR: u32 = 1;
        const VTABLE: u32 = 0x00EEF7B4;

        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this, init);
        (this as *mut u32).write(lf_checker_rt::relocated(VTABLE));
        ((this + 0x70) as *mut u8).write(0);
        this
    }
});
