// original: 0x00DA6970 task_vt_eefb74_ctor (proposed)

/// Constructor of the task class whose virtual table is 0x00EEFB74: run
/// the base constructor with a reordered argument list, keep the two
/// middle arguments at `+0x60`/`+0x64`, install the table and clear the
/// trailing state.
///
/// The base call receives (`a1`, `a2`, `a3`, `a4`, `a7`, `a8`, 0): the
/// first four arguments in order, then the last two, then a zero seventh
/// word. Returns `this`. Original: thiscall, eight stack words.
lf_checker_rt::export!(thiscall, rw_00da6970(this: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32) -> u32 {
    unsafe {
        const BASE_CTOR: u32 = 1;
        const VTABLE: u32 = 0x00EEFB74;

        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this, a1, a2, a3, a4, a7, a8, 0);
        ((this + 0x60) as *mut u32).write(a5);
        ((this + 0x64) as *mut u32).write(a6);
        (this as *mut u32).write(lf_checker_rt::relocated(VTABLE));
        ((this + 0x68) as *mut u32).write(0);
        ((this + 0x6C) as *mut u32).write(0);
        ((this + 0x70) as *mut u16).write(0);
        this
    }
});
