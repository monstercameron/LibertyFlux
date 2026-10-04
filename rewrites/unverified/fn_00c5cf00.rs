// original: 0x00c5cf00 CTaskSimpleThrowGrenadeFromVehicle::vf0

/// Deleting destructor of the throw-grenade-from-vehicle simple task.
///
/// Writes the class virtual table address at `+0`, destroys the embedded
/// member at `+0x14` (callee 1) and the base task (callee 2), then frees the
/// object through the global heap (callee 3) when the low bit of `flag` is
/// set. Returns `this`. `flag` is read as one byte.
///
/// Original: 0x00c5cf00 (thiscall: `this` in ecx, one stack word).
lf_checker_rt::export!(thiscall, rw_00c5cf00(this: u32, flag: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xecb48c;
        const MEMBER: u32 = 0x14;
        const MEMBER_DTOR: u32 = 1;
        const BASE_DTOR: u32 = 2;
        const DELETE_CALLEE: u32 = 3;
        const HEAP_SLOT: u32 = 0x167e2a0;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(MEMBER_DTOR, u32, this + MEMBER);
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this);
        if (flag & 1) != 0 {
            let heap = lf_checker_rt::global::<u32>(HEAP_SLOT).read();
            lf_checker_rt::callee_thiscall!(DELETE_CALLEE, u32, heap, this);
        }
        this
    }
});

