// original: 0x00c5cb70 CTaskSimplePlayerBeArrested_ctor (proposed)

/// Constructor of the player-be-arrested simple task.
///
/// Runs the base simple-task constructor (callee 1), writes the class virtual
/// table at `+0`, stores the argument at `+0x14`, zeroes the word at `+0x18`
/// and the flag byte at `+0x1c`. Adds a reference to the argument (callee 2)
/// when it is non-null. Returns `this`.
///
/// Original: 0x00c5cb70 (thiscall: `this` in ecx, one stack word).
lf_checker_rt::export!(thiscall, rw_00c5cb70(this: u32, a0: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xecb384;
        const REF: u32 = 0x14;
        const BASE_CTOR: u32 = 1;
        const ADDREF: u32 = 2;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((this + REF) as *mut u32).write_unaligned(a0);
        ((this + 0x18) as *mut u32).write_unaligned(0);
        ((this + 0x1c) as *mut u8).write(0);
        if a0 != 0 {
            lf_checker_rt::callee_thiscall!(ADDREF, u32, a0, this + REF);
        }
        this
    }
});

