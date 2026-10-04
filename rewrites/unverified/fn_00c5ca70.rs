// original: 0x00c5ca70 CTaskSimpleBeHit_ctor (proposed)

/// Constructor of the be-hit simple task.
///
/// Runs the base simple-task constructor (callee 1), writes the class virtual
/// table at `+0`, stores the reference at `+0x14`, two arguments at
/// `+0x1c`/`+0x20`, the flag byte at `+0x19` (low byte of the fourth
/// argument), and zeroes the byte at `+0x18` and the word at `+0x24`. Adds a
/// reference to the first argument (callee 2) when non-null. Returns `this`.
///
/// Original: 0x00c5ca70 (thiscall: `this` in ecx, four stack words).
lf_checker_rt::export!(thiscall, rw_00c5ca70(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xecb224;
        const REF: u32 = 0x14;
        const BASE_CTOR: u32 = 1;
        const ADDREF: u32 = 2;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((this + REF) as *mut u32).write_unaligned(a0);
        ((this + 0x18) as *mut u8).write(0);
        ((this + 0x19) as *mut u8).write((a3 & 0xff) as u8);
        ((this + 0x1c) as *mut u32).write_unaligned(a1);
        ((this + 0x20) as *mut u32).write_unaligned(a2);
        ((this + 0x24) as *mut u32).write_unaligned(0);
        if a0 != 0 {
            lf_checker_rt::callee_thiscall!(ADDREF, u32, a0, this + REF);
        }
        this
    }
});

