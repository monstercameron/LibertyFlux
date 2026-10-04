// original: 0x00DA6860 CTaskComplexFleeAndDive::ctor (proposed)

/// Constructor of the flee-and-dive task: build the header, install the
/// virtual table, scatter the arguments, clear the timer block, then
/// stamp the timer when a deadline is given.
///
/// `near`/`far` are floats kept at `+0x34`/`+0x38`; `pos` points at three
/// words copied to `+0x20..+0x28`; `limit` is kept at `+0x30` (its zero
/// test selects the member-release call and the flag at `+0x4c`);
/// `deadline`, unless -1, stamps `+0x40` with the tick global, `+0x44`
/// with itself and `+0x48` with 1. Returns `this`.
/// Original: thiscall, five stack words, callee cleanup.
lf_checker_rt::export!(thiscall, rw_00da6860(this: u32, near: u32, pos: u32, far: u32, limit: u32, deadline: u32) -> u32 {
    unsafe {
        const HEADER_CTOR: u32 = 1;
        const MEMBER_RELEASE: u32 = 2;
        const VTABLE: u32 = 0x00EEFB1C;
        const TICK_SLOT: u32 = 0x011735B4;
        const NO_DEADLINE: u32 = 0xFFFF_FFFF;

        lf_checker_rt::callee_thiscall!(HEADER_CTOR, u32, this);
        (this as *mut u32).write(lf_checker_rt::relocated(VTABLE));
        for i in 0..3u32 {
            let w = ((pos + i * 4) as *const u32).read();
            ((this + 0x20 + i * 4) as *mut u32).write(w);
        }
        ((this + 0x30) as *mut u32).write(limit);
        ((this + 0x34) as *mut u32).write(near);
        ((this + 0x38) as *mut u32).write(far);
        ((this + 0x3C) as *mut u32).write(deadline);
        ((this + 0x40) as *mut u32).write(0);
        ((this + 0x44) as *mut u32).write(0);
        ((this + 0x48) as *mut u16).write(0);
        ((this + 0x4C) as *mut u8).write(u8::from(limit != 0));
        if limit != 0 {
            lf_checker_rt::callee_stdcall!(MEMBER_RELEASE, u32, this + 0x30);
        }
        if deadline != NO_DEADLINE {
            let tick = (lf_checker_rt::relocated(TICK_SLOT) as *const u32).read();
            ((this + 0x40) as *mut u32).write(tick);
            ((this + 0x44) as *mut u32).write(deadline);
            ((this + 0x48) as *mut u8).write(1);
        }
        this
    }
});
