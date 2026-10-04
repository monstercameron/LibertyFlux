// original: 0x00DA68F0 task_vt_eefbcc_ctor (proposed)

/// Constructor of the task class whose virtual table is 0x00EEFBCC: build
/// the header, scatter nine arguments into the object, install the table,
/// then hand the owner slot to the member-release callee when it is set.
///
/// `owner` is kept at `+0x14` and tested; `kind`'s low byte goes to
/// `+0x19`; `blend`/`bias`/`span` are floats kept at `+0x24`/`+0x28`/
/// `+0x34`; `a`/`b`/`c`/`d` are words kept at `+0x2c`/`+0x1c`/`+0x20`/
/// `+0x30`; `+0x18` and `+0x38` are cleared. Returns `this`.
/// Original: thiscall, nine stack words, callee cleanup.
lf_checker_rt::export!(thiscall, rw_00da68f0(this: u32, owner: u32, kind: u32, blend: u32, a: u32, b: u32, c: u32, bias: u32, d: u32, span: u32) -> u32 {
    unsafe {
        const HEADER_CTOR: u32 = 1;
        const MEMBER_RELEASE: u32 = 2;
        const VTABLE: u32 = 0x00EEFBCC;
        const OWNER_OFF: u32 = 0x14;

        lf_checker_rt::callee_thiscall!(HEADER_CTOR, u32, this);
        ((this + 0x19) as *mut u8).write((kind & 0xFF) as u8);
        ((this + 0x1C) as *mut u32).write(b);
        ((this + 0x20) as *mut u32).write(c);
        ((this + 0x24) as *mut u32).write(bias);
        ((this + 0x28) as *mut u32).write(blend);
        ((this + 0x2C) as *mut u32).write(a);
        ((this + 0x30) as *mut u32).write(d);
        ((this + 0x34) as *mut u32).write(span);
        (this as *mut u32).write(lf_checker_rt::relocated(VTABLE));
        ((this + OWNER_OFF) as *mut u32).write(owner);
        ((this + 0x18) as *mut u8).write(0);
        ((this + 0x38) as *mut u32).write(0);
        if owner != 0 {
            lf_checker_rt::callee_stdcall!(MEMBER_RELEASE, u32, this + OWNER_OFF);
        }
        this
    }
});
