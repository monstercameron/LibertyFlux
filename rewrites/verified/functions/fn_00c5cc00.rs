// original: 0x00c5cc00 CTaskComplexDestroyCar_dtor (proposed)

/// Destructor body of the destroy-car complex task.
///
/// Writes the class virtual table at `+0`, releases the held reference at
/// `+0x18` through callee 1 when it is non-null, then runs the base task
/// destructor (callee 2, reached by a tail jump in the original). Returns
/// nothing.
///
/// Original: 0x00c5cc00 (thiscall: `this` in ecx, no stack words).
lf_checker_rt::export!(thiscall, rw_00c5cc00(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xecb27c;
        const MEMBER: u32 = 0x18;
        const RELEASE: u32 = 1;
        const BASE_DTOR: u32 = 2;
        let held = ((this + MEMBER) as *const u32).read_unaligned();
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        if held != 0 {
            lf_checker_rt::callee_thiscall!(RELEASE, u32, held, this + MEMBER);
        }
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this);
        0
    }
});

