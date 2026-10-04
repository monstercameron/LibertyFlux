// original: 0x00c5ca30 CTaskSimpleArrestPed_ctor (proposed)

/// Constructor of the arrest-ped simple task.
///
/// Runs the base simple-task constructor (callee 1), writes the class virtual
/// table at `+0`, stores the argument at `+0x14` with a zeroed flag byte and
/// word after it, and sets the duration at `+0x20` to 0x124. Adds a reference
/// to the argument (callee 2) when it is non-null. Returns `this`.
///
/// Original: 0x00c5ca30 (thiscall: `this` in ecx, one stack word).
lf_checker_rt::export!(thiscall, rw_00c5ca30(this: u32, a0: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xecb32c;
        const REF: u32 = 0x14;
        const DURATION: u32 = 0x124;
        const BASE_CTOR: u32 = 1;
        const ADDREF: u32 = 2;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((this + REF) as *mut u32).write_unaligned(a0);
        ((this + 0x18) as *mut u8).write(0);
        ((this + 0x1c) as *mut u32).write_unaligned(0);
        ((this + 0x20) as *mut u32).write_unaligned(DURATION);
        if a0 != 0 {
            lf_checker_rt::callee_thiscall!(ADDREF, u32, a0, this + REF);
        }
        this
    }
});

