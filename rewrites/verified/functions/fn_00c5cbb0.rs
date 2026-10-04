// original: 0x00c5cbb0 CTaskSimpleThrowGrenadeFromVehicle_ctor (proposed)

/// Constructor of the throw-grenade-from-vehicle simple task.
///
/// Runs the base simple-task constructor (callee 1) and the embedded member
/// constructor at `+0x14` (callee 2), writes the class virtual table at `+0`,
/// stores the arguments at `+0x2c` and `+0x30`, zeroes the words at `+0x20`,
/// `+0x28` and the flag byte at `+0x24` plus the word at `+0x34`, then
/// initialises the member with mode -1 (callee 3). Returns `this`.
///
/// Original: 0x00c5cbb0 (thiscall: `this` in ecx, two stack words).
lf_checker_rt::export!(thiscall, rw_00c5cbb0(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xecb48c;
        const MEMBER: u32 = 0x14;
        const MODE: u32 = 0xffff_ffff;
        const BASE_CTOR: u32 = 1;
        const MEMBER_CTOR: u32 = 2;
        const MEMBER_INIT: u32 = 3;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(MEMBER_CTOR, u32, this + MEMBER);
        ((this + 0x20) as *mut u32).write_unaligned(0);
        ((this + 0x24) as *mut u8).write(0);
        ((this + 0x28) as *mut u32).write_unaligned(0);
        ((this + 0x2c) as *mut u32).write_unaligned(a0);
        ((this + 0x30) as *mut u32).write_unaligned(a1);
        ((this + 0x34) as *mut u16).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(MEMBER_INIT, u32, this + MEMBER, MODE);
        this
    }
});

