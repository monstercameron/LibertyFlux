// original: 0x00c5c9a0 CTaskComplexGangDriveby_ctor (proposed)

/// Constructor of the gang-driveby complex task.
///
/// Runs the base task constructor (callee 1), writes the class virtual table
/// at `+0`, stores the reference argument at `+0x14`, the float bits at
/// `+0x34`, two arguments at `+0x38`/`+0x3c` and the flag byte at `+0x40`,
/// zeroing the words at `+0x20`/`+0x24`/`+0x28` and the flag at `+0x30`. Adds
/// a reference to the first argument (callee 2) when non-null. When the
/// second argument is non-null it is read as four words copied to
/// `+0x20`..`+0x2c` and the flag at `+0x30` becomes 1. Returns `this`.
///
/// Original: 0x00c5c9a0 (thiscall: `this` in ecx, six stack words).
lf_checker_rt::export!(thiscall, rw_00c5c9a0(
    this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32,
) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xecb3dc;
        const REF: u32 = 0x14;
        const VEC: u32 = 0x20;
        const HAS_VEC: u32 = 0x30;
        const BASE_CTOR: u32 = 1;
        const ADDREF: u32 = 2;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((this + REF) as *mut u32).write_unaligned(a0);
        ((this + VEC) as *mut u32).write_unaligned(0);
        ((this + 0x24) as *mut u32).write_unaligned(0);
        ((this + 0x28) as *mut u32).write_unaligned(0);
        ((this + HAS_VEC) as *mut u8).write(0);
        ((this + 0x34) as *mut u32).write_unaligned(a2);
        ((this + 0x38) as *mut u32).write_unaligned(a3);
        ((this + 0x3c) as *mut u32).write_unaligned(a4);
        ((this + 0x40) as *mut u8).write((a5 & 0xff) as u8);
        if a0 != 0 {
            lf_checker_rt::callee_thiscall!(ADDREF, u32, a0, this + REF);
        }
        if a1 != 0 {
            ((this + HAS_VEC) as *mut u8).write(1);
            for k in 0..4u32 {
                let w = ((a1 + k * 4) as *const u32).read_unaligned();
                ((this + VEC + k * 4) as *mut u32).write_unaligned(w);
            }
        }
        this
    }
});

