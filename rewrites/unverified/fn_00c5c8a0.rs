// original: 0x00c5c8a0 CTaskComplexDestroyCar_ctor (proposed)

/// Constructor of the destroy-car complex task.
///
/// Runs the base task constructor (callee 1), writes the class virtual table
/// at `+0`, stores the four arguments at `+0x18`, `+0x1c`, `+0x20`, `+0x24`,
/// and adds a reference to the first (callee 2) when it is non-null.
/// Returns `this`.
///
/// Original: 0x00c5c8a0 (thiscall: `this` in ecx, four stack words).
lf_checker_rt::export!(thiscall, rw_00c5c8a0(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xecb27c;
        const REF: u32 = 0x18;
        const BASE_CTOR: u32 = 1;
        const ADDREF: u32 = 2;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((this + REF) as *mut u32).write_unaligned(a0);
        ((this + 0x1c) as *mut u32).write_unaligned(a1);
        ((this + 0x20) as *mut u32).write_unaligned(a2);
        ((this + 0x24) as *mut u32).write_unaligned(a3);
        if a0 != 0 {
            lf_checker_rt::callee_thiscall!(ADDREF, u32, a0, this + REF);
        }
        this
    }
});

