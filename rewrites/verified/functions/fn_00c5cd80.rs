// original: 0x00c5cd80 CTaskSimplePlayerBeArrested_dtor (proposed)

/// Destructor body of the player-be-arrested simple task.
///
/// Writes the class virtual table at `+0`, releases the held reference at
/// `+0x14` through callee 1 when it is non-null and clears the slot, then
/// runs the base task destructor (callee 2, reached by a tail jump in the
/// original). Returns nothing.
///
/// Original: 0x00c5cd80 (thiscall: `this` in ecx, no stack words).
lf_checker_rt::export!(thiscall, rw_00c5cd80(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xecb384;
        const MEMBER: u32 = 0x14;
        const RELEASE: u32 = 1;
        const BASE_DTOR: u32 = 2;
        let held = ((this + MEMBER) as *const u32).read_unaligned();
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        if held != 0 {
            lf_checker_rt::callee_thiscall!(RELEASE, u32, held, this + MEMBER);
            ((this + MEMBER) as *mut u32).write_unaligned(0);
        }
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this);
        0
    }
});

