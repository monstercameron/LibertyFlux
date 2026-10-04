// original: 0x00DA6800 CTaskComplexEscapeBlast::ctor (proposed)

/// Constructor of the escape-blast task: build the header, install the
/// virtual table, scatter the arguments into the object, then hand the
/// owner slot to the member-release callee when it is set.
///
/// `owner` is kept at `+0x14` and tested; `pos` points at three words
/// copied to `+0x20..+0x28`; `speed` and `range` are floats kept at
/// `+0x30` and `+0x34`; the low byte of `flags` is kept at `+0x38`.
/// Returns `this`. Original: thiscall, five stack words, callee cleanup.
lf_checker_rt::export!(thiscall, rw_00da6800(this: u32, owner: u32, pos: u32, speed: u32, flags: u32, range: u32) -> u32 {
    unsafe {
        const HEADER_CTOR: u32 = 1;
        const MEMBER_RELEASE: u32 = 2;
        const VTABLE: u32 = 0x00EEFC24;
        const OWNER_OFF: u32 = 0x14;
        const POS_OFF: u32 = 0x20;

        lf_checker_rt::callee_thiscall!(HEADER_CTOR, u32, this);
        ((this + OWNER_OFF) as *mut u32).write(owner);
        (this as *mut u32).write(lf_checker_rt::relocated(VTABLE));
        for i in 0..3u32 {
            let w = ((pos + i * 4) as *const u32).read();
            ((this + POS_OFF + i * 4) as *mut u32).write(w);
        }
        ((this + 0x30) as *mut u32).write(speed);
        ((this + 0x34) as *mut u32).write(range);
        ((this + 0x38) as *mut u8).write((flags & 0xFF) as u8);
        if owner != 0 {
            lf_checker_rt::callee_stdcall!(MEMBER_RELEASE, u32, this + OWNER_OFF);
        }
        this
    }
});
