// original: 0x00c5cac0 CTaskSimpleNewGangDriveBy_ctor (proposed)

/// Constructor of the new-gang-driveby simple task.
///
/// Runs the base simple-task constructor (callee 1) and the embedded member
/// constructor at `+0x14` (callee 2), writes the class virtual table at `+0`,
/// stores the reference at `+0x20`, the float bits at `+0x40`, the mode at
/// `+0x4c`, a 0x100 kind word at `+0x48` with flag byte 1 at `+0x4a`, and
/// zeroes the surrounding words. Adds a reference to the first argument
/// (callee 3) when non-null. When the second argument is non-null it is read
/// as four words copied to `+0x30`..`+0x3c`. Returns `this`.
///
/// Original: 0x00c5cac0 (thiscall: `this` in ecx, four stack words).
lf_checker_rt::export!(thiscall, rw_00c5cac0(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xecb434;
        const MEMBER: u32 = 0x14;
        const REF: u32 = 0x20;
        const VEC: u32 = 0x30;
        const BASE_CTOR: u32 = 1;
        const MEMBER_CTOR: u32 = 2;
        const ADDREF: u32 = 3;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(MEMBER_CTOR, u32, this + MEMBER);
        ((this + REF) as *mut u32).write_unaligned(a0);
        ((this + VEC) as *mut u32).write_unaligned(0);
        ((this + 0x34) as *mut u32).write_unaligned(0);
        ((this + 0x38) as *mut u32).write_unaligned(0);
        ((this + 0x40) as *mut u32).write_unaligned(a2);
        ((this + 0x44) as *mut u32).write_unaligned(0);
        ((this + 0x48) as *mut u16).write_unaligned(0x100);
        ((this + 0x4a) as *mut u8).write(1);
        ((this + 0x4c) as *mut u32).write_unaligned(a3);
        ((this + 0x50) as *mut u32).write_unaligned(0);
        ((this + 0x54) as *mut u32).write_unaligned(0);
        ((this + 0x58) as *mut u32).write_unaligned(0);
        ((this + 0x5c) as *mut u32).write_unaligned(0);
        if a0 != 0 {
            lf_checker_rt::callee_thiscall!(ADDREF, u32, a0, this + REF);
        }
        if a1 != 0 {
            for k in 0..4u32 {
                let w = ((a1 + k * 4) as *const u32).read_unaligned();
                ((this + VEC + k * 4) as *mut u32).write_unaligned(w);
            }
        }
        this
    }
});

