// original: 0x00DA4E00 CTaskComplexShockingEventGoto::ctor (proposed)

/// Constructor of the Goto task: run the family base constructor on
/// `this` with the caller's argument, install this class's virtual table,
/// then clear the trailing state fields, then query the member radius through the float callee and keep it at +0x80. Returns `this`.
/// Original: thiscall, one stack word, callee cleanup.
lf_checker_rt::export!(thiscall, rw_00da4e00(this: u32, init: u32) -> u32 {
    unsafe {
        const BASE_CTOR: u32 = 1;
        const RADIUS_CALLEE: u32 = 2;
        const RADIUS_SLOT: u32 = 0x80;
        const MEMBER_OFF: u32 = 0x20;
        const VTABLE: u32 = 0x00EEF6F4;

        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this, init);
        (this as *mut u32).write(lf_checker_rt::relocated(VTABLE));
        ((this + 0x70) as *mut u32).write(0);
        ((this + 0x74) as *mut u32).write(0);
        ((this + 0x78) as *mut u32).write(0);
        ((this + 0x7C) as *mut u16).write(0);
        let probe = (this + MEMBER_OFF) as *const u32;
        let radius: f32 = lf_checker_rt::callee_cdecl!(RADIUS_CALLEE, f32, probe.read());
        ((this + RADIUS_SLOT) as *mut f32).write(radius);
        this
    }
});
