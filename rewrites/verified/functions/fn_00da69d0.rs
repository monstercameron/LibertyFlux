// original: 0x00DA69D0 task_vt_eefac4_ctor (proposed)

/// Constructor of the task class whose virtual table is 0x00EEFAC4: build
/// the header, install the table, scatter seven arguments, set the mode
/// word to 3, clear the trailing state, then hand the owner slot to the
/// member-release callee when it is set.
///
/// `owner` is kept at `+0x14` and tested; `style`/`extra` low bytes go to
/// `+0x38`/`+0x39`; `x`/`y` are floats kept at `+0x34`/`+0x40`; `limit`
/// and `span` are words kept at `+0x30`/`+0x3c`. Returns `this`.
/// Original: thiscall, seven stack words, callee cleanup.
lf_checker_rt::export!(thiscall, rw_00da69d0(this: u32, owner: u32, style: u32, x: u32, limit: u32, span: u32, y: u32, extra: u32) -> u32 {
    unsafe {
        const HEADER_CTOR: u32 = 1;
        const MEMBER_RELEASE: u32 = 2;
        const VTABLE: u32 = 0x00EEFAC4;
        const OWNER_OFF: u32 = 0x14;
        const MODE: u32 = 3;

        lf_checker_rt::callee_thiscall!(HEADER_CTOR, u32, this);
        ((this + OWNER_OFF) as *mut u32).write(owner);
        ((this + 0x30) as *mut u32).write(limit);
        ((this + 0x34) as *mut u32).write(x);
        ((this + 0x38) as *mut u8).write((style & 0xFF) as u8);
        ((this + 0x39) as *mut u8).write((extra & 0xFF) as u8);
        ((this + 0x3C) as *mut u32).write(span);
        ((this + 0x40) as *mut u32).write(y);
        (this as *mut u32).write(lf_checker_rt::relocated(VTABLE));
        ((this + 0x44) as *mut u32).write(MODE);
        ((this + 0x48) as *mut u32).write(0);
        ((this + 0x4C) as *mut u32).write(0);
        ((this + 0x50) as *mut u16).write(0);
        if owner != 0 {
            lf_checker_rt::callee_stdcall!(MEMBER_RELEASE, u32, this + OWNER_OFF);
        }
        this
    }
});
